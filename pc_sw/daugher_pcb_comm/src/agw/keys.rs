use std::{
    sync::{
        atomic::{AtomicU8, Ordering, AtomicBool},
        Arc,
    },
    time::{Duration, Instant},
};

use crate::{w211can::CanBus};

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

#[derive(Debug, Clone)]
pub struct WheelKeyManager {
    page: Arc<AtomicU8>,
    up: Arc<AtomicBool>,
    down: Arc<AtomicBool>,
    plus: Arc<AtomicBool>,
    minus: Arc<AtomicBool>,
    answer: Arc<AtomicBool>,
    decline: Arc<AtomicBool>
}

impl WheelKeyManager {
    pub fn new() -> Self {
        let page_ref = Arc::new(AtomicU8::new(0));
        let page_ref_c = page_ref.clone();


        let up = Arc::new(AtomicBool::new(false));
        let down = Arc::new(AtomicBool::new(false));
        let plus = Arc::new(AtomicBool::new(false));
        let minus = Arc::new(AtomicBool::new(false));
        let answer = Arc::new(AtomicBool::new(false));
        let decline = Arc::new(AtomicBool::new(false));

        let mut up_c = up.clone();
        let mut down_c = down.clone();
        let mut plus_c = plus.clone();
        let mut minus_c = minus.clone();
        let mut answer_c = answer.clone();
        let mut decline_c = decline.clone();

        std::thread::spawn(move || {

            let can = CanBus::B.create_can_socket(&[0x01CA]).unwrap();

            let mut last_press_time = Instant::now();
            loop {
                if let Some(parse) = can.read_frame(0x01CA) {
                    last_press_time = Instant::now();
                    page_ref_c.store(parse[0], Ordering::Relaxed);
                    decline_c.store(parse[1] & 0x80 != 0, Ordering::Relaxed);
                    answer_c.store(parse[1] & 0x40 != 0, Ordering::Relaxed);
                    minus_c.store(parse[1] & 0x20 != 0, Ordering::Relaxed);
                    plus_c.store(parse[1] & 0x10 != 0, Ordering::Relaxed);
                    down_c.store(parse[1] & 0x02 != 0, Ordering::Relaxed);
                    up_c.store(parse[1] & 0x01 != 0, Ordering::Relaxed);
                } else if last_press_time.elapsed().as_millis() > 1000 {
                    last_press_time = Instant::now();
                    up_c.store(false, Ordering::Relaxed);
                    down_c.store(false, Ordering::Relaxed);
                    plus_c.store(false, Ordering::Relaxed);
                    minus_c.store(false, Ordering::Relaxed);
                    answer_c.store(false, Ordering::Relaxed);
                    decline_c.store(false, Ordering::Relaxed);
                }
                std::thread::sleep(Duration::from_millis(40));
            }
        });
        
        Self { 
            page: page_ref,
            up,
            down,
            plus,
            minus,
            answer,
            decline, 
            
        }
    }

    pub fn current_page(&self) -> KombiPage {
        match self.page.load(Ordering::Relaxed) {
            3 => KombiPage::Audio,
            4 => KombiPage::Nav,
            5 => KombiPage::Tel,
            _ => KombiPage::Other,
        }
    }

    pub fn up(&self) -> bool {
        self.up.load(Ordering::Relaxed)
    }

    pub fn down(&self) -> bool {
        self.down.load(Ordering::Relaxed)
    }

    pub fn plus(&self) -> bool {
        self.plus.load(Ordering::Relaxed)
    }

    pub fn minus(&self) -> bool {
        self.minus.load(Ordering::Relaxed)
    }

    pub fn answer(&self) -> bool {
        self.answer.load(Ordering::Relaxed)
    }

    pub fn decline(&self) -> bool {
        self.decline.load(Ordering::Relaxed)
    }

}
