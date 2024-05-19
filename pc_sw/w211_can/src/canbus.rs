use std::time::Duration;

use tokio_socketcan::{CANFrame, CANSocket};
use tokio_socketcan_isotp::{FlowControlOptions, Id, IsoTpBehaviour, IsoTpOptions, IsoTpSocket, StandardId};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum CanBus {
    B = 0b01,
    C = 0b10,
    E = 0b11,
}

impl CanBus {
    pub fn get_net_name(&self) -> &'static str {
        match self {
            CanBus::C => "vcan_c",
            CanBus::B => "vcan_b",
            CanBus::E => "vcan_e",
        }
    }

    pub fn create_can_socket(&self) -> Result<CANSocket, tokio_socketcan::Error> {
        Self::create_can_socket_with_name(self.get_net_name())
    }

    pub fn create_can_socket_with_name(name: &str) -> Result<CANSocket, tokio_socketcan::Error> {
        CANSocket::open(name)
    }

    pub fn create_isotp_socket(&self, rx: u16, tx: u16, stmin: u8, bs: u8) -> IsoTpSocket {
        Self::create_isotp_socket_with_name(self.get_net_name(), rx, tx, stmin, bs)
    }

    pub fn create_isotp_socket_with_name(name: &str, rx: u16, tx: u16, stmin: u8, bs: u8) -> IsoTpSocket {
        // We are in a known CAN format (W211), so can shortcuts can be made
        let fc_opts = FlowControlOptions::new(bs, stmin, 0);
        let behaviour = IsoTpBehaviour::CAN_ISOTP_RX_PADDING | IsoTpBehaviour::CAN_ISOTP_TX_PADDING;
        let isotp_opts = IsoTpOptions::new(
            behaviour, 
            Duration::from_millis(0), 
            0, 
            0xCC, 
            0xCC, 
            0
        ).unwrap();
        
        IsoTpSocket::open_with_opts(
            name, 
            Id::Standard(StandardId::new(rx).unwrap()), 
            Id::Standard(StandardId::new(tx).unwrap()), 
            Some(isotp_opts), 
            Some(fc_opts), 
            None
        ).unwrap()
    }
}

pub fn frame_to_u64(f: &CANFrame) -> (u64, u8) {
    let mut v: u64 = 0;
    for (x, item) in f.data().iter().enumerate() {
        v |= (*item as u64) << (8*(7-x));
    }
    (v, f.data().len() as u8)
}

pub fn u64_to_frame(id: u16, v: u64, dlc: u8) -> CANFrame {
    let mut data = vec![0; 8];
    for (x, item) in data.iter_mut().enumerate().take(dlc as usize) {
        *item = ((v >> (8*(7-x))) & 0xFF) as u8;
    }
    CANFrame::new(id as u32, &data[0..dlc as usize], false, false).unwrap()
}
