use std::time::Duration;

use crate::custom_display_format::CDMIsoTp;
use futures_util::StreamExt;
use tokio::runtime::Runtime;
use w211_can::{canbus::{frame_to_u64, CanBus}, canc::{GS_418h_FPC, GS_418}, tokio_socketcan::CANFilter};

/// DANGER TO MANIFOLD!
pub struct DestructionMonitor {
}

impl DestructionMonitor {
    pub fn new(vlad: CDMIsoTp, rt: &Runtime) -> Self {
        rt.spawn(async move {
            let mut can_c_socket = CanBus::C.create_can_socket().unwrap();
            can_c_socket.set_filter(&[
                CANFilter::new(0x418, 0xFFF).unwrap()
            ]).unwrap();

            let mut open_display = false;
            while let Some(Ok(f)) = can_c_socket.next().await {
                let (data, _) = frame_to_u64(&f);
                let f = GS_418::new(data);
                if let Some(GS_418h_FPC::HOCH) = f.get_FPC() {
                    if !open_display {
                        // Open display!
                        vlad.show_display("~I3~C2~P0000~J2~Z~G2WARNING!!!~L~L~G1Danger to Manifold~P7890".into(), u32::MAX);
                        tokio::time::sleep(Duration::from_millis(40)).await;
                        vlad.sound_buzzer(crate::custom_display_format::ToneType::LongBeep, crate::custom_display_format::ToneRepeatType::Fast);
                        open_display = true;
                    }
                } else {
                    if open_display {
                        vlad.stop_display();
                        tokio::time::sleep(Duration::from_millis(40)).await;
                        vlad.stop_buzzer();
                        open_display = false;
                    }
                }
            }
            vlad.stop_buzzer();
            vlad.stop_display();
        });
        Self{}
    }
}