use socketcan::{CanSocket, Socket, CanDataFrame, EmbeddedFrame, StandardId, Id, CanFrame, CanFilter};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum CanBus {
    C = 67,
    B = 66,
    E = 69,
}

pub struct CanWrapper(CanSocket);

// Done for easier wrapping of Tx/Rx functions
impl CanWrapper {
    pub fn send_frame(&self, id: u16, data: &[u8]) {
        let mut df = CanDataFrame::new(
            Id::Standard(unsafe { StandardId::new_unchecked(id) }), 
            &data
        ).unwrap();
        self.0.write_frame(&CanFrame::Data(df));
    }

    pub fn with_socket<F: Fn(&mut CanSocket)>(&mut self, f: F) {
        f(&mut self.0)
    }

    pub fn read_frame(&self, id: u16) -> Option<Vec<u8>> {
        let f = self.0.read_frame().ok()?;
        Some(f.data().to_vec())
    }
}

impl CanBus {
    pub fn get_network_name(&self) -> &'static str {
        match self {
            CanBus::C => "vcan_c",
            CanBus::B => "vcan_b",
            CanBus::E => "vcan_e",
        }
    }

    pub fn create_can_socket(&self, filter_ids: &[u16]) -> Result<CanWrapper, socketcan::Error> {
        let channel = CanSocket::open(&self.get_network_name())?;
        channel.set_nonblocking(true)?;


        if filter_ids.is_empty() {
            channel.set_nonblocking(true);
        } else {

            let mut cf = Vec::new();
            for id in filter_ids {
                cf.push(CanFilter::new(*id as u32, 0xFFFF))
            }
            channel.set_filters(&cf)?;
        }

        Ok(CanWrapper(channel))
    }
}
