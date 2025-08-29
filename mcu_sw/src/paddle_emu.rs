use rtic_sync::channel::Receiver;
use w211_can::canb::{MRM_A1, MRM_A1_CAN_ID, MRM_A2_CAN_ID};

use crate::can::frame_to_int;

pub struct PaddleEmulator {
    last_mrm_a1: MRM_A1,
    last_mrm_a2_bytes: [u8; 8],

    rx_mrm_data: Receiver<'static, (u16, [u8; 8]), 10>,
}

impl PaddleEmulator {
    pub fn new(rx_mrm_data: Receiver<'static, (u16, [u8; 8]), 10>) -> Self {
        Self {
            last_mrm_a1: MRM_A1::default(),
            last_mrm_a2_bytes: [0; 8],
            rx_mrm_data,
        }
    }

    pub fn generate_mrm_tx_frame(&mut self) -> [u8; 4] {
        // Fetch data
        if let Ok((id, bytes)) = self.rx_mrm_data.try_recv() {
            if id == MRM_A1_CAN_ID {
                self.last_mrm_a1 = MRM_A1::new(frame_to_int(&bytes, 8));
            } else if id == MRM_A2_CAN_ID {
                self.last_mrm_a2_bytes = bytes;
            } else {
                defmt::error!("MRM Receiver invalid CAN ID: 0x{:03X}", id);
            }
        }

        // Process state
        let mut paddle = 0u8;
        // Wheel button MUX mode (Custom (Ash mode))
        if self.last_mrm_a2_bytes[0] == 0x10 {
            if self.last_mrm_a2_bytes[1] == 0x0F {
                paddle = 1; // Upshift pressed
            } else if self.last_mrm_a2_bytes[1] == 0x0E {
                // Downshift pressed
                //
                // WAIT. The high beam stalk collides with this paddle.
                // Check if the high beam temporary stalk is pulled, and then
                // don't register this if it is pressed since its impossible for
                // a user to press both high beam and shift paddle.
                if !self.last_mrm_a1.get_LHP_EIN() {
                    paddle = 2;
                }
            }
        }
        return [paddle, 0, 0, 0];
    }
}
