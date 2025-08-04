use core::cell::RefCell;

use embassy_sync::blocking_mutex::ThreadModeMutex;
use w211_can::canb::MRM_A1;


pub struct PaddleEmulator {
    last_mrm_a1: ThreadModeMutex<RefCell<MRM_A1>>,
    last_mrm_a2_bytes: ThreadModeMutex<RefCell<[u8; 8]>>,
}

impl PaddleEmulator {
    pub const fn new() -> Self {
        Self {
            last_mrm_a1: ThreadModeMutex::new(RefCell::new(MRM_A1(0))),
            last_mrm_a2_bytes: ThreadModeMutex::new(RefCell::new([0; 8])),
        }
    }

    pub fn set_mrm_a1(&self, mrm: MRM_A1) {
        self.last_mrm_a1.lock(|inner| {
            *inner.borrow_mut() = mrm;
        })
    }

    pub fn set_mrm_a2(&self, mrm: [u8; 8]) {
        self.last_mrm_a2_bytes.lock(|inner| {
            *inner.borrow_mut() = mrm;
        })
    }

    pub fn generate_mrm_tx_frame(&self) -> [u8; 4] {
        // Process state
        let mut paddle = 0u8;
        let mrm_a1 = self.last_mrm_a1.borrow().borrow().clone();
        let mrm_a2 = self.last_mrm_a2_bytes.borrow().borrow().clone();

        // Wheel button MUX mode (Custom (Ash mode))
        if mrm_a2[0] == 0x10 {
            if mrm_a2[1] == 0x0F {
                paddle = 1; // Upshift pressed
            } else if mrm_a2[1] == 0x0E {
                // Downshift pressed
                //
                // WAIT. The high beam stalk collides with this paddle.
                // Check if the high beam temporary stalk is pulled, and then
                // don't register this if it is pressed since its impossible for
                // a user to press both high beam and shift paddle.
                if !mrm_a1.get_LHP_EIN() {
                    paddle = 2;
                }
            } else if mrm_a2[1] == 0x01 {
                // IC Undo button pressed

            }
        }
        return [paddle, 0, 0, 0];
    }
}
