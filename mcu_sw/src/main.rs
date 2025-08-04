#![no_std]
#![no_main]

mod bsp;
mod can;
mod io;
mod paddle_emu;

use atsamd_hal::{adc, pac, sercom::Sercom1};

use cortex_m_rt::{entry, exception};
use embassy_executor::{raw::Executor, Spawner};
use embassy_sync::{blocking_mutex::{raw::{CriticalSectionRawMutex, ThreadModeRawMutex}, ThreadModeMutex}, channel::Channel, once_lock::OnceLock};
use embassy_time::{Instant, Timer};
use mcan_core::CanId;
use static_cell::StaticCell;
use systick_timer::SystickDriver; // For panics

use core::{cell::RefCell, panic::PanicInfo, sync::atomic::{AtomicBool, Ordering}};

use crate::{
    bsp::{uart, AmpCurrentSense, AmpMosfet, AmpMute, AmpStandby, PcCurrentSense, PcMosfet, UartPads, VSense},
    can::{frame_to_int, uart_read_frame, Can0RxFifo0, Can1Aux, Can1RxFifo0, SERIAL_FRAME_LEN},
    io::BoardIO,
    paddle_emu::PaddleEmulator,
};
use atsamd_hal::{dmac::{Ch0, Ch1}, pac::{Interrupt, Supc, NVIC}};
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
use cortex_m::{asm::{nop, wfi}, interrupt::{free, CriticalSection, Mutex}};
use defmt_rtt as _;
use fugit::*;
use mcan::{
    bus::DynAux, embedded_can::{self, Id}, filter::Filter, interrupt::{state::EnabledLine0, OwnedInterruptSet}, message::Raw, messageram::SharedMemory, rx_fifo::DynRxFifo, tx_buffers::{DynTx, Tx}
};

use w211_can::canb::{EZS_A1_CAN_ID, MRM_A1, MRM_A1_CAN_ID, MRM_A2_CAN_ID};


atsamd_hal::bind_multiple_interrupts!(struct DmacIrqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

atsamd_hal::bind_multiple_interrupts!(struct Sercom1Irqs {
    SERCOM1: [SERCOM1_0, SERCOM1_1, SERCOM1_2, SERCOM1_OTHER] => atsamd_hal::sercom::uart::InterruptHandler<Sercom1>;
});

atsamd_hal::bind_multiple_interrupts!(struct Adc0Irqs {
    ADC0: [ADC0_RESRDY, ADC0_OTHER] => atsamd_hal::adc::InterruptHandler<adc::Adc0>;
});

#[panic_handler]
fn panic(p: &PanicInfo) -> ! {
    defmt::error!("PANIC: {:?}", p);
    loop{}
}

static KEY_ACTIVE: AtomicBool = AtomicBool::new(false);

#[link_section = ".can"]
static mut CAN0_MEM: SharedMemory<Capacities> = SharedMemory::new();
#[link_section = ".can"]
static mut CAN1_MEM: SharedMemory<Capacities> = SharedMemory::new();

// Systick handler
embassy_time_driver::time_driver_impl!(static DRIVER: SystickDriver<16> = SystickDriver::new(100_000_000, 0xA0_FF));

#[exception]
fn SysTick() {
    DRIVER.systick_interrupt();
}

pub static EXECUTOR_THREAD_LOW: StaticCell<Executor> = StaticCell::new();
pub static SPAWNER_THREAD: StaticCell<Spawner> = StaticCell::new();

#[entry]
fn main() -> ! {
    // Initialization (Called on startup)
    let mut bsp_peripherals = atsamd_hal::pac::Peripherals::take().unwrap();
    let mut core_peripherals = cortex_m::Peripherals::take().unwrap();
    let pins = bsp::Pins::new(bsp_peripherals.port);
    // CPU is at default 48Mhz here
    let (mut buses, clocks, tokens) = clock_system_at_reset(
        bsp_peripherals.oscctrl,
        bsp_peripherals.osc32kctrl,
        bsp_peripherals.gclk,
        bsp_peripherals.mclk,
        &mut bsp_peripherals.nvmctrl,
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
    let (_gclk0, _dfll, _dpll0) = clocks.gclk0.swap_sources(dfll, dpll0);
    // Start GCLK2 off DPLL1 with a divider of 2 (160Mhz/2) = 80Mhz
    let (gclk2_uninit, _dpll1) = Gclk::from_source(tokens.gclks.gclk2, dpll1);
    let gclk2 = gclk2_uninit.div(GclkDiv8::Div(2)).enable();

    // Peripheral clock enabling
    let (pclk_sercom1, gclk2) = Pclk::enable(tokens.pclks.sercom1, gclk2);
    let (pclk_canb, gclk2) = Pclk::enable(tokens.pclks.can0, gclk2);
    let (pclk_canc, gclk2) = Pclk::enable(tokens.pclks.can1, gclk2);
    let (pclk_adc0, gclk2) = Pclk::enable(tokens.pclks.adc0, gclk2);
    // OS START
    DRIVER.start(&mut core_peripherals.SYST);
    let executor = EXECUTOR_THREAD_LOW.init(Executor::new(usize::MAX as *mut ()));
    let spawner = SPAWNER_THREAD.init(executor.spawner());

    // -- CAN Configuration and setup
    let (deps_canb, gclk2) = Dependencies::new(
        gclk2,
        pclk_canb,
        clocks.ahbs.can0,
        pin_alias!(pins.pa23).into_mode(),
        pin_alias!(pins.pa22).into_mode(),
        bsp_peripherals.can0,
    );
    let (deps_canc, gclk2) = Dependencies::new(
        gclk2,
        pclk_canc,
        clocks.ahbs.can1,
        pin_alias!(pins.pb13).into_mode(),
        pin_alias!(pins.pb12).into_mode(),
        bsp_peripherals.can1,
    );

    // CANB is set to 83333 below via a hack
    let mut can_b =
        mcan::bus::CanConfigurable::new(500_000u32.Hz(), deps_canb, unsafe { &mut *(&raw mut CAN0_MEM) })
            .unwrap();
    let mut can_c =
        mcan::bus::CanConfigurable::new(500_000u32.Hz(), deps_canc, unsafe { &mut *(&raw mut CAN1_MEM) })
            .unwrap();

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
    can_c.aux.operational_mode(); // Start up CAN C (CAN B is started below)

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
    can_b.aux.operational_mode();


    CANB_TX.init(ThreadModeMutex::new(RefCell::new(can_b.tx))).unwrap_or_else(|_|panic!("CANB TX init failed"));
    CANC_TX.init(ThreadModeMutex::new(RefCell::new(can_c.tx))).unwrap_or_else(|_|panic!("CANC TX init failed"));
    free(|cs| {
        CANB_ISR_DATA.init(cs, (can_b.rx_fifo_0, can_b.aux, line_interrupts_canb));
        CANC_ISR_DATA.init(cs, (can_c.rx_fifo_0, can_c.aux, line_interrupts_canc));

        // Wire the interrupts for CAN0 up to the CPU, so it can wake up
        // when there is activity on CAN0
        unsafe {
            core_peripherals.NVIC.set_priority(Interrupt::CAN0, 2);
            NVIC::unmask(Interrupt::CAN0);
            NVIC::unpend(Interrupt::CAN0);
            core_peripherals.NVIC.set_priority(Interrupt::CAN1, 2);
            NVIC::unmask(Interrupt::CAN1);
            NVIC::unpend(Interrupt::CAN1);
        }
        
    });

    unsafe {
        let mask = Peripherals::steal().can0.ie().read().bits();
        defmt::info!("CAN0 IE: {:032b}", mask);
        let c1_en = NVIC::is_enabled(Interrupt::CAN0);
        let c2_en = NVIC::is_enabled(Interrupt::CAN1);
        defmt::info!("NVIC: {} {}", c1_en, c2_en);
    }

    // -- DMA Setup - Use DMA for Tx and Rx of the UART peripheral
    let dmac = DmaController::init(bsp_peripherals.dmac, &mut bsp_peripherals.pm);
    let dma_channels = dmac.into_future(DmacIrqs).split();
    let dma_uart_rx = dma_channels.0.init(PriorityLevel::Lvl0);
    let dma_uart_tx = dma_channels.1.init(PriorityLevel::Lvl0);

    // -- UART SETUP (Talk to PC)
    let uart = uart(
        pclk_sercom1,
        921600u32.Hz(),
        bsp_peripherals.sercom1,
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
    let apb_adc = buses.apb.enable(tokens.apbs.adc0);

    let adc0_fut = atsamd_hal::adc::AdcBuilder::new(adc::Accumulation::Single(adc::AdcResolution::_12))
        .with_vref(adc::Reference::Intvcc1)
        .with_clock_divider(adc::Prescaler::Div4)
        .with_clock_cycles_per_sample(32)
        .enable(bsp_peripherals.adc0, apb_adc, &pclk_adc0)
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
        adc0_fut,
        amp_mosfet,
        pc_mosfet,
        amp_mute,
        amp_standby,
        can_c_shutdown,
        v_sense,
        curr_sense_pc,
        curr_sense_amp,
        Instant::now()
    );

    spawner.must_spawn(can_rx_handler(uart_tx));
    spawner.must_spawn(can_tx_handler(uart_rx, onboard_led));
    spawner.must_spawn(board_io_loop(board_io, bsp_peripherals.supc));
    defmt::info!("All tasks are started");
    loop{
        cortex_m::asm::wfi();
        unsafe { executor.poll(); }
    }
}

static CAN_RX_CHANNEL: Channel<CriticalSectionRawMutex, SerialCanFrame, 200> = Channel::new();
pub static CANE_TO_BOARD_CHANNEL: Channel<ThreadModeRawMutex, SerialCanFrame, 50> = Channel::new();
static PADDLE_EMULATOR: PaddleEmulator = PaddleEmulator::new();

pub struct OptionalMutMutex<T>(pub Mutex<RefCell<Option<T>>>);

impl<T> Default for OptionalMutMutex<T> {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl<T> OptionalMutMutex<T> {
    pub const fn new_empty() -> Self {
        Self(Mutex::new(RefCell::new(None)))
    }

    pub fn new_value(v: T) -> Self {
        Self(Mutex::new(RefCell::new(Some(v))))
    }

    pub fn init(&self, cs: &CriticalSection, v: T) {
        self.0.borrow(cs).borrow_mut().replace(v);
    }

    #[inline(always)]
    pub fn with<R, F: FnOnce(&mut T) -> R>(&self, cs: &CriticalSection, f: F) -> Option<R> {
        self.0.borrow(cs).borrow_mut().as_mut().map(|x| f(x))
    }
}


static CANB_ISR_DATA: OptionalMutMutex<(Can0RxFifo0, Can0Aux<Gclk2Id>, OwnedInterruptSet<Can0, EnabledLine0>)> = OptionalMutMutex::new_empty();
static CANC_ISR_DATA: OptionalMutMutex<(Can1RxFifo0, Can1Aux<Gclk2Id>, OwnedInterruptSet<Can1, EnabledLine0>)> = OptionalMutMutex::new_empty();

static CANB_TX: OnceLock<ThreadModeMutex<RefCell<Tx<'static, Can0, Capacities>>>> = OnceLock::new();
static CANC_TX: OnceLock<ThreadModeMutex<RefCell<Tx<'static, Can1, Capacities>>>> = OnceLock::new();

use pac::interrupt;

#[interrupt]
fn CAN0() {
    free(|cs| {
        CANB_ISR_DATA.with(cs, |(fifo, aux, interrupts)| {
            for interrupt  in interrupts.iter_flagged() {
                match interrupt {
                    mcan::interrupt::Interrupt::RxFifo0NewMessage => {
                        for msg in fifo.into_iter() {
                            if let Id::Standard(std_id) = msg.id() {
                                let mut d = [0u8; 8];
                                d[..msg.dlc() as usize].copy_from_slice(&msg.data());
                                let _ = CAN_RX_CHANNEL.sender().try_send(SerialCanFrame {
                                    net: CanNet::B,
                                    id: std_id.as_raw(),
                                    dlc: msg.dlc(),
                                    data: d,
                                });
                            }
                        }
                    },
                    _ => {
                        defmt::warn!("Unhandled CANB interrupt {}", interrupt as u8)
                    }
                }
            }
        });
    })
}

#[interrupt]
fn CAN1() {
    free(|cs| {
        CANC_ISR_DATA.with(cs, |(fifo, aux, interrupts)| {
            for interrupt  in interrupts.iter_flagged() {
                match interrupt {
                    mcan::interrupt::Interrupt::RxFifo0NewMessage => {
                        for msg in fifo.into_iter() {
                            if let Id::Standard(std_id) = msg.id() {
                                let mut d = [0u8; 8];
                                d[..msg.dlc() as usize].copy_from_slice(&msg.data());
                                let _ = CAN_RX_CHANNEL.sender().try_send(SerialCanFrame {
                                    net: CanNet::C,
                                    id: std_id.as_raw(),
                                    dlc: msg.dlc(),
                                    data: d,
                                });
                            }
                        }
                    },
                    _ => {
                        defmt::warn!("Unhandled CANC interrupt {}", interrupt as u8)
                    }
                }
            }
        });
    })
}

#[embassy_executor::task]
async fn can_rx_handler(
    mut uart_tx: UartFutureTxDuplexDma<uart::Config<UartPads>, Ch1>,
) {
    let mut buf = [0u8; SERIAL_FRAME_LEN];
    defmt::info!("CAN->UART start");
    loop {
        let msg = CAN_RX_CHANNEL.receive().await;
        msg.to_bytes(&mut buf);
        match (msg.net, msg.id) {
            (CanNet::B, EZS_A1_CAN_ID) => {
                let _ = CANE_TO_BOARD_CHANNEL.sender().try_send(msg);
            }
            (CanNet::B, MRM_A1_CAN_ID) => {
                PADDLE_EMULATOR.set_mrm_a1(MRM_A1::new(frame_to_int(&msg.data, 8)));
            },
            (CanNet::B, MRM_A2_CAN_ID) => {
                PADDLE_EMULATOR.set_mrm_a2(msg.data);
            }
            _ => {}
        }
        if KEY_ACTIVE.load(Ordering::Relaxed) {
            if let Err(e) = uart_tx.write(&buf).await {
                defmt::error!("UART TX failed: {:?}", e)
            }
        }
    }
}

#[inline(always)]
fn tx_to_can<C: CanId>(can: &ThreadModeMutex<RefCell<Tx<'static, C, Capacities>>>, f: SerialCanFrame) -> Option<()> {
    let frame = f.to_can_msg()?;
    can.lock(|tx| {
        tx.borrow_mut().transmit_queued(frame.build().unwrap())
    }).ok()
}

#[embassy_executor::task]
async fn can_tx_handler(
    mut uart_rx: UartFutureRxDuplexDma<uart::Config<UartPads>, Ch0>,
    mut led: OnboardLED
) {
    defmt::info!("CAN<-UART start");
    loop {
        led.set_high().unwrap();
        match uart_read_frame(&mut uart_rx).await {
            Some(frame) => {
                led.set_low().unwrap();
                if KEY_ACTIVE.load(Ordering::Relaxed) {
                    match frame.net {
                        CanNet::B => {
                            let _ = tx_to_can(CANB_TX.get().await, frame);
                        }
                        CanNet::C => {
                            let _ = tx_to_can(CANC_TX.get().await, frame);
                        }
                        CanNet::E => { // E (Internal to command controller)
                            let _ = CANE_TO_BOARD_CHANNEL.try_send(frame);
                        }
                    }
                }
            }
            None => {
                // TODO - Handle error here
            }
        }
    }
}

#[embassy_executor::task]
async fn board_io_loop(
    mut board_io: BoardIO,
    mut supc: Supc
) {
    defmt::info!("Board IO thread start");
    loop {
        let key_state = board_io.update(&mut supc).await;
        let tx_mrm = PADDLE_EMULATOR.generate_mrm_tx_frame();
        KEY_ACTIVE.store(key_state, Ordering::Relaxed);
        if key_state {
            let f = SerialCanFrame::new(CanNet::C, 0x232, &tx_mrm);
            tx_to_can(CANB_TX.get().await, f);
        }
        let wait = if false { // shutdown
            1000
        } else {
            20
        };
        Timer::after_millis(wait).await;
    }
}