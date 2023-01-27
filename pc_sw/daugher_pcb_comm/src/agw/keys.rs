use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use crate::{canbus::CanStorage, mcu_comm::CanBus};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WheelKey {
    VolUp,
    VolDown,
    Up,
    Down,
    Answer,
    Decline,
    PageUp,
    PageDown,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyEvent {
    KeyDown(WheelKey),
    KeyHeld(WheelKey),
    KeyRelease { key: WheelKey, time: u128 },
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum KombiPage {
    Audio,
    Nav,
    Tel,
    Other,
}

pub struct WheelKeyManager {
    page: Arc<AtomicU8>,
}

impl WheelKeyManager {
    pub fn new(can_db: CanStorage) -> Self {
        let page_ref = Arc::new(AtomicU8::new(0));
        let page_ref_c = page_ref.clone();
        let mut volume = 20;

        let mut vol_down = false;
        let mut vol_up = false;

        std::thread::spawn(move || {
            for x in 1..=3 {
                std::process::Command::new("pactl")
                    .args([
                        "set-sink-volume",
                        &format!("{}", x),
                        &format!("{}%", volume),
                    ])
                    .output();
            }
            let mut last_press_time = Instant::now();
            let mut last_key: Option<WheelKey> = None;
            let mut page = KombiPage::Other;
            loop {
                if let Some(parse) = can_db.get_frame(CanBus::B, 0x01CA, 50) {
                    last_press_time = Instant::now();
                    println!("FRAME!: {:02X?}", unsafe { parse.array });
                    page_ref_c.store(unsafe { parse.array[0] }, Ordering::Relaxed);
                    let v_down_now = (unsafe { parse.array[1] } & 0x04) != 0;
                    let v_up_now = (unsafe { parse.array[1] } & 0x08) != 0;
                    if !v_down_now && vol_down {
                        if volume > 0 {
                            volume -= 5;
                            for x in 1..=3 {
                                std::process::Command::new("pactl")
                                    .args([
                                        "set-sink-volume",
                                        &format!("{}", x),
                                        &format!("{}%", volume),
                                    ])
                                    .output();
                            }
                            println!("Volume down. Now at {}%", volume);
                        }
                    } else if !v_up_now && vol_up {
                        if volume < 50 {
                            volume += 5;
                            for x in 1..=3 {
                                std::process::Command::new("pactl")
                                    .args([
                                        "set-sink-volume",
                                        &format!("{}", x),
                                        &format!("{}%", volume),
                                    ])
                                    .output();
                            }
                            println!("Volume up. Now at {}%", volume);
                        }
                    }

                    vol_down = v_down_now;
                    vol_up = v_up_now;
                } else if last_press_time.elapsed().as_millis() > 1000 {
                    last_key = None;
                    vol_down = false;
                    vol_up = false;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        });
        Self { page: page_ref }
    }

    pub fn current_page(&self) -> KombiPage {
        match self.page.load(Ordering::Relaxed) {
            3 => KombiPage::Audio,
            4 => KombiPage::Nav,
            5 => KombiPage::Tel,
            _ => KombiPage::Other,
        }
    }
}
