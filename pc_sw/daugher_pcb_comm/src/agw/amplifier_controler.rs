use std::time::Duration;

use crate::w211can::CanBus;


pub struct AudioControl{
}

impl AudioControl {
    pub fn new() -> Self {
        std::thread::spawn(move|| {
            let cane = CanBus::E.create_can_socket(&[]).unwrap();
            loop {
                cane.send_frame(0x0001, &[0x01]);
                std::thread::sleep(Duration::from_millis(250));
            }
        });
        Self{}
    }
}