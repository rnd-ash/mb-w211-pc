#![no_std]
#![no_main]

mod bsp;
mod can;
mod io;
mod paddle_emu;
mod usb;

use core::{panic::PanicInfo, sync::atomic::AtomicBool};

use atsamd_hal::{adc, rtc::rtic::rtc_clock, sercom::Sercom1};

use defmt_rtt as _;

atsamd_hal::rtc_monotonic!(Mono, rtc_clock::Clock32k);

atsamd_hal::bind_multiple_interrupts!(pub struct DmacIrqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

atsamd_hal::bind_multiple_interrupts!(pub struct Sercom1Irqs {
    SERCOM1: [SERCOM1_0, SERCOM1_1, SERCOM1_2, SERCOM1_OTHER] => atsamd_hal::sercom::uart::InterruptHandler<Sercom1>;
});

atsamd_hal::bind_multiple_interrupts!(pub struct Adc0Irqs {
    ADC0: [ADC0_RESRDY, ADC0_OTHER] => atsamd_hal::adc::InterruptHandler<adc::Adc0>;
});

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    defmt::error!("{}", info);
    loop{}
}

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS_0])]
mod app {

    use core::sync::atomic::Ordering;

    use super::*;
    use crate::{
        bsp::{AmpCurrentSense, AmpMosfet, AmpMute, AmpStandby, PcCurrentSense, PcMosfet, UartPads, VSense, uart},
        can::{Can0RxFifo0, Can0Tx, Can1RxFifo0, Can1Tx, SERIAL_FRAME_LEN, uart_read_frame},
        io::BoardIO,
        paddle_emu::PaddleEmulator, usb::UsbIsrInfo,
    };
    use atsamd_hal::{adc::{self, AdcBuilder}, clock::v2::{gclk::Gclk3Id, osculp32k::OscUlp32k, rtcosc::RtcOsc}, dmac::{Ch0, Ch1}, pac::Supc, rtic_time::Monotonic, usb::{UsbBus, usb_device::{bus::UsbBusAllocator, device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid}}}};
    use atsamd_hal::{
        can::Dependencies,
        clock::v2::{
            clock_system_at_reset,
            dpll::Dpll,
            gclk::{Gclk, GclkDiv16, GclkDiv8},
            pclk::Pclk,
            types::{Can0, Can1},
        },
        dmac::{DmaController, PriorityLevel},
        prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin,
    };
    use bsp::{CanCShutdown, OnboardLED};
    use can::{Can0Aux, CanNet, Capacities, SerialCanFrame};
    use defmt_rtt as _;
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
    use rtic_sync::{channel::{Receiver, Sender}, make_channel};
    use w211_can::canb::{EZS_A1_CAN_ID, MRM_A1_CAN_ID, MRM_A2_CAN_ID};

    #[local]
    struct Local {
        onboard_led: OnboardLED,
        canb_aux: Can0Aux<Gclk3Id>,
        canb_rx: Can0RxFifo0,
        canc_rx: Can1RxFifo0,
        canb_interrupts: OwnedInterruptSet<Can0, EnabledLine0>,
        canc_interrupts: OwnedInterruptSet<Can1, EnabledLine0>,
        canb_serial_tx: Sender<'static, SerialCanFrame, 200>,
        canc_serial_tx: Sender<'static, SerialCanFrame, 200>,
        tx_cane: Sender<'static, SerialCanFrame, 10>,
        can_to_serial_recv: Receiver<'static, SerialCanFrame, 200>,
        serial_to_can_recv: Receiver<'static, SerialCanFrame, 200>,
        board_io: io::BoardIO,
        paddle_emu: paddle_emu::PaddleEmulator,

        tx_ezs_a1: Sender<'static, [u8; 8], 10>,
        tx_mrm_data: Sender<'static, (u16, [u8; 8]), 10>,
    }

    #[shared]
    struct Shared {
        canb_tx: Can0Tx,
        canc_tx: Can1Tx,
        usb_isr_info: UsbIsrInfo<'static>,
    }

    #[init(local=[
        #[link_section = ".can"]
        can_memory0: SharedMemory<Capacities> = SharedMemory::new()
        #[link_section = ".can"]
        can_memory1: SharedMemory<Capacities> = SharedMemory::new(),
        usb_bus_alloc: Option<UsbBusAllocator<UsbBus>> = None,
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
        defmt::info!("RTIC Init");
        // Build the following clock tree
        //
        // DFLL(48Mhz)
        // └── GCLK1 (2Mhz)
        //     ├── DPLL0(100Mhz)
        //     │   └── GCLK0(100Mhz)
        //     │       ├── F_CPU
        //     └── DPLL1(160Mhz)
        //         ├── GCLK2(80Mhz)
        //         │   ├── CAN1 (C)
        //         │   └── ADC0
        //         └── GCLK3(2Mhz)
        //              └── CAN0 (B)

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
        let (gclk0, dfll, _dpll0) = clocks.gclk0.swap_sources(dfll, dpll0);
        // Start GCLK2 off DPLL1 with a divider of 2 (160Mhz/2) = 80Mhz
        let (gclk2_uninit, dpll1) = Gclk::from_source(tokens.gclks.gclk2, dpll1);
        let (gclk3_uninit, dfll) = Gclk::from_source(tokens.gclks.gclk3, dfll);
        let (gclk4_uninit, dfll) = Gclk::from_source(tokens.gclks.gclk4, dfll);
        let gclk2 = gclk2_uninit.div(GclkDiv8::Div(2)).enable();
        let gclk3 = gclk3_uninit.div(GclkDiv8::Div(24)).enable();
        let gclk4 = gclk4_uninit.enable();

        // Peripheral clock enabling
        let (pclk_sercom1, gclk2) = Pclk::enable(tokens.pclks.sercom1, gclk2);
        let (pclk_canb, _gclk3) = Pclk::enable(tokens.pclks.can0, gclk3);
        let (pclk_canc, gclk2) = Pclk::enable(tokens.pclks.can1, gclk2);
        let (pclk_adc0, _gclk2) = Pclk::enable(tokens.pclks.adc0, gclk2);
        let (pclk_usb, _gclk4) = Pclk::enable(tokens.pclks.usb, gclk4);

        let usb_bus = UsbBus::new(
            &pclk_usb.into(),
            &mut mclk,
            pins.usb_dm,
            pins.usb_dp,
            cx.device.usb,
        );

        let usb_allocator = cx.local.usb_bus_alloc.insert(UsbBusAllocator::new(usb_bus));

        let serial = usbd_serial::SerialPort::new(usb_allocator);

        let usb_device = UsbDeviceBuilder::new(
            usb_allocator,
            UsbVidPid(0x16c0, 0x0001),
        )
        .strings(&[StringDescriptors::default()
            .manufacturer("rnd-ash")
            .serial_number("211")
            .product("w211-pc-mcu")])
        .expect("Failed to set USB Strings")
        .usb_rev(atsamd_hal::usb::usb_device::device::UsbRev::Usb200)
        .device_protocol(usbd_serial::USB_CLASS_CDC)
        .max_packet_size_0(64)
        .unwrap()
        .self_powered(true)
        .max_power(240)
        .unwrap()
        .build();

        let (serial_rx_sender, serial_tx_sender) = make_channel!(SerialCanFrame, 200);

        let usb_isr_info = UsbIsrInfo {
            dev: usb_device,
            serial,
            last_state: atsamd_hal::usb::usb_device::device::UsbDeviceState::Default,
            buf: [0; SERIAL_FRAME_LEN],
            sender_can_frames: serial_rx_sender
        };


        // Enable RTC and start time driver for RTIC using that
        let (osculp32k, _) = OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
        let _ = RtcOsc::enable(tokens.rtcosc, osculp32k);
        defmt::info!("Mono start");
        Mono::start(cx.device.rtc); // Start time driver now that clocks are ready
        defmt::info!("Mono started");
        // -- CAN Configuration and setup
        let (deps_canb, gclk0) = Dependencies::new(
            gclk0,
            pclk_canb,
            clocks.ahbs.can0,
            pin_alias!(pins.pa23).into_mode(),
            pin_alias!(pins.pa22).into_mode(),
            cx.device.can0,
        );
        let (deps_canc, _gclk0) = Dependencies::new(
            gclk0,
            pclk_canc,
            clocks.ahbs.can1,
            pin_alias!(pins.pb13).into_mode(),
            pin_alias!(pins.pb12).into_mode(),
            cx.device.can1,
        );
        let mut can_b =
            mcan::bus::CanConfigurable::new(83_333u32.Hz(), deps_canb, cx.local.can_memory0)
                .unwrap();
        let mut can_c =
            mcan::bus::CanConfigurable::new(500_000u32.Hz(), deps_canc, cx.local.can_memory1)
                .unwrap();

        let (tx_serial_can, rx_serial_can) = rtic_sync::make_channel!(SerialCanFrame, 200);

        let (tx_ezs_a1, rx_ezs_a1) = rtic_sync::make_channel!([u8; 8], 10);
        let (tx_mrm, rx_mrm) = rtic_sync::make_channel!((u16, [u8; 8]), 10);
        let (tx_cane, rx_cane) = rtic_sync::make_channel!(SerialCanFrame, 10);

        can_b.config().loopback = false;
        can_b.config().mode = mcan::config::Mode::Classic;
        can_b.config().timestamp = mcan::config::Timestamp::default();
        can_b.config().nominal_timing.allow_fractional = true;
        can_b.config().nominal_timing.phase_seg_1 = 20;
        can_b.config().nominal_timing.phase_seg_2 = 3;

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
        
        let can_b = can_b.finalize_initialized().unwrap();
        let can_c = can_c.finalize_initialized().unwrap();
        can_b.aux.operational_mode();
        can_c.aux.operational_mode();

        // ADC0 setup (For Vsense, Current_PC, Current_Amp)
        let apb_adc = buses.apb.enable(tokens.apbs.adc0);
        let adc0 = AdcBuilder::new(adc::Accumulation::Single(adc::AdcResolution::_12))
            .with_clock_cycles_per_sample(8)
            .with_clock_divider(adc::Prescaler::Div4)
            .with_vref(adc::Reference::Intvcc1)
            .enable(cx.device.adc0, apb_adc, &pclk_adc0)
            .unwrap()
            .into_future(Adc0Irqs);

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
            adc0,
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
        io_controller::spawn(cx.device.supc).unwrap();
        (
            Shared { 
                canb_tx: can_b.tx,
                canc_tx: can_c.tx,
                usb_isr_info
            },
            Local {
                board_io,
                onboard_led,
                tx_ezs_a1,
                tx_mrm_data: tx_mrm,
                canb_aux: can_b.aux,
                canb_rx: can_b.rx_fifo_0,
                canc_rx: can_c.rx_fifo_0,
                canb_serial_tx: tx_serial_can.clone(),
                canc_serial_tx: tx_serial_can,
                can_to_serial_recv: rx_serial_can,
                serial_to_can_recv: serial_tx_sender,
                
                canb_interrupts: line_interrupts_canb,
                canc_interrupts: line_interrupts_canc,
                paddle_emu,
                tx_cane
            },
        )
    }

    #[task(priority=1, local=[board_io, paddle_emu], shared=[canc_tx])]
    async fn io_controller(
        mut cx: io_controller::Context,
        mut supc: Supc
    ) {
        defmt::info!("IO Controller task started");
        loop {
            let key_state = cx.local.board_io.update(&mut supc).await;
            let tx_canc_mrm = cx.local.paddle_emu.generate_mrm_tx_frame();
            KEY_IN_EZS.store(key_state, Ordering::Relaxed);
            if key_state {
                let f = SerialCanFrame::new(CanNet::C, 0x232, &tx_canc_mrm);
                let _ = cx.shared.canc_tx.lock(|canc| {
                    canc.transmit_queued(Message::new(f.to_can_msg()).unwrap())
                });
            }
            Mono::delay(20u64.millis()).await;
        }
    }

    #[task(priority=1, local=[can_to_serial_recv], shared=[usb_isr_info])]
    async fn serial_tx_handler(mut cx: serial_tx_handler::Context) {
        defmt::info!("Serial TX task started");
        let serial_tx_handler::LocalResources {
            can_to_serial_recv,
            ..
        } = cx.local;
        let mut buf = [0u8; 16];
        // Never changes so we can just place it here
        loop {
            if let Ok(frame) = can_to_serial_recv.recv().await {
                if KEY_IN_EZS.load(Ordering::Relaxed) {
                    // Only send to UART if PC is alive
                    frame.to_bytes(&mut buf);
                    let res = cx.shared.usb_isr_info.lock(|usb| {
                        usb.serial.write(&buf)
                    });
                    
                }
            }
        }
    }

    #[task(priority=1, local=[onboard_led, tx_cane, serial_to_can_recv], shared=[canb_tx, canc_tx])]
    async fn serial_rx_handler(mut cx: serial_rx_handler::Context) {
        let serial_rx_handler::LocalResources {
            onboard_led,
            serial_to_can_recv,
            ..
        } = cx.local;
        defmt::info!("Serial RX task started");
        loop {
            let _ = onboard_led.set_high();
            match serial_to_can_recv.recv().await {
                Ok(frame) => {
                    let _ = onboard_led.set_low();
                    PC_AWAKE.store(true, Ordering::Relaxed);
                    match frame.net {
                        1 => {
                            // B
                            if KEY_IN_EZS.load(Ordering::Relaxed) {
                                let _ = cx.shared.canb_tx.lock(|canb| {
                                    canb.transmit_queued(Message::new(frame.to_can_msg()).unwrap())
                                });
                            }
                        }
                        2 => {
                            // C
                            if KEY_IN_EZS.load(Ordering::Relaxed) {
                                let _ = cx.shared.canc_tx.lock(|canc| {
                                    canc.transmit_queued(Message::new(frame.to_can_msg()).unwrap())
                                });
                            }
                        }
                        3 => { // E (Internal to command controller)
                            let _ = cx.local.tx_cane.try_send(frame);
                        }
                        _ => {
                            defmt::error!("Invalid CAN Net byte {}", frame.net)
                        }
                    }
                    let _ = onboard_led.set_high();
                }
                _ => {
                    // TODO - Handle error here
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

    #[task(priority = 3, binds=USB_TRCPT0, shared=[usb_isr_info])]
    #[unsafe(link_section = ".data.usb_trcpt0")]
    fn usb_trcpt0(mut cx: usb_trcpt0::Context) {
        //cx.shared.usb_polls.lock(|x| *x += 1);
        cx.shared.usb_isr_info.lock(|lck| poll_usb(lck))
    }

    #[task(priority = 3, binds=USB_TRCPT1, shared=[usb_isr_info])]
    #[unsafe(link_section = ".data.usb_trcpt1")]
    fn usb_trcpt1(mut cx: usb_trcpt1::Context) {
        //cx.shared.usb_polls.lock(|x| *x += 1);
        cx.shared.usb_isr_info.lock(|lck| poll_usb(lck));
    }

    #[task(priority = 3, binds=USB_OTHER, shared=[usb_isr_info])]
    #[unsafe(link_section = ".data.usb_other")]
    fn usb_other(mut cx: usb_other::Context) {
        //cx.shared.usb_polls.lock(|x| *x += 1);
        cx.shared.usb_isr_info.lock(|lck| poll_usb(lck))
    }

    #[inline]
    #[unsafe(link_section = ".data.poll_usb")]
    fn poll_usb(info: &mut UsbIsrInfo) {

        if info.dev.state() != info.last_state {
            info.last_state = info.dev.state();
            info.buf = [0; SERIAL_FRAME_LEN];
        }

        if info.dev.poll(&mut [&mut info.serial]) {
            if let Some(frame) = uart_read_frame(&mut info.serial, &mut info.buf) {
                let _ = info.sender_can_frames.try_send(frame);
            }
        }
    }
}

pub static PC_AWAKE: AtomicBool = AtomicBool::new(false);
pub static KEY_IN_EZS: AtomicBool = AtomicBool::new(false);