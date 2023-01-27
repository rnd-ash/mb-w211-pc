use crate::agw::{build_agw_packet_checksum_in_place, IcText, PageTxData, TextFmtFlags};
use futures::StreamExt;
use std::borrow::BorrowMut;
use std::slice::Chunks;
use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc, RwLock};
use tokio::count;
use tokio::time::Instant;

use super::{AgwPageFsm, KombiAck};

pub struct AudioPage {
    last_rotate_time: Instant,
    rotating_body: Vec<String>,
    rotating_idx: usize,
}

impl AudioPage {
    pub fn new() -> Self {
        Self {
            last_rotate_time: Instant::now(),
            rotating_body: Vec::new(),
            rotating_idx: 0
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AudioPageState {
    pub header_text: IcText,
    pub body_text: IcText,
    pub symbol_top: AudioSymbol,
    pub symbol_bottom: AudioSymbol,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AudioSymbol {
    None = 0x00,
    NextTrack = 0x01,
    PrevTrack = 0x02,
    FastFwd = 0x03,
    FastRev = 0x04,
    Play = 0x05,
    Rewind = 0x06,
    Up = 0x09,
    Down = 0x0A,
}

impl Default for AudioPageState {
    fn default() -> Self {
        Self {
            header_text: IcText {
                format: TextFmtFlags::LEFT,
                text: "Audio".to_string(),
            },
            body_text: IcText {
                format: TextFmtFlags::CENTER,
                text: "Starting...".to_string(),
            },
            symbol_top: AudioSymbol::None,
            symbol_bottom: AudioSymbol::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AudioPageCmd {
    SetPage(AudioPageState),
    SetBody(IcText),
    SetHeader(IcText),
    SetIcons(AudioSymbol, AudioSymbol),
}

impl AgwPageFsm<AudioPageState, AudioPageCmd> for AudioPage {
    fn name(&self) -> &'static str {
        "AUDIO"
    }

    fn build_pkg_20(&self, state: &AudioPageState) -> Vec<u8> {
        vec![0x03, 0x20, 0x02, 0x11, 0xC3]
    }

    fn build_pkg_24(&self, state: &AudioPageState) -> Vec<u8> {
        let mut buf = vec![0x03, 0x24, 0x02, 0x60, 0x01, 0x01, 0x00, 0x00, 0x00, 0x13];
        buf.push(state.symbol_top as u8);
        buf.push(0x01);
        buf.push(state.symbol_bottom as u8);
        buf.push(0x02);
        buf.push(0x00);
        buf.push(2 + state.header_text.text.len() as u8);
        buf.push(state.header_text.format.bits());
        buf.extend_from_slice(state.header_text.text.clone().as_bytes());
        buf.push(0x00);
        build_agw_packet_checksum_in_place(buf)
    }

    fn build_pkg_26(&self, state: &AudioPageState) -> Vec<u8> {
        let mut buf = vec![0x03, 0x26, 0x01, 0x00, 0x01];
        buf.push(2 + state.body_text.text.len() as u8);
        buf.push(state.body_text.format.bits());
        buf.extend_from_slice(state.body_text.text.clone().as_bytes());
        buf.push(0x00);
        build_agw_packet_checksum_in_place(buf)
    }

    fn build_pkg_28(&self, state: &AudioPageState) -> Vec<u8> {
        let mut buf = vec![0x03, 0x28, 0x02];
        buf.push(state.symbol_top as u8);
        buf.push(0x01);
        buf.push(state.symbol_bottom as u8);
        buf.push(0x02);
        build_agw_packet_checksum_in_place(buf)
    }

    fn build_pkg_29(&self, state: &AudioPageState) -> Vec<u8> {
        let mut buf = vec![0x03, 0x29];
        buf.push(2 + state.header_text.text.len() as u8);
        buf.push(state.header_text.format.bits());
        buf.extend_from_slice(state.header_text.text.clone().as_bytes());
        buf.push(0x00);
        build_agw_packet_checksum_in_place(buf)
    }

    fn on_page_idle(&mut self, state: &mut AudioPageState) -> Option<Vec<u8>> {
        if self.rotating_body.len() != 0 && self.last_rotate_time.elapsed().as_millis() > 2000 {
            self.last_rotate_time = Instant::now();

            let mut tmp = state.clone();
            tmp.body_text.text = "~G1".into();
            if self.rotating_idx != 0 {
                tmp.body_text.text.push_str("...");
            }
            tmp.body_text.text.push_str(&self.rotating_body[self.rotating_idx].clone());
            if self.rotating_idx != self.rotating_body.len()-1 {
                tmp.body_text.text.push_str("...");
            }
            tmp.body_text.format = TextFmtFlags::CENTER;
            self.rotating_idx += 1;
            if self.rotating_idx >= self.rotating_body.len() {
                self.rotating_idx = 0;
            }
            Some(self.build_pkg_26(&tmp))
        } else {
            None
        }
    }

    fn on_event(
        &mut self,
        cmd: AudioPageCmd,
        state: AudioPageState
    ) -> (AudioPageState, Option<Vec<u8>>) {
        let mut state = state;
        let mut to_tx = None;
        match cmd {
            AudioPageCmd::SetPage(p) => {
                if p != state {
                    state = p.clone();
                    self.last_rotate_time = Instant::now();
                    if state.body_text.text.len() > 15 {
                        self.rotating_body = state
                            .body_text
                            .text
                            .split(" ")
                            .map(|x| x.to_string())
                            .collect();
                        
                        // Joiner algo
                        let mut res: Vec<String> = Vec::new();
                        let mut x = 0;
                        let mut tmp = String::new();
                        while x < self.rotating_body.len() {
                            let len = tmp.len() + self.rotating_body[x].len() + 1;
                            if len > 15 {
                                res.push(tmp);
                                tmp = self.rotating_body[x].clone();
                            } else {
                                tmp.push_str(self.rotating_body[x].as_str());
                                tmp.push_str(" ");
                            }
                            x += 1;
                        }
                        res.push(tmp);
                        self.rotating_body = res;
                        self.rotating_idx = 0;
                    } else {
                        self.rotating_body.clear();
                        to_tx = Some(self.build_pkg_24(&state));
                    }
                }
            }
            AudioPageCmd::SetBody(b) => {
                if b != state.body_text {
                    state.body_text = b.clone();
                    self.last_rotate_time = Instant::now();
                    if state.body_text.text.len() > 15 {
                        self.rotating_body = state
                            .body_text
                            .text
                            .split(" ")
                            .map(|x| x.to_string())
                            .collect();
                        let mut res: Vec<String> = Vec::new();
                        let mut x = 0;
                        let mut tmp = String::new();
                        while x < self.rotating_body.len() {
                            let len = tmp.len() + self.rotating_body[x].len() + 1;
                            if len > 15 {
                                if tmp.ends_with(" ") {
                                    tmp = tmp.clone().strip_suffix(" ").unwrap_or(&tmp).to_string()
                                }
                                res.push(tmp);
                                tmp = self.rotating_body[x].clone();
                            } else {
                                tmp.push_str(self.rotating_body[x].as_str());
                                tmp.push_str(" ");
                            }
                            x += 1;
                        }
                        res.push(tmp);
                        self.rotating_body = res;
                        self.rotating_idx = 0;
                    } else {
                        self.rotating_body.clear();
                        to_tx = Some(self.build_pkg_26(&state));
                    }
                }
            }
            AudioPageCmd::SetHeader(h) => {
                if h != state.header_text {
                    state.header_text = h;
                    to_tx = Some(self.build_pkg_29(&state))
                }
            }
            AudioPageCmd::SetIcons(u, d) => {
                if u != state.symbol_top && d != state.symbol_bottom {
                    state.symbol_top = u;
                    state.symbol_bottom = d;
                    to_tx = Some(self.build_pkg_28(&state))
                }
            }
        }
        (state, to_tx)
    }

    fn get_id(&self) -> u8 {
        0x03
    }
}
