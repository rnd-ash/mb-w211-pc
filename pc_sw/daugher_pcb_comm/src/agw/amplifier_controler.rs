use std::time::Duration;

use crate::w211can::CanBus;


pub struct AudioControl{
}

impl AudioControl {
    pub fn new() -> Self {
        let mut amp_state: u8 = 0x01;
        std::thread::spawn(move|| {
            let cane = CanBus::E.create_can_socket(&[]).unwrap();
            loop {
                amp_state = 0x00;
                if let Ok(s) = std::process::Command::new("/usr/bin/cat").args(["/proc/asound/card1/pcm0p/sub0/status"]).output().map(|x| x.stdout) {
                    if let Ok(str) = String::from_utf8(s) {
                        if str.contains("RUNNING") {
                            amp_state = 0x01;
                        }
                    }
                }
                cane.send_frame(0x0001, &[amp_state]);
                std::thread::sleep(Duration::from_millis(250));
            }
        });
        Self{}
    }
}