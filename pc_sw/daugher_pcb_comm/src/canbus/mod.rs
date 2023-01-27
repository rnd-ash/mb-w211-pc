use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};
pub mod isotp;

use crate::mcu_comm::CanBus;

#[derive(Clone, Copy)]
#[repr(C)]
pub union CanFrameData {
    pub data: u64,
    pub array: [u8; 8],
}

#[derive(Default, Clone)]
pub struct CanStorage {
    pub canb: Arc<RwLock<HashMap<u16, (usize, CanFrameData)>>>,
    pub canc: Arc<RwLock<HashMap<u16, (usize, CanFrameData)>>>,
    pub cane: Arc<RwLock<HashMap<u16, (usize, CanFrameData)>>>,
    ts: Arc<AtomicUsize>,
}

unsafe impl Send for CanStorage {}
unsafe impl Sync for CanStorage {}

impl CanStorage {
    pub fn get_counter(&self) -> Arc<AtomicUsize> {
        return self.ts.clone();
    }

    pub fn add_frame(&mut self, can: CanBus, id: u16, data: &[u8], ts: usize) {
        if ts > self.ts.load(Ordering::Relaxed) {
            self.ts.store(ts, Ordering::Relaxed);
        }
        if can == CanBus::Loopback {
            return;
        }
        let mut x = data.to_vec();
        x.resize(8, 0x00);
        let d = u64::from_le_bytes(x.try_into().unwrap());
        match can {
            CanBus::C => self
                .canc
                .write()
                .unwrap()
                .insert(id, (ts, CanFrameData { data: d })),
            CanBus::B => self
                .canb
                .write()
                .unwrap()
                .insert(id, (ts, CanFrameData { data: d })),
            CanBus::E => self
                .cane
                .write()
                .unwrap()
                .insert(id, (ts, CanFrameData { data: d })),
            _ => return,
        };
    }

    pub fn get_frame(&self, can: CanBus, id: u16, timeout: usize) -> Option<CanFrameData> {
        if can == CanBus::Loopback {
            return None;
        }
        let map = match can {
            CanBus::C => self.canc.read().unwrap(),
            CanBus::B => self.canb.read().unwrap(),
            CanBus::E => self.cane.read().unwrap(),
            _ => return None,
        };
        let v = map.get(&id)?;
        if self.ts.load(Ordering::Relaxed) - v.0 > timeout {
            return None;
        } else {
            return Some(v.1);
        }
    }
}
