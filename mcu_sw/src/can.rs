use core::cmp::min;

use atsamd_hal::{
    can::Dependencies,
    clock::v2::types::{Can0, Can1},
    dmac,
    sercom::uart::{self, UartFutureRxDuplexDma},
};
use mcan::{
    embedded_can::StandardId,
    generic_array::typenum::{U0, U1, U32, U64},
    message::{
        rx,
        tx::{self, MessageBuilder},
    },
    rx_fifo::{Fifo0, Fifo1, RxFifo},
};

use crate::bsp::{self, UartPads};

pub struct Capacities;

impl mcan::messageram::Capacities for Capacities {
    type StandardFilters = U1;
    type ExtendedFilters = U1;
    type RxBufferMessage = rx::Message<64>;
    type DedicatedRxBuffers = U0;
    type RxFifo0Message = rx::Message<64>;
    type RxFifo0 = U64;
    type RxFifo1Message = rx::Message<64>;
    type RxFifo1 = U64;
    type TxMessage = tx::Message<64>;
    type TxBuffers = U32;
    type DedicatedTxBuffers = U0;
    type TxEventFifo = U32;
}

pub type Can0RxFifo0 =
    RxFifo<'static, Fifo0, Can0, <Capacities as mcan::messageram::Capacities>::RxFifo0Message>;
pub type Can0RxFifo1 =
    RxFifo<'static, Fifo1, Can0, <Capacities as mcan::messageram::Capacities>::RxFifo1Message>;

pub type Can1RxFifo0 =
    RxFifo<'static, Fifo0, Can1, <Capacities as mcan::messageram::Capacities>::RxFifo0Message>;
pub type Can1RxFifo1 =
    RxFifo<'static, Fifo1, Can1, <Capacities as mcan::messageram::Capacities>::RxFifo1Message>;

pub type Can0Tx = mcan::tx_buffers::Tx<'static, Can0, Capacities>;
pub type Can1Tx = mcan::tx_buffers::Tx<'static, Can1, Capacities>;

pub type Can0TxEventFifo = mcan::tx_event_fifo::TxEventFifo<'static, Can0>;
pub type Can1TxEventFifo = mcan::tx_event_fifo::TxEventFifo<'static, Can1>;

pub type Can0Aux<GclkId> = mcan::bus::Aux<
    'static,
    Can0,
    Dependencies<Can0, GclkId, bsp::CANBRx, bsp::CANBTx, bsp::pac::Can0>,
>;

pub type Can1Aux<GclkId> = mcan::bus::Aux<
    'static,
    Can1,
    Dependencies<Can1, GclkId, bsp::CANCRx, bsp::CANCTx, bsp::pac::Can1>,
>;

pub fn frame_to_int(data: &[u8], dlc: u8) -> u64 {
    let mut ret = 0;
    for i in 0..dlc as usize {
        ret |= (data[i] as u64) << (8 * (7 - i));
    }
    ret
}

pub fn int_to_frame(data: &[u8], dlc: u8) -> u64 {
    let mut ret = 0;
    for i in 0..dlc as usize {
        ret |= (data[i] as u64) << (8 * (7 - i));
    }
    ret
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CanNet {
    B = 1,
    C = 2,
    E = 3,
}

const SERIAL_FRAME_LEN: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SerialCanFrame {
    pub net: u8,
    pub id: u16,
    pub dlc: u8,
    pub data: [u8; 8],
}

impl SerialCanFrame {
    pub fn new(net: CanNet, id: u16, data: &[u8]) -> Self {
        let dlc = min(8, data.len());
        let mut d = [0; 8];
        d[..dlc].copy_from_slice(&data[..dlc]);
        Self {
            net: net as u8,
            id,
            dlc: dlc as u8,
            data: d,
        }
    }

    pub fn to_bytes(&self, buf: &mut [u8; SERIAL_FRAME_LEN]) {
        // Stuff some bits
        // NET | DLC -> 1 byte
        buf[0] = 0xDE;
        buf[1] = 0xAD;
        buf[2] = 0xBE;
        buf[3] = 0xEF;
        buf[4] = (self.net as u8) << 4 | self.dlc;
        buf[5] = (self.id & 0xFF) as u8;
        buf[6] = ((self.id >> 8) & 0xFF) as u8;
        buf[7..7 + self.dlc as usize].copy_from_slice(&self.data[..self.dlc as usize]);
        // Last byte is CRC
        let mut res = 0u8;
        for i in 0..15u8 {
            res = res.wrapping_add(i).wrapping_add(buf[i as usize]);
        }
        buf[15] = res;
    }

    pub fn to_can_msg(&self) -> Option<mcan::message::tx::MessageBuilder> {
        Some(MessageBuilder {
            id: mcan::embedded_can::Id::Standard(StandardId::new(self.id)?),
            frame_type: tx::FrameType::Classic(tx::ClassicFrameType::Data(
                &self.data[..self.dlc as usize],
            )),
            store_tx_event: None,
        })
    }

    pub fn from_bytes(bytes: &[u8; SERIAL_FRAME_LEN]) -> Option<Self> {
        let targ_crc = bytes[15];
        let mut my_crc = 0u8;
        for i in 0..15u8 {
            my_crc = my_crc.wrapping_add(i).wrapping_add(bytes[i as usize]);
        }
        if my_crc != targ_crc {
            defmt::error!(
                "[Rx frame] CRC mismatch!. Recv: {:02X} Calc: {:02X}",
                targ_crc,
                my_crc
            );
            None
        } else {
            // CRC OK
            let id = bytes[5] as u16 | (bytes[6] as u16) << 8;
            let dlc = bytes[4] & 0x0F;
            let net = (bytes[4] & 0xF0) >> 4;
            if dlc == 0 || dlc > 8 {
                defmt::error!("[Rx frame] Invalid CAN DLC {}", dlc);
                None
            } else if id > 0x7FF {
                defmt::error!("[Rx frame] Invalid CAN ID: {:04X}", id);
                None
            } else if net == 0 || net > 3 {
                defmt::error!("[Rx frame] Invalid net ID: {}", net);
                None
            } else {
                // Valid!
                let mut data = [0u8; 8];
                data[..dlc as usize].copy_from_slice(&bytes[7..7 + dlc as usize]);
                Some(SerialCanFrame { net, id, dlc, data })
            }
        }
    }
}

pub async fn uart_read_frame(
    uart: &mut UartFutureRxDuplexDma<uart::Config<UartPads>, dmac::Ch0>,
) -> Option<SerialCanFrame> {
    let mut buf = [0u8; SERIAL_FRAME_LEN];
    // First, read the first 4 bytes (our signature)

    // We read the first byte alone so that the UART's RX buffer is rotated on
    // successive read failures until the magic byte is located and the rest
    // of the signature passes
    loop {
        uart.read(&mut buf[..1]).await.ok()?; // Byte 0 of sig
        if (buf[0]) == 0xDE {
            break; // Break when we maybe have our first magic byte located
        }
    }
    uart.read(&mut buf[1..4]).await.ok()?; // Byte 1-4 of the sig
    if &buf[..4] != &[0xDE, 0xAD, 0xBE, 0xEF] {
        // Full signature we expect
        // Invalid sig!
        return None;
    }
    // Read the remaining 12 bytes
    uart.read(&mut buf[4..]).await.ok()?;
    // Process and check CRC here, none is returned if CRC failed (Check defmt logs)
    SerialCanFrame::from_bytes(&buf)
}
