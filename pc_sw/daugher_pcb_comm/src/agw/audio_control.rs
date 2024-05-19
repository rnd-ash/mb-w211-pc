use std::sync::Arc;

use w211_can::{canbus::CanBus, tokio_socketcan::{CANFrame, CANSocket}};

pub const MAX_VOLUME: u16 = u16::MAX/2; // &

pub const AUDIO_OUTPUT: &str = "alsa_output.usb-0d8c_USB_Sound_Device-00.analog-surround-51";


pub struct AudioManager {
    pub master_volume: u16, // 0 - 1
    pub muted: bool,
    amplifier_can: Arc<CANSocket>,
}

impl AudioManager {

    pub fn new() -> AudioManager {

        let can = CanBus::E.create_can_socket().unwrap();

        let man = Self {
            master_volume: MAX_VOLUME/4,
            muted: false,
            amplifier_can: Arc::new(can),
        };
        //println!("{:?}", Command::new("/usr/bin/pulseaudio").arg("-k").output());
        //println!("{:?}", Command::new("/usr/bin/pulseaudio").arg("--start").output());
        man.update_all_channels();
        man
    }

    fn update_volume(nv: u16) {
        let _ = std::process::Command::new("/usr/bin/pacmd")
            .args([
                "set-sink-volume",
                AUDIO_OUTPUT,
                &format!("{nv}")
            ])
            .output();
    }

    fn write_can(&self) {
        let tx_byte: u8 = if self.master_volume > 0 {
            0x01
        } else {
            0x00
        };
        let can = self.amplifier_can.clone();
        tokio::spawn(async move {
            let _ = can.write_frame(
                CANFrame::new(0x001, &[tx_byte], false, false).unwrap()
            ).unwrap().await;
        });
    }

    fn update_all_channels(&self) {
        if self.muted {
            Self::update_volume(0)
        } else {
            Self::update_volume(self.master_volume)
        }
        self.write_can();
    }

    pub fn offset_volume(&mut self, o: i16) {
        self.muted = false;
        if let Some(new) = self.master_volume.checked_add_signed(o) {
            self.set_volume(new);
        } else {
            self.set_volume(0)
        }
    }

    pub fn set_volume(&mut self, v: u16) {
        self.muted = false;
        if v > MAX_VOLUME {
            self.master_volume = MAX_VOLUME;
        } else {
            self.master_volume = v;
        }
        self.update_all_channels();
    }

    pub fn set_mute(&mut self, muted: bool) {
        self.muted = muted;
        self.update_all_channels();
    }
}

