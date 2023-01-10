use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, RwLock, atomic::{AtomicU32, Ordering}}, cmp::{max, min}, time::{Duration, Instant}};

use crate::mcu_comm::{CanBus, PCCanFrame};

use super::CanFrameData;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SendState {
    None = 0,
    Sending = 1,
    Sent = 2,
    Timeout = 3,
}

#[derive(Clone, Debug)]
pub struct IsoTpEndpoint {
    tx_id: u16,
    rx_id: u16,
    bus: CanBus,
    // Padding is enabled by default (211 behavior)

    // From ECU to client
    isotp_rx: Arc<Receiver<Vec<u8>>>,
    // From client to ECU
    isotp_tx: Sender<Vec<u8>>,

    // From CAN to endpoint
    can_rx: Sender<Vec<u8>>,
    // From endpoint to CAN
    can_tx: Arc<Receiver<PCCanFrame>>,

    send_state: Arc<AtomicU32>

}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
struct IsoTpPayload {
    data: Vec<u8>,
    capacity: usize,
    pos: usize,
    cts: bool
}

unsafe impl Send for IsoTpEndpoint{}
unsafe impl Sync for IsoTpEndpoint{}

impl IsoTpEndpoint {
    pub fn new(tx_id: u16, rx_id: u16, bs: u8, stmin: u8, bus: CanBus) -> Self {

        let (isotp_tx_send, isotp_tx_recv) = mpsc::channel::<Vec<u8>>(); // Client -> Endpoint -> ECU
        let (isotp_rx_send, isotp_rx_recv) = mpsc::channel::<Vec<u8>>(); // ECU -> Endpoint -> Client

        let (can_tx_send, can_tx_recv) = mpsc::channel::<PCCanFrame>(); // Endpoint -> CAN
        let (can_rx_send, can_rx_recv) = mpsc::channel::<Vec<u8>>(); // CAN -> Endpoint

        let send_state = Arc::new(AtomicU32::new(SendState::None as u32));

        let send_state_t = send_state.clone();

        std::thread::spawn(move|| {
            let mut tx_isotp = IsoTpPayload::default(); // From client
            let mut rx_isotp = IsoTpPayload::default(); // From ECU
            let mut last_tx_time = Instant::now();
            let mut last_rx_time = Instant::now();
            let mut pci = 0x20;
            let mut tx_count = 0;
            let mut rx_count = 0;

            let mut ecu_bs: u8 = 0;
            let mut ecu_stmin: u8 = 0;
            loop {
                fn gen_pc_frame(id: u16, bus: CanBus, data: &[u8]) -> PCCanFrame {
                    let mut p = PCCanFrame { 
                        can_bus_tag: bus, 
                        can_id: id, 
                        dlc: 8, 
                        data: [0xCC; 8]
                    };
                    let max = min(8, data.len());
                    p.data[0..max].copy_from_slice(&data[0..max]);
                    p
                }

                // Check incoming frames
                if let Ok(in_frame) = can_rx_recv.try_recv() {
                    log::debug!("In CAN Frame {:02X?}", in_frame);
                    match in_frame[0] & 0xF0 {
                        0x00 => {
                            // Simple 1 frame Rx
                            if in_frame[0] <= 0x07 {
                                isotp_rx_send.send(in_frame[1..1+in_frame[0] as usize].to_vec());
                            }
                        },
                        0x10 => {
                            // Start of multi frame
                            if rx_isotp.capacity == 0 {
                                rx_isotp.capacity = ((in_frame[0] as usize) & 0x0F) << 4 | (in_frame[1] as usize);
                                rx_isotp.pos = 6;
                                rx_isotp.data = Vec::with_capacity(rx_isotp.capacity);
                                rx_isotp.data.extend_from_slice(&in_frame[2..]);
                                can_tx_send.send(gen_pc_frame(tx_id, bus, &[0x30, bs, stmin]));
                                last_rx_time = Instant::now();
                                rx_count = 0;
                            } else {
                                log::warn!("Trying to receive ISOTP when already receiving!?");
                                can_tx_send.send(gen_pc_frame(tx_id, bus, &[0x32, 0x00, 0x00]));
                            }
                        },
                        0x20 => {
                            if rx_isotp.capacity != 0 {
                                let max_read = min(7, rx_isotp.capacity-rx_isotp.pos);
                                rx_isotp.data.extend_from_slice(&in_frame[1..1+max_read]);
                                rx_isotp.pos += max_read;
                                rx_count+=1;
                                if rx_isotp.pos >= rx_isotp.capacity {
                                    // Done!
                                    isotp_rx_send.send(rx_isotp.data.clone());
                                    rx_isotp = Default::default();
                                }
                                if rx_count >= bs && bs != 0 {
                                    can_tx_send.send(gen_pc_frame(tx_id, bus, &[0x30, bs, stmin])); // Send another FC
                                    rx_count = 0;
                                }
                                last_rx_time = Instant::now();
                            }
                        },
                        0x30 => {
                            tx_isotp.cts = true;
                            last_tx_time = Instant::now();
                            tx_count = 0;
                            ecu_bs = in_frame[1];
                            ecu_stmin = in_frame[2];
                        }
                        _ => continue
                    }
                }

                // Now check if we should be sending
                if tx_isotp.capacity == 0 {
                    if let Ok(in_isotp) = isotp_tx_recv.try_recv() {
                        log::info!("Request to send payload {:02X?}", in_isotp);
                        tx_isotp.capacity = in_isotp.len();
                        tx_isotp.cts = false;
                        tx_isotp.data = in_isotp;
                        // Check to send first frame
                        if tx_isotp.capacity < 7 { // One frame
                            let mut data = vec![tx_isotp.capacity as u8];
                            data.extend_from_slice(&tx_isotp.data);
                            can_tx_send.send(gen_pc_frame(tx_id, bus, &data));
                            send_state_t.store(SendState::Sent as u32, Ordering::Relaxed);
                            tx_isotp.capacity = 0;
                        } else {
                            // Send start frame
                            let mut data = vec![0x10 | ((tx_isotp.capacity >> 8) as u8) & 0x0F, (tx_isotp.capacity & 0xFF) as u8];
                            data.extend_from_slice(&tx_isotp.data[..6]);
                            can_tx_send.send(gen_pc_frame(tx_id, bus, &data));
                            send_state_t.store(SendState::Sending as u32, Ordering::Relaxed);
                            tx_isotp.pos = 6;
                            tx_isotp.cts = false;
                            last_tx_time = Instant::now();
                            pci = 0x21;
                            tx_count = 0;
                        }
                    }
                }

                if tx_isotp.capacity != 0 && tx_isotp.cts && last_tx_time.elapsed().as_millis() >= stmin as u128 {
                    // Check if we need to send
                    let max_copy = min(7, tx_isotp.capacity-tx_isotp.pos);
                    let mut data = vec![pci];
                    data.extend_from_slice(&tx_isotp.data[tx_isotp.pos..tx_isotp.pos+max_copy]);
                    can_tx_send.send(gen_pc_frame(tx_id, bus, &data));
                    tx_isotp.pos += 7;
                    pci += 1;
                    if pci == 0x30 {
                        pci = 0x20;
                    }
                    tx_count+=1;
                    if tx_count > ecu_bs && ecu_bs != 0 {
                        tx_isotp.cts = false;
                    }
                    if tx_isotp.pos >= tx_isotp.capacity {
                        // Done sending
                        tx_isotp = Default::default();
                        send_state_t.store(SendState::Sent as u32, Ordering::Relaxed);
                    }
                    last_tx_time = Instant::now();
                }

                // Timeout check
                if tx_isotp.capacity != 0 && last_tx_time.elapsed().as_millis() > 2500 {
                    tx_isotp.capacity = 0;
                    log::error!("ISOTP Transmit timeout!");
                    send_state_t.store(SendState::Timeout as u32, Ordering::Relaxed);
                }
                if rx_isotp.capacity != 0 && last_rx_time.elapsed().as_millis() > 2500 {
                    rx_isotp.capacity = 0;
                    log::error!("ISOTP Receive timeout!");
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        });

        Self {
            tx_id,
            rx_id,
            bus,
            isotp_rx: Arc::new(isotp_rx_recv),
            isotp_tx: isotp_tx_send,
            can_rx: can_rx_send,
            can_tx: Arc::new(can_tx_recv),
            send_state
        }
    }

    pub fn poll_iso_tp_payload(&self) -> Option<Vec<u8>> {
        self.isotp_rx.try_recv().ok()
    }

    pub fn send_isotp_payload(&self, payload: Vec<u8>) {
        self.isotp_tx.send(payload);
    }

    pub fn send_isotp_payload_blocking(&self, payload: Vec<u8>) -> bool {
        let _ = self.get_send_status(); // Just to clear old state
        self.isotp_tx.send(payload);
        let now = Instant::now();
        let mut ret = false;
        loop {
            let state = self.get_send_status();
            if state == SendState::Timeout {
                log::error!("Blocking report: ISO-TP Send Timeout!");
                break;
            } else if state == SendState::Sent {
                log::info!("Blocking report: ISO-TP Send OK!");
                ret = true;
                break;
            }
            if now.elapsed().as_secs() > 10 {
                log::error!("Blocking report timeout!");
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        return ret;
    }

    pub fn get_send_status(&self) -> SendState {
        let s = match self.send_state.load(Ordering::Relaxed) {
            0 => SendState::None,
            1 => SendState::Sending,
            2 => SendState::Sent,
            3 => SendState::Timeout,
            _ => panic!("Illegal send state!")
        };

        if s == SendState::Sent || s == SendState::Timeout {
            self.send_state.store(SendState::None as u32, Ordering::Relaxed);
        }
        s
    }

    pub fn get_can_to_send(&self) -> Option<PCCanFrame> {
        self.can_tx.try_recv().ok()
    }

    pub fn on_can_read(&self, id: u16, data: &[u8]) {
        if id == self.rx_id {
            self.can_rx.send(data.to_vec());
        }
    }

    pub fn get_rx_id(&self) -> u16 {
        self.rx_id
    }

}