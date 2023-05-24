use socketcan::{CanSocket, Socket, CanDataFrame, EmbeddedFrame, StandardId, Id, CanFrame, CanFilter};
use socketcan_isotp::{IsoTpSocket, IsoTpBehaviour, IsoTpOptions, LinkLayerOptions, FlowControlOptions};

use socketcan_isotp::StandardId as IsoTpStandardId;
use socketcan_isotp::Id as IsoTpId;

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
        let df = CanDataFrame::new(
            Id::Standard(unsafe { StandardId::new_unchecked(id) }), 
            &data
        ).unwrap();
        let _ = self.0.write_frame(&CanFrame::Data(df));
    }

    pub fn with_socket<F: Fn(&mut CanSocket)>(&mut self, f: F) {
        f(&mut self.0)
    }

    pub fn read_frame(&self) -> Option<(u16, Vec<u8>)> {
        let f = self.0.read_frame().ok()?;
        let id = if let Id::Standard(id) = f.id() {
            id.as_raw()
        } else {
            0x7FF
        };
        Some((id, f.data().to_vec()))
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

    pub fn create_iso_tp_socket(&self, tx_id: u16, rx_id: u16, stmin: u8, bs: u8) -> Result<IsoTpSocket, socketcan_isotp::Error> {
        let opts: IsoTpOptions = IsoTpOptions::new(
            IsoTpBehaviour::CAN_ISOTP_TX_PADDING | IsoTpBehaviour::CAN_ISOTP_TX_PADDING,
            std::time::Duration::from_millis(0),
            0x00,
            0xCC,
            0xCC,
            0x00,
        )
        .unwrap();

        let link_opts: LinkLayerOptions = LinkLayerOptions::default();

        let (tx_id, rx_id) = (
            IsoTpId::Standard(unsafe { IsoTpStandardId::new_unchecked(tx_id) }),
            IsoTpId::Standard(unsafe { IsoTpStandardId::new_unchecked(rx_id) }),
        );

        let fc_opts = FlowControlOptions::new(bs, stmin, 0);

        let socket = socketcan_isotp::IsoTpSocket::open_with_opts(
            &self.get_network_name(),
            rx_id,
            tx_id,
            Some(opts),
            Some(fc_opts),
            Some(link_opts),
        )?;
        socket.set_nonblocking(true)?;
        Ok(socket)
    }

    pub fn create_can_socket(&self, filter_ids: &[u16]) -> Result<CanWrapper, socketcan::Error> {
        let channel = CanSocket::open(&self.get_network_name())?;
        channel.set_nonblocking(true)?;
        if filter_ids.is_empty() {
            channel.set_filter_accept_all()?;
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


fn frame_to_u64(f: &CanDataFrame) -> (u64, u8) {
    let mut v: u64 = 0;
    for x in 0..f.dlc() {
        v |= (f.data()[x] as u64) << 8*(7-x);
    }
    (v, f.dlc() as u8)
}

fn u64_to_frame(id: u16, v: u64, dlc: u8) -> CanDataFrame {
    let mut data = vec![0; dlc as usize];
    for x in 0..dlc as usize {
        data[x] = ((v >> (8*(7-x))) & 0xFF) as u8;
    }
    CanDataFrame::new(
        Id::Standard(unsafe { StandardId::new_unchecked(id) }), 
        &data
    ).unwrap()
}
