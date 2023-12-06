use bitflags::bitflags;
use std::{marker::PhantomData, sync::{mpsc, atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Init package
    fn build_pkg_20(&self, state: &T) -> Vec<u8>;
    /// Format and mode package
    fn build_pkg_24(&self, state: &T) -> Vec<u8>;
    /// Body package
    fn build_pkg_26(&self, state: &T) -> Vec<u8>;
    /// Symbol package
    fn build_pkg_28(&self, state: &T) -> Vec<u8>;
    /// Header package
    fn build_pkg_29(&self, state: &T) -> Vec<u8>;
    /// Action that can be called when page is idling (Not sending data to IC)
    fn on_page_idle(&mut self, state: &mut T) -> Option<Vec<u8>>;
    /// Event manager for the page
    fn on_event(&mut self, cmd: Cmd, state: T) -> (T, Option<Vec<u8>>);
    fn name(&self) -> &'static str;
    fn get_id(&self) -> u8;
}

/// Audio gateway page wrapper
pub struct AgwPageWrapper {
    reset_signal: Arc<AtomicBool>
}

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
        let s = sender.clone();

        let should_reset = Arc::new(AtomicBool::new(false));
        let should_reset_c = should_reset.clone();

        std::thread::spawn(move || {
            let mut page_state = T::default();
            let mut tx_tracker = PageTxData::new(&s);
            tx_tracker.send(pg.build_pkg_20(&page_state)); // Start the state machine
            println!("AGW Wrapper for page {} started", pg.name());
            let mut allowed_to_send = true;
            loop {

                if (should_reset_c.load(Ordering::Relaxed)) {
                    should_reset_c.store(false, Ordering::Relaxed);
                    tx_tracker.reset();
                    tx_tracker.send(pg.build_pkg_20(&page_state));
                }

                // IC has read our package, this is its response
                if let Ok((pkg_id, ack)) = rx_ack.try_recv() {
                    if ack == KombiAck::Ok { // processed OK
                        log::debug!("Kombi received OK! Page {}, pkg {:02X}", pg.name(), pkg_id);
                        tx_tracker.reset();
                        tx_tracker.notify_ok(pkg_id);
                    } else { // Process status unknown, 0x15 is failure.
                        log::debug!("Kombi received Error! Page {}, pkg {:02X}", pg.name(), pkg_id);
                        // Try to resend
                        if tx_tracker.get_try_counter() < 3 {
                            tx_tracker.resend();
                        } else {
                            // Too many resends. Give up
                            log::error!("Too many retries, giving up. Page {}, pkg {:02X}", pg.name(), pkg_id);
                            tx_tracker.reset();
                        }
                    }
                }
                // We are awaiting IC response
                if let KombiAck::Pending(i) = tx_tracker.get_send_state() {
                    // Timeout
                    if i.elapsed().as_millis() > 2000 {
                        log::error!("Kombi response timeout! Page {}", pg.name());
                        if tx_tracker.get_try_counter() < 3 {
                            tx_tracker.resend();
                        } else {
                            log::error!("Too many retries, giving up. Page {}", pg.name());
                            tx_tracker.reset();
                        }
                    }
                }
                // IC wants to send our page a message
                if let Ok(pkg) = rx_payload.try_recv() {
                    if !tx_tracker.is_idle() {
                        log::warn!("IC Failed to send ACK packet but got the next package?. Page {}", pg.name());
                        tx_tracker.reset();
                    }
                    // Acknowledge it
                    log::debug!("{:02X?} from kombi. Page {}", pkg, pg.name());
                    s.send(vec![pg.get_id(), pkg[0], 0x06]).unwrap();
                    // Wait 40ms to avoid sending the next payload before IC can process it
                    std::thread::sleep(Duration::from_millis(40));
                    // For now, we just check the package ID, and all the details
                    // are hard coded.
                    match pkg[0] {
                        // IC wants to reset the page (Something bad happened)
                        0x22 => tx_tracker.send(pg.build_pkg_20(&page_state)),
                        // IC wants us to send PKG 24
                        0x21 => tx_tracker.send(pg.build_pkg_24(&page_state)),
                        // IC has told us how it wants the body (26) package
                        0x25 => tx_tracker.send(pg.build_pkg_26(&page_state)),
                        // IC wants to tell the page a command
                        0x27 => {
                            // Cmd byte
                            match pkg[1] {
                                // 0x06 - Allow Tx of data
                                0x06 => { 
                                    allowed_to_send = true; 
                                    // Reinit page if we are allowed to send
                                    tx_tracker.send(pg.build_pkg_24(&page_state))
                                }
                                // 0x07 - Stop sending data. This is useful when NAVI page is
                                // trying to get exclusive access to IC when doing things like distance countdown
                                0x07 => allowed_to_send = false,
                                _ => {
                                    log::warn!("Unknown pkg 27 state for page {}. {:02X?}", pg.name(), &pkg[1..]);
                                }
                            }
                        }
                        _ => {
                            log::error!("Unknown  pkg {:02X}. Page {}. {:02X?}", pkg[0], pg.name(), &pkg[1..]);
                        }
                    }
                }
                if let Ok(cmd) = rx_cmd.try_recv() {
                    let (state, to_tx) = pg.on_event(cmd, page_state);
                    page_state = state;
                    if tx_tracker.init_done() {
                        if let Some(tx) = to_tx {
                            if allowed_to_send { tx_tracker.send(tx); }
                        }
                    }
                    if tx_tracker.is_idle() {
                        if let Some(tx) = pg.on_page_idle(&mut page_state) {
                            if allowed_to_send { tx_tracker.send(tx); }
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        });

        (Self { reset_signal: should_reset }, tx_payload, tx_ack, tx_cmd)
    }

    pub fn reset(&self) {
        self.reset_signal.store(true, Ordering::Relaxed);
    }
}
