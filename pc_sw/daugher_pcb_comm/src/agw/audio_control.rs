use std::{sync::{atomic::AtomicBool, Arc}, time::Duration};
use w211_can::{canbus::CanBus, socketcan::{CanSocket, Socket, CanFrame, CanDataFrame, EmbeddedFrame}, socketcan_isotp::{Id, StandardId}};

pub const MAX_VOLUME: f32 = 0.8;

pub const AUDIO_OUTPUT: &str = "alsa_output.usb-0d8c_USB_Sound_Device-00.analog-surround-51";
pub const AUDIO_SINK: &str = "upmixing_front";

pub enum PwChannel {
    FR,
    FL,
    RR,
    RL,
    FC,
    LFE
}


pub struct AudioManager {
    pub master_volume: f32, // 0 - 1
    pub muted: bool,
    amplifier_can: CanSocket,
}

impl AudioManager {

    pub fn new() -> AudioManager {

        let can =CanBus::E.create_can_socket();
        can.set_nonblocking(true);

        let man = Self {
            master_volume: 0.2,
            muted: false,
            amplifier_can: can,
        };
        //std::thread::spawn(move|| {
        std::thread::sleep(Duration::from_secs(5));
        // Connect all channels in pipewire subsytem
        AudioManager::connect_channel("output_FR", "playback_RR"); // Right channel connection
        AudioManager::connect_channel("output_FL", "playback_RL"); // Left channel connection
        AudioManager::connect_channel("output_FL", "playback_LFE"); // LFE connection
        AudioManager::connect_channel("output_FR", "playback_LFE"); // LFE connection
        AudioManager::connect_channel("output_FR", "playback_FC"); // Center connection
        AudioManager::connect_channel("output_FL", "playback_FC"); // Center connection
        //});
        man.update_all_channels();
        man
    }

    fn connect_channel(c_in: &str, c_out: &str) {
        println!("{:?}", std::process::Command::new("/usr/bin/pw-link")
            .args([
                &format!("{AUDIO_SINK}:{c_in}"),
                &format!("{AUDIO_OUTPUT}:{c_out}"),
            ])
            .output());
    }

    fn update_volume(nv: f32) {
        let _ = std::process::Command::new("/usr/bin/wpctl")
            .args([
                "set-volume",
                "@DEFAULT_AUDIO_SINK@",
                &format!("{nv:.2}"),
                "'Master'"
            ])
            .output();
    }

    fn update_all_channels(&self) {
        let tx_byte: u8 = if self.master_volume > 0.0 {
            0x01
        } else {
            0x00
        };
        self.amplifier_can.write_frame(
            &CanFrame::Data(
                CanDataFrame::new(
                    Id::Standard(unsafe { StandardId::new_unchecked(0x001) }),
                    &[tx_byte]
                ).unwrap()
            )
        );
        if self.muted {
            Self::update_volume(0.0)
        } else {
            Self::update_volume(self.master_volume)
        }
    }

    pub fn offset_volume(&mut self, o: f32) {
        self.muted = false;
        if (self.master_volume - o) < 0.0 {
            self.set_volume(0.0)
        };
        self.set_volume(self.master_volume + o);
    }

    pub fn set_volume(&mut self, v: f32) {
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

