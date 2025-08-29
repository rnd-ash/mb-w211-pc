#![no_std]
#![no_main]

mod bsp;
mod can;
mod io;
mod paddle_emu;

use atsamd_hal::{adc, rtc::rtic::rtc_clock, sercom::Sercom1};

use panic_rtt_target as _; // For panics

atsamd_hal::rtc_monotonic!(Mono, rtc_clock::Clock1k);

atsamd_hal::bind_multiple_interrupts!(struct DmacIrqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

atsamd_hal::bind_multiple_interrupts!(struct Sercom1Irqs {
    SERCOM1: [SERCOM1_0, SERCOM1_1, SERCOM1_2, SERCOM1_OTHER] => atsamd_hal::sercom::uart::InterruptHandler<Sercom1>;
});

atsamd_hal::bind_multiple_interrupts!(struct Adc0Irqs {
    ADC0: [ADC0_RESRDY, ADC0_OTHER] => atsamd_hal::adc::InterruptHandler<adc::Adc0>;
});

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS_0])]
mod app {

    use super::*;
    use crate::{
        bsp::{uart, AmpCurrentSense, AmpMosfet, AmpMute, AmpStandby, PcCurrentSense, PcMosfet, UartPads, VSense},
        can::{uart_read_frame, Can0RxFifo0, Can0Tx, Can1RxFifo0, Can1Tx},
        io::BoardIO,
        paddle_emu::PaddleEmulator,
    };
    use atsamd_hal::{adc::{self, Adc}, clock::v2::{ahb::Ahb, osculp32k::{OscUlp1k, OscUlp32k}, rtcosc::RtcOsc}, dmac::{Ch0, Ch1}, pac::Supc};
    use atsamd_hal::{
        can::Dependencies,
        clock::v2::{
            clock_system_at_reset,
            dpll::Dpll,
            gclk::{Gclk, Gclk2Id, GclkDiv16, GclkDiv8},
            pclk::Pclk,
            types::{Can0, Can1},
            Source,
        },
        dmac::{DmaController, PriorityLevel},
        pac::Peripherals,
        prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin,
        sercom::uart::{self, UartFutureRxDuplexDma, UartFutureTxDuplexDma},
    };
    use bsp::{CanCShutdown, OnboardLED};
    use can::{Can0Aux, CanNet, Capacities, SerialCanFrame};
    use cortex_m::asm::{nop, wfi};
    use defmt_rtt as _;
    use fugit::ExtU32;
    use fugit::*;
    use mcan::{
        bus::DynAux,
        embedded_can::{self, Id},
        filter::Filter,
        interrupt::{state::EnabledLine0, OwnedInterruptSet},
        message::{
            tx::{AnyMessage, Message},
            Raw,
        },
        messageram::SharedMemory,
        tx_buffers::DynTx,
    };
    use rtic_monotonics::Monotonic;
    use rtic_sync::channel::{Receiver, Sender};
    use w211_can::canb::{EZS_A1_CAN_ID, MRM_A1_CAN_ID, MRM_A2_CAN_ID};

    #[local]
    struct Local {
        onboard_led: OnboardLED,
        uart_rx: UartFutureRxDuplexDma<uart::Config<UartPads>, Ch0>,
        uart_tx: UartFutureTxDuplexDma<uart::Config<UartPads>, Ch1>,
        canb_aux: Can0Aux<Gclk2Id>,
        canb_rx: Can0RxFifo0,
        canc_rx: Can1RxFifo0,
        canb_tx: Can0Tx,
        canc_tx: Can1Tx,
        canb_interrupts: OwnedInterruptSet<Can0, EnabledLine0>,
        canc_interrupts: OwnedInterruptSet<Can1, EnabledLine0>,
        canb_serial_tx: Sender<'static, SerialCanFrame, 100>,
        canc_serial_tx: Sender<'static, SerialCanFrame, 100>,
        can_to_serial_recv: Receiver<'static, SerialCanFrame, 100>,
        board_io: io::BoardIO,
        paddle_emu: paddle_emu::PaddleEmulator,

        tx_ezs_a1: Sender<'static, [u8; 8], 10>,
        tx_mrm_data: Sender<'static, (u16, [u8; 8]), 10>,

        tx_canb_sender: Sender<'static, SerialCanFrame, 10>,
        tx_canc_sender: Sender<'static, SerialCanFrame, 10>,
        tx_canb_recv: Receiver<'static, SerialCanFrame, 10>,
        tx_canc_recv: Receiver<'static, SerialCanFrame, 10>,
    }

    #[shared]
    struct Shared {
        key_active: bool,
    }

    #[init(local=[
        #[link_section = ".can"]
        can_memory0: SharedMemory<Capacities> = SharedMemory::new()
        #[link_section = ".can"]
        can_memory1: SharedMemory<Capacities> = SharedMemory::new()
    ])]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        // Initialization (Called on startup)
        let pins = bsp::Pins::new(cx.device.port);
        // CPU is at default 48Mhz here
        let (mut buses, clocks, tokens) = clock_system_at_reset(
            cx.device.oscctrl,
            cx.device.osc32kctrl,
            cx.device.gclk,
            cx.device.mclk,
            &mut cx.device.nvmctrl,
        );
        let (_, _, _, mut mclk) = unsafe { clocks.pac.steal() };

        // Build the following clock tree
        //
        // DFLL(48Mhz)
        // └── GCLK1 (2Mhz)
        //     ├── DPLL0(100Mhz)
        //     │   └── GCLK0(100Mhz)
        //     │       ├── F_CPU
        //     │       └── SERCOM1
        //     └── DPLL1(160Mhz)
        //         └── GCLK2(80Mhz)
        //             ├── CAN0
        //             ├── CAN1
        //             └── ADC0

        // GCLK 1 is formed by taking DFLL48 and dividing by 24 to get 2Mhz
        let (gclk1, dfll) = Gclk::from_source(tokens.gclks.gclk1, clocks.dfll);
        let gclk1 = gclk1.div(GclkDiv16::Div(24)).enable();
        // Power both DPLL0 and DPLL1 from GCLK1
        let (pclk_dpll0, gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);
        let (pclk_dpll1, _gclk1) = Pclk::enable(tokens.pclks.dpll1, gclk1);
        // DPLL0 multipler is 50 (2mhz x 50) = 100Mhz
        let dpll0 = Dpll::from_pclk(tokens.dpll0, pclk_dpll0)
            .loop_div(50, 0)
            .enable();
        // DPLL0 multipler is 80 (2mhz x 80) = 160Mhz
        let dpll1 = Dpll::from_pclk(tokens.dpll1, pclk_dpll1)
            .loop_div(80, 0)
            .enable();
        // Swap GCLK0 from DFLL to DPLL0 so it runs at 100Mhz
        //let (gclk0, _dfll, _dpll0) = clocks.gclk0.swap_sources(dfll, dpll0);
        // Start GCLK2 off DPLL1 with a divider of 2 (160Mhz/2) = 80Mhz
        let (gclk2_uninit, _dpll1) = Gclk::from_source(tokens.gclks.gclk2, dpll1);
        let gclk2 = gclk2_uninit.div(GclkDiv8::Div(2)).enable();

        // Peripheral clock enabling
        let (pclk_sercom1, gclk2) = Pclk::enable(tokens.pclks.sercom1, gclk2);
        let (pclk_canb, gclk2) = Pclk::enable(tokens.pclks.can0, gclk2);
        let (pclk_canc, gclk2) = Pclk::enable(tokens.pclks.can1, gclk2);
        let (pclk_adc0, gclk2) = Pclk::enable(tokens.pclks.adc0, gclk2);

        // Enable RTC and start time driver for RTIC using that
        let (osculp1k, _) = OscUlp1k::enable(tokens.osculp32k.osculp1k, clocks.osculp32k_base);
        let _ = RtcOsc::enable(tokens.rtcosc, osculp1k);
        Mono::start(cx.device.rtc); // Start time driver now that clocks are ready
        cx.core.SCB.set_sleepdeep();

        // -- CAN Configuration and setup
        let (deps_canb, gclk2) = Dependencies::new(
            gclk2,
            pclk_canb,
            clocks.ahbs.can0,
            pin_alias!(pins.pa23).into_mode(),
            pin_alias!(pins.pa22).into_mode(),
            cx.device.can0,
        );
        let (deps_canc, gclk2) = Dependencies::new(
            gclk2,
            pclk_canc,
            clocks.ahbs.can1,
            pin_alias!(pins.pb13).into_mode(),
            pin_alias!(pins.pb12).into_mode(),
            cx.device.can1,
        );

        // CANB is set to 83333 below via a hack
        let mut can_b =
            mcan::bus::CanConfigurable::new(500_000u32.Hz(), deps_canb, cx.local.can_memory0)
                .unwrap();
        let mut can_c =
            mcan::bus::CanConfigurable::new(500_000u32.Hz(), deps_canc, cx.local.can_memory1)
                .unwrap();

        let (tx_serial_can, rx_serial_can) = rtic_sync::make_channel!(SerialCanFrame, 100);

        let (tx_ezs_a1, rx_ezs_a1) = rtic_sync::make_channel!([u8; 8], 10);
        let (tx_mrm, rx_mrm) = rtic_sync::make_channel!((u16, [u8; 8]), 10);
        let (tx_cane, rx_cane) = rtic_sync::make_channel!(SerialCanFrame, 10);

        // Channels to write back to CAN
        let (tx_canb_sender, tx_canb_recv) = rtic_sync::make_channel!(SerialCanFrame, 10);
        let (tx_canc_sender, tx_canc_recv) = rtic_sync::make_channel!(SerialCanFrame, 10);

        can_b.config().loopback = false;
        can_b.config().mode = mcan::config::Mode::Classic;
        can_b.config().timestamp = mcan::config::Timestamp::default();
        // Manual bit timing for 83_333

        can_c.config().loopback = false;
        can_c.config().mode = mcan::config::Mode::Classic;
        can_c.config().timestamp = mcan::config::Timestamp::default();

        let isrs_canb = can_b
            .interrupts()
            .split(
                [
                    mcan::interrupt::Interrupt::RxFifo0NewMessage,
                    mcan::interrupt::Interrupt::RxFifo1NewMessage,
                    mcan::interrupt::Interrupt::ErrorPassive,
                    mcan::interrupt::Interrupt::BusOff,
                    mcan::interrupt::Interrupt::WarningStatusChanged,
                    mcan::interrupt::Interrupt::ProtocolErrorData,
                ]
                .into_iter()
                .collect(),
            )
            .unwrap();
        let line_interrupts_canb = can_b.interrupt_configuration().enable_line_0(isrs_canb);
        can_b
            .filters_standard()
            .push(Filter::Classic {
                action: mcan::filter::Action::StoreFifo0,
                filter: embedded_can::StandardId::ZERO,
                mask: embedded_can::StandardId::ZERO,
            })
            .unwrap_or_else(|_| panic!("Could not set CAN0 filter"));

        let isrs_canc = can_c
            .interrupts()
            .split(
                [
                    mcan::interrupt::Interrupt::RxFifo0NewMessage,
                    mcan::interrupt::Interrupt::RxFifo1NewMessage,
                    mcan::interrupt::Interrupt::BusOff,
                    mcan::interrupt::Interrupt::ErrorPassive,
                ]
                .into_iter()
                .collect(),
            )
            .unwrap();
        let line_interrupts_canc = can_c.interrupt_configuration().enable_line_0(isrs_canc);
        can_c
            .filters_standard()
            .push(Filter::Classic {
                action: mcan::filter::Action::StoreFifo0,
                filter: embedded_can::StandardId::ZERO,
                mask: embedded_can::StandardId::ZERO,
            })
            .unwrap_or_else(|_| panic!("Could not set CAN1 filter"));
        let can_c = can_c.finalize_initialized().unwrap();
        let can_b = can_b.finalize_initialized().unwrap();
        can_c.aux.operational_mode(); // Start up CAN C (CAN B is started below)\

        // Bitrate hack for 83333bps Baud on CAN B
        // See https://github.com/GrepitAB/mcan/issues/58
        unsafe {
            let div_round = |x: u32, y: u32| -> u32 { ((x) + (y) / 2) / (y) };

            let div_round_up = |x: u32, y: u32| -> u32 { ((x) + (y) - 1) / (y) };
            let f_can = gclk2.freq().to_Hz(); // 80000000
            let clocks_per_bit = div_round(f_can, 83333); // 960
            let clocks_to_sample = div_round_up(clocks_per_bit * 7, 8); // 840
            let clocks_after_sample = clocks_per_bit - clocks_to_sample; // 120
            let divisor = core::cmp::max(
                div_round_up(clocks_to_sample, 256), // max(4, 1) -> 4
                div_round_up(clocks_after_sample, 128),
            );
            // Hack to get 83333 working (80Mhz clock source)
            let canb_reg = Peripherals::steal().can0;
            // Put the CAN controller back into init mode

            canb_reg.cccr().write(|w| w.init().set_bit());
            while canb_reg.cccr().read().init().bit_is_clear() {
                core::hint::spin_loop();
            }
            canb_reg.cccr().write(|w| w.cce().set_bit());

            canb_reg.nbtp().reset();
            canb_reg.nbtp().modify(|_, w| {
                w.ntseg1()
                    .bits(div_round(clocks_to_sample, divisor) as u8 - 2)
                    .ntseg2()
                    .bits(div_round(clocks_after_sample, divisor) as u8 - 1)
                    .nbrp()
                    .bits(divisor as u16 - 1)
                    .nsjw()
                    .bits(div_round(clocks_after_sample, divisor * 4) as u8)
            });

            canb_reg.cccr().write(|w| w.cce().clear_bit());
            nop();
            nop();
            canb_reg.cccr().write(|w| w.init().clear_bit());
            while canb_reg.cccr().read().init().bit_is_set() {
                core::hint::spin_loop();
            }
        }

        // -- DMA Setup - Use DMA for Tx and Rx of the UART peripheral
        let dmac = DmaController::init(cx.device.dmac, &mut cx.device.pm);
        let dma_channels = dmac.into_future(DmacIrqs).split();
        let dma_uart_rx = dma_channels.0.init(PriorityLevel::Lvl0);
        let dma_uart_tx = dma_channels.1.init(PriorityLevel::Lvl0);

        // -- UART SETUP (Talk to PC)
        let uart = uart(
            pclk_sercom1,
            921600u32.Hz(),
            cx.device.sercom1,
            &mut mclk,
            pins.pa17,
            pins.pa16,
        );
        let uart_future = uart
            .into_future(Sercom1Irqs)
            .with_rx_dma_channel(dma_uart_rx)
            .with_tx_dma_channel(dma_uart_tx);
        let (uart_rx, uart_tx) = uart_future.split();

        // ADC0 setup (For Vsense, Current_PC, Current_Amp)
        let adc_config = adc::Config::new();
        let apb_adc = buses.apb.enable(tokens.apbs.adc0);
        let adc0: Adc<adc::Adc0> = Adc::new(cx.device.adc0, adc_config, apb_adc, &pclk_adc0).unwrap();
        let adc0_fut = adc0.into_future(Adc0Irqs);

        // IO Pins
        let onboard_led: OnboardLED = pin_alias!(pins.onboard_led).into();
        let pc_mosfet: PcMosfet = pin_alias!(pins.pc_mosfet).into();
        let amp_mosfet: AmpMosfet = pin_alias!(pins.amp_mosfet).into();
        let amp_mute: AmpMute = pin_alias!(pins.amp_mute).into();
        let amp_standby: AmpStandby = pin_alias!(pins.amp_standby).into();
        let can_c_shutdown: CanCShutdown = pin_alias!(pins.can_c_shutdown).into();
        // ADC Pins
        let v_sense: VSense = pin_alias!(pins.v_sense).into();
        let curr_sense_amp: AmpCurrentSense = pin_alias!(pins.amp_c_sense).into();
        let curr_sense_pc: PcCurrentSense = pin_alias!(pins.pc_c_sense).into();

        let board_io = BoardIO::new(
            adc0_fut,
            amp_mosfet,
            pc_mosfet,
            amp_mute,
            amp_standby,
            can_c_shutdown,
            v_sense,
            curr_sense_pc,
            curr_sense_amp,
            rx_ezs_a1,
            rx_cane,
            tx_serial_can.clone(),
            Mono::now(),
        );
        let paddle_emu = PaddleEmulator::new(rx_mrm);
        serial_tx_handler::spawn().unwrap();
        serial_rx_handler::spawn().unwrap();
        io_controller::spawn(tx_canc_sender.clone(), cx.device.supc).unwrap();
        can_b_tx::spawn().unwrap();
        can_c_tx::spawn().unwrap();
        (
            Shared { key_active: true },
            Local {
                board_io,
                onboard_led,
                tx_ezs_a1,
                tx_mrm_data: tx_mrm,
                uart_tx: uart_tx,
                uart_rx: uart_rx,
                canb_aux: can_b.aux,
                canb_rx: can_b.rx_fifo_0,
                canc_rx: can_c.rx_fifo_0,
                canb_tx: can_b.tx,
                canc_tx: can_c.tx,
                canb_serial_tx: tx_serial_can.clone(),
                canc_serial_tx: tx_serial_can,
                can_to_serial_recv: rx_serial_can,
                canb_interrupts: line_interrupts_canb,
                canc_interrupts: line_interrupts_canc,

                tx_canb_recv,
                tx_canc_recv,

                tx_canb_sender,
                tx_canc_sender,
                paddle_emu,
            },
        )
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            wfi();
        }
    }

    #[task(priority=1, local=[board_io, paddle_emu], shared=[key_active])]
    async fn io_controller(
        mut cx: io_controller::Context,
        mut tx_c: Sender<'static, SerialCanFrame, 10>,
        supc: Supc
    ) {
        supc.vref().write(|w| {
            w.ondemand().set_bit();
            w.tsen().set_bit()
        });
        loop {
            let key_state = cx.local.board_io.update(&supc).await;
            let tx_canc_mrm = cx.local.paddle_emu.generate_mrm_tx_frame();
            cx.shared.key_active.lock(|r| *r = key_state);
            if key_state {
                let f = SerialCanFrame::new(CanNet::C, 0x232, &tx_canc_mrm);
                let _ = tx_c.try_send(f);
            }
            if cx.local.board_io.is_shutdown {
                wfi();
            } else {
                Mono::delay(20u64.millis()).await;
            }
        }
    }

    #[task(priority=1, local=[uart_tx, can_to_serial_recv], shared=[key_active])]
    async fn serial_tx_handler(mut cx: serial_tx_handler::Context) {
        let serial_tx_handler::LocalResources {
            uart_tx,
            can_to_serial_recv,
            ..
        } = cx.local;
        let mut buf = [0u8; 16];
        // Never changes so we can just place it here
        loop {
            if let Ok(frame) = can_to_serial_recv.recv().await {
                if cx.shared.key_active.lock(|r| *r) {
                    // Only send to UART if PC is alive
                    frame.to_bytes(&mut buf);
                    let _ = uart_tx.write(&buf).await;
                }
            }
        }
    }

    #[task(priority=1, local=[onboard_led, tx_canb_sender, tx_canc_sender, uart_rx], shared=[key_active])]
    async fn serial_rx_handler(mut cx: serial_rx_handler::Context) {
        let serial_rx_handler::LocalResources {
            onboard_led,
            mut uart_rx,
            tx_canb_sender,
            tx_canc_sender,
            ..
        } = cx.local;
        defmt::info!("UART Rx started");
        loop {
            let _ = onboard_led.set_high();
            match uart_read_frame(&mut uart_rx).await {
                Some(frame) => {
                    let _ = onboard_led.set_low();
                    let allowed_to_send = cx.shared.key_active.lock(|r| *r);
                    match frame.net {
                        1 => {
                            // B
                            if allowed_to_send {
                                let _ = tx_canb_sender.try_send(frame);
                            }
                        }
                        2 => {
                            // C
                            if allowed_to_send {
                                let _ = tx_canc_sender.try_send(frame);
                            }
                        }
                        3 => { // E (Internal to command controller)
                        }
                        _ => {
                            defmt::error!("Invalid CAN Net byte {}", frame.net)
                        }
                    }
                    let _ = onboard_led.set_high();
                }
                None => {
                    // TODO - Handle error here
                }
            }
        }
    }

    // Task to write out to CAN B
    #[task(priority=1, local=[tx_canb_recv, canb_tx])]
    async fn can_b_tx(cx: can_b_tx::Context) {
        loop {
            if let Ok(f) = cx.local.tx_canb_recv.recv().await {
                if let Some(builder) = f.to_can_msg() {
                    let _ = cx
                        .local
                        .canb_tx
                        .transmit_queued(Message::new(builder).unwrap());
                }
            }
        }
    }

    // Task to write out to CAN C
    #[task(priority=1, local=[tx_canc_recv, canc_tx])]
    async fn can_c_tx(cx: can_c_tx::Context) {
        loop {
            if let Ok(f) = cx.local.tx_canc_recv.recv().await {
                if let Some(builder) = f.to_can_msg() {
                    let _ = cx
                        .local
                        .canc_tx
                        .transmit_queued(Message::new(builder).unwrap());
                }
            }
        }
    }

    #[task(priority=2, binds=CAN0, local=[canb_interrupts, canb_aux, canb_rx, canb_serial_tx, tx_ezs_a1, tx_mrm_data])]
    fn can_b_isr(mut cx: can_b_isr::Context) {
        // Called on event of CAN B
        for evt in cx.local.canb_interrupts.iter_flagged() {
            match evt {
                mcan::interrupt::Interrupt::RxFifo0NewMessage => {
                    for msg in &mut cx.local.canb_rx {
                        if let Id::Standard(std_id) = msg.id() {
                            let mut d = [0u8; 8];
                            d[..msg.dlc() as usize].copy_from_slice(&msg.data());
                            let id = std_id.as_raw();
                            // Event dispatch for MRM and EZS frames on CAN B
                            match id {
                                EZS_A1_CAN_ID => {
                                    let _ = cx.local.tx_ezs_a1.try_send(d);
                                }
                                MRM_A1_CAN_ID | MRM_A2_CAN_ID => {
                                    let _ = cx.local.tx_mrm_data.try_send((id, d));
                                }
                                _ => {}
                            }
                            // Write all frames to UART
                            let _ = cx.local.canb_serial_tx.try_send(SerialCanFrame {
                                net: CanNet::B as u8,
                                id: std_id.as_raw(),
                                dlc: msg.dlc(),
                                data: d,
                            });
                        }
                    }
                }
                mcan::interrupt::Interrupt::BusOff => {
                    // Try to rejoin
                    cx.local.canb_aux.operational_mode();
                }
                _ => {
                    defmt::debug!("CANB unhandled event: {:?}", defmt::Debug2Format(&evt));
                }
            }
        }
    }

    #[task(priority=2, binds=CAN1, local=[canc_interrupts, canc_rx, canc_serial_tx])]
    fn can_c_isr(mut cx: can_c_isr::Context) {
        // Called on event of CAN C
        for evt in cx.local.canc_interrupts.iter_flagged() {
            //defmt::debug!("CAN1: {:?}", defmt::Debug2Format(&evt));
            match evt {
                mcan::interrupt::Interrupt::RxFifo0NewMessage => {
                    for msg in &mut cx.local.canc_rx {
                        if let Id::Standard(std_id) = msg.id() {
                            let mut d = [0u8; 8];
                            d[..msg.dlc() as usize].copy_from_slice(&msg.data());
                            let _ = cx.local.canc_serial_tx.try_send(SerialCanFrame {
                                net: CanNet::C as u8,
                                id: std_id.as_raw(),
                                dlc: msg.dlc(),
                                data: d,
                            });
                        }
                    }
                }
                _ => {
                    defmt::debug!("CANC unhandled event: {:?}", defmt::Debug2Format(&evt));
                }
            }
        }
    }
}
