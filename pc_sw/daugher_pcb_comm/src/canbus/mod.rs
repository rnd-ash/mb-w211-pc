use std::{collections::HashMap, sync::{RwLock, Arc}};
pub mod isotp;

use crate::mcu_comm::CanBus;



#[derive(Clone, Copy)]
#[repr(C)]
pub union CanFrameData {
    data: u64,
    array: [u8; 8]
}

#[derive(Default, Clone)]
pub struct CanStorage {
    pub canb: Arc<RwLock<HashMap<u16, CanFrameData>>>,
    pub canc: Arc<RwLock<HashMap<u16, CanFrameData>>>,
    pub cane: Arc<RwLock<HashMap<u16, CanFrameData>>>,
}

impl CanStorage {
    pub fn add_frame(&mut self, can: CanBus, id: u16, data: &[u8]) {
        let mut x = data.to_vec();
        x.resize(8, 0x00);
        let d = u64::from_be_bytes(x.try_into().unwrap());
        match can {
            CanBus::C => self.canc.write().unwrap().insert(id, CanFrameData { data: d }),
            CanBus::B => self.canb.write().unwrap().insert(id, CanFrameData { data: d }),
            CanBus::E => self.cane.write().unwrap().insert(id, CanFrameData { data: d }),
        };
    }

    pub fn get_frame(&self, can: CanBus, id: u16) -> Option<CanFrameData> {
        let map = match can {
            CanBus::C => self.canc.read().unwrap(),
            CanBus::B => self.canb.read().unwrap(),
            CanBus::E => self.cane.read().unwrap(),
        };
        return map.get(&id).cloned()
    }
}