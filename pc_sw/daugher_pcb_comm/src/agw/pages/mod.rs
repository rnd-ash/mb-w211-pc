use bitflags::bitflags;
use std::{marker::PhantomData, sync::mpsc, time::Duration};
use tokio::time::Instant;

pub mod audio;
pub mod navigation;
pub mod telephone;

pub fn clear_recv<T>(r: mpsc::Receiver<T>) {
    while r.try_recv().is_ok() {}
}

pub fn build_agw_packet_checksum_in_place(mut buf: Vec<u8>) -> Vec<u8> {
    let mut res: u8 = 0xFF;
    for x in 0..buf.len() {
        res = res.wrapping_sub(x as u8);
        res = res.wrapping_sub(buf[x]);
    }
    buf.push(res);
    buf
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IcText {
    pub format: TextFmtFlags,
    pub text: String,
}

bitflags! {
    pub struct TextFmtFlags: u8 {
        const NONE = 0x00;
        const LEFT = 0x01;
        const RIGHT = 0x08;
        const CENTER = 0x10;
        const FLASH = 0x20;
        const HIGHLIGHT = 0x40;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub enum AgwPageId {
    Audio = 0x03,
    Navigation = 0x04,
    Telephone = 0x05,
}

impl TryFrom<u8> for AgwPageId {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x03 => Ok(AgwPageId::Audio),
            0x04 => Ok(AgwPageId::Navigation),
            0x05 => Ok(AgwPageId::Telephone),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
#[repr(u8)]
pub enum KombiAck {
    None,             // Not sending
    Pending(Instant), // When we last sent it, awaiting Kombi response
    Ok = 0x06,        // Send state OK
    Err = 0x15,       // Send state was an error
}

impl TryFrom<u8> for KombiAck {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x06 => Ok(KombiAck::Ok),
            0x15 => Ok(KombiAck::Err),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct PageTxData<'a> {
    ack: KombiAck,
    last_sent: Vec<u8>,
    sent_counter: u32,
    sender: &'a mpsc::SyncSender<Vec<u8>>,
    init_completed: bool,
    last_sent_pkg: u8,
}

impl<'a> PageTxData<'a> {
    pub fn new(sender: &'a mpsc::SyncSender<Vec<u8>>) -> Self {
        Self {
            ack: KombiAck::None,
            last_sent: vec![],
            sent_counter: 0,
            sender,
            init_completed: false,
            last_sent_pkg: 0x20
        }
    }

    pub fn send(&mut self, data: Vec<u8>) {
        self.ack = KombiAck::Pending(Instant::now());
        self.last_sent = data.clone();
        if data[1] < 0x26 {
            self.init_completed = false;
        }
        self.last_sent_pkg = data[1];
        self.sender.send(data);
        self.sent_counter += 1;
    }

    pub fn resend(&mut self) {
        self.send(self.last_sent.clone());
    }

    pub fn reset(&mut self) {
        self.ack = KombiAck::None;
        self.last_sent = vec![];
        self.sent_counter = 0;
    }

    pub fn notify_ok(&mut self, pkg: u8) {
        if pkg >= 0x26 {
            self.init_completed = true;
        } else {
            self.init_completed = false;
        }
    }

    pub fn get_try_counter(&self) -> u32 {
        self.sent_counter
    }

    pub fn get_send_state(&self) -> KombiAck {
        self.ack.clone()
    }

    pub fn is_idle(&self) -> bool {
        return match self.ack {
            KombiAck::Ok | KombiAck::Err | KombiAck::None => true,
            _ => false,
        };
    }

    pub fn init_done(&self) -> bool {
        self.init_completed
    }
}

/// Since all 3 pages for AGW can be described as a finite state machine, we can actually define them
/// all as generics
pub trait AgwPageFsm<T, Cmd>
where
    T: Default + Send + Sync,
    Cmd: Send + Sync + 'static,
{
    fn build_pkg_20(&self, state: &T) -> Vec<u8>;
    fn build_pkg_24(&self, state: &T) -> Vec<u8>;
    fn build_pkg_26(&self, state: &T) -> Vec<u8>;
    fn build_pkg_28(&self, state: &T) -> Vec<u8>;
    fn build_pkg_29(&self, state: &T) -> Vec<u8>;
    fn on_page_idle(&mut self, state: &mut T, tracker: &mut PageTxData);
    fn on_event(&mut self, cmd: Cmd, state: T, tracker: &mut PageTxData) -> T;
    fn name(&self) -> &'static str;
}

pub struct AgwPageWrapper {}

impl AgwPageWrapper {
    pub fn new<T: Default + Send + Sync, Cmd: Send + Sync + 'static>(
        sender: mpsc::SyncSender<Vec<u8>>,
        mut pg: (impl AgwPageFsm<T, Cmd> + Send + Sync + 'static),
    ) -> (
        Self,
        mpsc::Sender<Vec<u8>>,
        mpsc::Sender<(u8, KombiAck)>,
        mpsc::Sender<Cmd>,
    ) {
        let (tx_payload, rx_payload) = mpsc::channel::<Vec<u8>>();
        let (tx_ack, rx_ack) = mpsc::channel::<(u8, KombiAck)>();
        let (tx_cmd, rx_cmd) = mpsc::channel::<Cmd>();
        std::thread::spawn(move || {
            let mut page_state = T::default();
            let mut tx_tracker = PageTxData::new(&sender);
            tx_tracker.send(pg.build_pkg_20(&page_state)); // Start the state machine
            println!("AGW Wrapper for page {} started", pg.name());
            loop {
                if let Ok((pkg_id, ack)) = rx_ack.try_recv() {
                    if ack == KombiAck::Ok {
                        log::debug!("Kombi received OK! Page {}, pkg {:02X}", pg.name(), pkg_id);
                        tx_tracker.reset();
                        tx_tracker.notify_ok(pkg_id);
                    } else {
                        log::debug!("Kombi received Error! Page {}, pkg {:02X}", pg.name(), pkg_id);
                        if tx_tracker.get_try_counter() < 3 {
                            tx_tracker.resend();
                        } else {
                            log::error!("Too many retries, giving up. Page {}, pkg {:02X}", pg.name(), pkg_id);
                            tx_tracker.reset();
                        }
                    }
                }
                if let KombiAck::Pending(i) = tx_tracker.get_send_state() {
                    if i.elapsed().as_millis() > 2000 {
                        log::error!("Kombi response timeout! Page {}", pg.name());
                        tx_tracker.reset();
                    }
                }
                if let Ok(pkg) = rx_payload.try_recv() {
                    if !tx_tracker.is_idle() {
                        log::warn!("IC Failed to send ACK packet but got the next package?. Page {}", pg.name());
                        tx_tracker.reset();
                    }
                    // Acknowledge it
                    log::debug!("{:02X?} from kombi. Page {}", pkg, pg.name());
                    sender.send(vec![0x03, pkg[0], 0x06]).unwrap();
                    std::thread::sleep(Duration::from_millis(40));
                    // Gen pkg 24
                    match pkg[0] {
                        0x22 => tx_tracker.send(pg.build_pkg_20(&page_state)),
                        0x21 => {
                            // RELOAD
                            tx_tracker.send(pg.build_pkg_24(&page_state))
                        }
                        0x25 => tx_tracker.send(pg.build_pkg_26(&page_state)),
                        _ => {
                            log::error!("Unknown  pkg {:02X}. Page {}", pkg[0], pg.name());
                        }
                    }
                }
                if tx_tracker.init_done() && tx_tracker.is_idle() {
                    if let Ok(cmd) = rx_cmd.try_recv() {
                        page_state = pg.on_event(cmd, page_state, &mut tx_tracker);
                    }
                }
                if tx_tracker.init_done() && tx_tracker.is_idle() {
                    pg.on_page_idle(&mut page_state, &mut tx_tracker);
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        });

        (Self {}, tx_payload, tx_ack, tx_cmd)
    }
}
