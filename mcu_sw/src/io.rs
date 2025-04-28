use atsamd_hal::prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin;
use fugit::Instant;
use rtic_monotonics::Monotonic;
use rtic_sync::channel::Receiver;
use w211_can::canb::EZS_A1;

use crate::{
    bsp::{self, CanCShutdown},
    can::{frame_to_int, SerialCanFrame},
    Mono,
};

pub struct BoardIO {
    pub amp_mosfet: bsp::AmpMosfet,
    pub pc_mosfet: bsp::PcMosfet,

    pub amp_mute: bsp::AmpMute,
    pub amp_standby: bsp::AmpStandby,
    pub can_c_shutdown: CanCShutdown,

    pub rx_ezs_a1: Receiver<'static, [u8; 8], 10>,
    pub rx_cane: Receiver<'static, SerialCanFrame, 10>,
    pub last_ezsa1_time: Instant<u32, 1, 1000>,
    pub is_shutdown: bool,

    pub ezs_a1: w211_can::canb::EZS_A1,
}

impl BoardIO {
    pub fn new(
        amp_mosfet: bsp::AmpMosfet,
        pc_mosfet: bsp::PcMosfet,
        amp_mute: bsp::AmpMute,
        amp_standby: bsp::AmpStandby,
        can_c_shutdown: CanCShutdown,
        rx_ezs_a1: Receiver<'static, [u8; 8], 10>,
        rx_cane: Receiver<'static, SerialCanFrame, 10>,
        time: Instant<u32, 1, 1000>,
    ) -> Self {
        let mut s = Self {
            amp_mosfet,
            pc_mosfet,
            amp_mute,
            amp_standby,
            can_c_shutdown,
            rx_ezs_a1,
            rx_cane,
            last_ezsa1_time: time,
            is_shutdown: false,
            ezs_a1: EZS_A1::default(),
        };

        s.shutdown();
        s
    }

    pub fn shutdown(&mut self) {
        if !self.is_shutdown {
            defmt::info!("Shutting down");
            let _ = self.can_c_shutdown.set_high();
            let _ = self.amp_mute.set_low();
            let _ = self.amp_standby.set_low();
            let _ = self.amp_mosfet.set_low();
            let _ = self.pc_mosfet.set_low();
        }
        self.is_shutdown = true;
    }

    pub fn update(&mut self) -> bool {
        let mut can_alive = false;
        let mut key_in_ezs = false;
        let ezs_frame = self.rx_ezs_a1.try_recv();
        if let Ok(ezs_frame) = ezs_frame {
            self.last_ezsa1_time = Mono::now();
            self.ezs_a1 = EZS_A1::new(frame_to_int(&ezs_frame, 8));
            can_alive = true;
            if self.is_shutdown {
                defmt::info!("Waking up!");
            }
            self.is_shutdown = false;
            let _ = self.amp_mosfet.set_high();
            let _ = self.pc_mosfet.set_high();
            let _ = self.amp_standby.set_high();
            let _ = self.can_c_shutdown.set_low();
            // MUTE is not turned on here, it is turned on below by the CAN E events
        }
        if let Some(time_since_ezsa1) = Mono::now().checked_duration_since(self.last_ezsa1_time) {
            if time_since_ezsa1.to_millis() < 500 {
                // Assuming CAN is still active
                can_alive = true;
            } else if time_since_ezsa1.to_millis() > 5_000 {
                // Assuming CAN is dead

                // Shutdown
                self.shutdown();
            }
        }
        if can_alive {
            key_in_ezs = self.ezs_a1.get_KL_15R_EIN();

            let _ = self.amp_mute.set_state(key_in_ezs.into());
            let _ = self.can_c_shutdown.set_state((!key_in_ezs).into()); // Inverse!

            // Process any incommming events from PC now
            if let Ok(pc_evt) = self.rx_cane.try_recv() {
                if pc_evt.id == 0x0001 {
                    let en = pc_evt.data[0] == 0x01;
                    let _ = self.amp_mute.set_state(en.into());
                }
            }
        }

        key_in_ezs
    }
}
