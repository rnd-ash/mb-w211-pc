pub use atsamd_hal as hal;
use atsamd_hal::clock::v2::gclk::{Gclk0Id, Gclk2Id};
use atsamd_hal::clock::v2::pclk::Pclk;
use atsamd_hal::sercom::{IoSet1, Sercom1};
pub use hal::pac;

use hal::sercom::uart::{self, BaudMode, Oversampling};
use hal::time::Hertz;

hal::bsp_peripherals!(Sercom1 { UartSercom });

hal::bsp_pins!(
    PA04 {
        // ADC0[4]
        name: pc_c_sense
        aliases: {
            AlternateB: PcCurrentSense
        }
    }
    PB04 {
        name: amp_mosfset
        aliases: {
            PushPullOutput: AmpMosfet
        }
    }
    PA05 {
        name: amp_mute
        aliases: {
            PushPullOutput: AmpMute
        }
    }
    PB05 {
        name: pc_mosfet
        aliases: {
            PushPullOutput: PcMosfet
        }
    }
    PA07 {
        // ADC0[0] 
        name: vsense
        aliases: {
            AlternateB: VSense
        }
    }
    PB09 {
        // ADC0[3]
        name: amp_c_sense
        aliases: {
            AlternateB: AmpCurrentSense
        }
    }
    PB12 {
        name: pb12
        aliases: {
            AlternateH: CANCTx,
        }
    }
    PB13 {
        name: pb13
        aliases: {
            AlternateH: CANCRx,
        }
    }
    PA16 {
        name: pa16
        aliases: {
            AlternateC: UartTx
        }
    }
    PA17 {
        name: pa17
        aliases: {
            AlternateC: UartRx
        }
    }
    PA14 {
        name: onboard_led,
        aliases: {
            PushPullOutput: OnboardLED
        }
    }
    PA12 {
        name: can_c_shutdown
        aliases: {
            PushPullOutput: CanCShutdown
        }
    }
    PA22 {
        name: pa22,
        aliases: {
            AlternateI: CANBTx,
        }
    }
    PA23 {
        name: pa23,
        aliases: {
            AlternateI: CANBRx,
        }
    }
    PA27 {
        name: amp_standby
        aliases: {
            PushPullOutput: AmpStandby
        }
    }
);

/// UART pads for the labelled RX & TX pins
pub type UartPads = uart::Pads<UartSercom, IoSet1, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    pclk_sercom1: Pclk<Sercom1, Gclk2Id>,
    baud: impl Into<Hertz>,
    sercom: Sercom1,
    mclk: &mut pac::Mclk,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, sercom, pads, pclk_sercom1.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}
