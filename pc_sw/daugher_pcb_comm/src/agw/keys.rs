use std::sync::{
        atomic::{AtomicU8, Ordering},
        Arc
    };


use futures_util::StreamExt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use w211_can::{canb::{KOMBI_A5, MRM_A2}, canbus::CanBus, tokio_socketcan::CANFilter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum W213WheelEvent {
    TouchPadX(u8),
    TouchPadY(u8),
    Key(W213WheelKey),
    Idle
}

impl From<[u8; 2]> for W213WheelEvent {
    fn from(value: [u8; 2]) -> Self {
        if value[0] == 0x10 && value[1] != 0x00 {
            match W213WheelKey::try_from(value[1]) {
                Ok(key) => Self::Key(key),
                Err(_) => Self::Idle,
            }
        } else if value[0] == 0x70 {
            Self::TouchPadX(value[1])
        } else if value[0] == 0x30 {
            Self::TouchPadY(value[1])
        } else {
            match value[0] {
                0x10 => Self::Key(W213WheelKey::VolUp),
                0x20 => Self::Key(W213WheelKey::VolDown),
                0x40 => Self::Key(W213WheelKey::Answer),
                0x80 => Self::Key(W213WheelKey::Decline),
                0x04 => Self::Key(W213WheelKey::RightSwipe),
                0x08 => Self::Key(W213WheelKey::LeftSwipe),
                0x01 => Self::Key(W213WheelKey::UpSwipe),
                0x02 => Self::Key(W213WheelKey::DownSwipe),
                _ => Self::Idle
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum W213WheelKey {
    Back,
    Home,
    TouchPad,
    Answer,
    Decline,
    Mute,
    UnMute, // For AGW only
    Speak,
    Star,
    ICHome,
    ICBack,
    ICTouchPad,
    DistronicDIS,
    DistronicLIM,
    DistronicMinus(u8),
    DistronicPlus(u8),
    DistronicRes,
    DistronicCancel,
    VolUp,
    VolDown,
    LeftSwipe,
    RightSwipe,
    UpSwipe,
    DownSwipe,
}

impl TryFrom<u8> for W213WheelKey {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::ICBack),
            0x02 => Ok(Self::ICHome),
            0x03 => Ok(Self::Back),
            0x04 => Ok(Self::Home),
            0x05 => Ok(Self::Speak),
            0x06 => Ok(Self::Star),
            0x07 => Ok(Self::Mute),
            0x08 => Ok(Self::ICTouchPad),
            0x09 => Ok(Self::TouchPad),
            0x10 => Ok(Self::DistronicCancel),
            0x11 => Ok(Self::DistronicDIS),
            0x12 => Ok(Self::DistronicRes),
            0x13 => Ok(Self::DistronicLIM),
            0x14 => Ok(Self::DistronicMinus(1)),
            0x15 => Ok(Self::DistronicPlus(1)),
            0x16 => Ok(Self::DistronicMinus(2)),
            0x17 => Ok(Self::DistronicPlus(2)),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum KombiPage {
    Audio,
    Nav,
    Tel,
    Other,
}

#[derive(Debug)]
pub struct WheelKeyManager {
    page: Arc<AtomicU8>,
    key_press: UnboundedReceiver<W213WheelKey>,
}

pub fn move_mouse(pos: u8, is_x: bool) {
    let pos = (pos as i32) - 0x7E;
    if pos == 0 {
        return;
    }
    let a = if pos > 0 {pos} else {pos*-1};
    for _ in 0..a {
        let s = (pos/a)*2;
        let (xmove, ymove) = if is_x {(s, 0)} else {(0,s)};
        std::process::Command::new("xdotool")
        .args([
            "mousemove_relative",
            "--",
            &format!("{xmove}"),
            &format!("{ymove}")
        ]).output();
    }
}

pub fn click_mouse() {
    std::process::Command::new("xdotool")
        .args([
            "click",
            "1"
        ]).output();
}

impl WheelKeyManager {
    pub fn new(can_name: String) -> (Self, UnboundedSender<W213WheelKey>) {
        let page_ref = Arc::new(AtomicU8::new(0));
        let page_ref_c = page_ref.clone();
        let (tx, rx) = unbounded_channel::<W213WheelKey>();
        let tx_c = tx.clone();
        tokio::spawn(async move {
            let mut can = CanBus::create_can_socket_with_name(&can_name).unwrap();
            let filters = [
                CANFilter::new(0x01CA, 0xFFF).unwrap(),
                CANFilter::new(0x01A8, 0xFFF).unwrap()
            ];
            can.set_filter(&filters).unwrap();
            let mut last_evt = W213WheelEvent::Idle;
            let mut prev_evt = W213WheelEvent::Idle;
            loop {
                if let Some(Ok(frame)) = can.next().await {
                    let data = frame.data();
                    if frame.id() == KOMBI_A5::get_canid() as u32 {
                        if data != [0x00, 0x00, 0x00, 0x00] {
                            page_ref_c.store(data[0], Ordering::Relaxed);
                        }
                        // Volume works differently, we don't care if its the same
                        // since volume works as a scroll wheel
                        if last_evt != prev_evt {
                            prev_evt = last_evt;
                            match last_evt {
                                W213WheelEvent::TouchPadX(x) => {
                                    move_mouse(x, true);
                                },
                                W213WheelEvent::TouchPadY(y) => {
                                    move_mouse(y, false);
                                },
                                W213WheelEvent::Key(k) if k == W213WheelKey::TouchPad => {
                                    click_mouse();
                                },
                                W213WheelEvent::Key(k) => {
                                    let _ = tx.send(k);
                                }
                                W213WheelEvent::Idle => (),
                            }
                        } else {
                            // If duplicate data
                            match last_evt {
                                W213WheelEvent::Key(k) => {
                                    if k == W213WheelKey::VolUp || k == W213WheelKey::VolDown {
                                        let _ = tx.send(k);
                                    }
                                },
                                W213WheelEvent::TouchPadX(x) => {
                                    move_mouse(x, true);
                                },
                                W213WheelEvent::TouchPadY(y) => {
                                    move_mouse(y, false);
                                },
                                _ => {}
                            }
                        }
                    } else if frame.id() == MRM_A2::get_canid() as u32 {
                        last_evt = W213WheelEvent::from([data[0], data[1]]);
                    } else {
                        println!("UNKNOWN Wheel frame! {:02X?}", frame)
                    }
                }
            }
        });
        
        (Self { 
            page: page_ref,
            key_press: rx,
        }, tx_c)
    }

    pub fn current_page(&self) -> KombiPage {
        match self.page.load(Ordering::Relaxed) {
            3 => KombiPage::Audio,
            4 => KombiPage::Nav,
            5 => KombiPage::Tel,
            _ => KombiPage::Other,
        }
    }

    pub async fn event(&mut self) -> Option<W213WheelKey> {
        self.key_press.recv().await
    }
}
