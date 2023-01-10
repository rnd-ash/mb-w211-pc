use std::{sync::mpsc::{Sender, self}, time::Instant, vec};

use crate::{canbus::isotp::IsoTpEndpoint, mcu_comm::{MCUComm, CanBus}};
use bitflags::bitflags;

use self::bluetooth_manager::BluetoothManager;

mod bluetooth_manager;
mod keys;

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
    Down = 0x0A
}

bitflags! {
    struct TextFmtFlags: u8 {
        const NONE = 0x00;
        const LEFT = 0x01;
        const RIGHT = 0x08;
        const CENTER = 0x10;
        const FLASH = 0x20;
        const HIGHLIGHT = 0x40;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IcText {
    format: TextFmtFlags,
    text: String
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
pub enum KombiTxPkg {
    None,
    Pkg20,
    Pkg24,
    Pkg26,
    Pkg28,
    Pkg29
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct NavPageState {
    current_road: String,
    next_road: String,
    meta: Vec<u8>
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AudioPageState {
    header_text: IcText,
    body_text: IcText,
    symbol_top: AudioSymbol,
    symbol_bottom: AudioSymbol
}

impl Default for AudioPageState {
    fn default() -> Self {
        Self { 
            header_text: IcText { 
                format: TextFmtFlags::LEFT, 
                text: "Audio".into() 
            }, 
            body_text: IcText { 
                format: TextFmtFlags::CENTER, 
                text: "Testing".into() 
            },  
            symbol_top: AudioSymbol::None, 
            symbol_bottom: AudioSymbol::None, 
        }
    }
}



pub fn build_agw_packet_checksum(buf: &[u8]) -> u8 {
    let mut res: u8 = 0xFF;
    for x in 0..buf.len() {
        res = res.wrapping_sub(x as u8);
        res = res.wrapping_sub(buf[x]);
    }
    return res
}

fn build_audio_pkg20() -> Vec<u8> {
    vec![0x03, 0x20, 0x02, 0x11, 0xC3]
}

fn build_audio_pkg24(header: IcText, icon_top: AudioSymbol, icon_bottom: AudioSymbol) -> Vec<u8> {
    let mut buf = vec![0x03, 0x24, 0x02, 0x60, 0x01, 0x01, 0x00, 0x00, 0x00, 0x13];
    buf.push(icon_top as u8);
    buf.push(0x01);
    buf.push(icon_bottom as u8);
    buf.push(0x02);
    buf.push(0x00);
    buf.push(2+header.text.len() as u8);
    buf.push(header.format.bits());
    buf.extend_from_slice(header.text.as_bytes());
    buf.push(0x00);
    let cs = build_agw_packet_checksum(&buf);
    buf.push(cs);
    buf
}

fn build_audio_pkg26(body: IcText) -> Vec<u8> {
    let mut buf = vec![0x03, 0x26, 0x01, 0x00, 0x01];
    buf.push(2+body.text.len() as u8);
    buf.push(body.format.bits());
    buf.extend_from_slice(body.text.as_bytes());
    buf.push(0x00);
    let cs = build_agw_packet_checksum(&buf);
    buf.push(cs);
    buf
}

fn build_audio_pkg28(icon_top: AudioSymbol, icon_bottom: AudioSymbol) -> Vec<u8> {
    let mut buf = vec![0x03, 0x28, 0x02];
    buf.push(icon_top as u8);
    buf.push(0x01);
    buf.push(icon_bottom as u8);
    buf.push(0x02);
    let cs = build_agw_packet_checksum(&buf);
    buf.push(cs);
    buf
}

fn build_audio_pkg29(header: IcText) -> Vec<u8> {
    let mut buf = vec![0x03, 0x29];
    buf.push(header.text.len() as u8);
    buf.push(header.format.bits());
    buf.extend_from_slice(header.text.as_bytes());
    buf.push(0x00);
    let cs = build_agw_packet_checksum(&buf);
    buf.push(cs);
    buf
}

fn build_navi_pkg20() -> Vec<u8> {
    return vec![0x04, 0x20, 0x02, 0x11, 0xC2]
}

fn build_navi_pkg24() -> Vec<u8> {
    return vec![0x04, 0x24, 0x03, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8E]
}

fn build_navi_pkg26(state: &NavPageState) -> Vec<u8> {
    let mut buf = vec![0x04, 0x26, 0x01, 0x00, 0x03];
    // First string
    buf.push(3+state.next_road.len() as u8);
    buf.push(0x10); // Todo format
    buf.extend_from_slice(state.next_road.as_bytes());
    buf.push(0x00);
    // Second string
    buf.push(3+state.current_road.len() as u8);
    buf.push(0x10); // Todo format
    buf.extend_from_slice(state.current_road.as_bytes());
    buf.push(0x00);
    // Symbol data
    buf.push(2+state.meta.len() as u8);
    buf.push(0x80);
    buf.extend_from_slice(&state.meta);
    buf.push(0x00);
    let cs = build_agw_packet_checksum(&buf);
    buf.push(cs);
    buf
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AgwPageState {
    curr_pkg: KombiTxPkg,
    ack_state: bool,
    send_time: Instant
}

impl Default for AgwPageState {
    fn default() -> Self {
        Self { curr_pkg: KombiTxPkg::Pkg20, ack_state: false, send_time: Instant::now() }
    }
}

#[inline(always)]
fn send_pkg(endpoint: &IsoTpEndpoint, state: &mut AgwPageState, pkg: KombiTxPkg, data: Vec<u8>) {
    endpoint.send_isotp_payload(data);
    state.ack_state = false;
    state.curr_pkg = pkg;
    state.send_time = Instant::now();
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NaviHeading {
    S,
    SE,
    SW,
    N,
    NE,
    NW,
    W,
    E
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgwCommand {
    SetAudioBodyText(IcText),
    SetAudioHeaderText(IcText),
    SetAudioSymbols(AudioSymbol, AudioSymbol),
    SetNaviCurrentRoad(String),
    SetNaviTargetRoad(String),
    SetNaviCompassHeading(NaviHeading)
}

pub struct AgwEmulator {
    bluetooth_handler: Option<BluetoothManager>,
    sender: Sender<AgwCommand>
}

impl AgwEmulator {
    pub fn new(mcu: &mut MCUComm) -> Self {
        let endpoint = IsoTpEndpoint::new(0x01A4, 0x01D0, 0, 0x28, CanBus::B);
        mcu.register_endpoint(&endpoint);
        let (sender, receiver) = mpsc::channel::<AgwCommand>();
        std::thread::spawn(move||{
            let mut audio_state = AudioPageState::default();
            log::info!("AGW Emulator thread starting");
            let mut init = false;

            let mut nav_state = NavPageState {
                current_road: "This road".into(),
                next_road: "~J1That road".into(),
                //meta: vec![0x14, 0x00, 0x00, 0x00, 0x00, 0x01, 0xA0, 0x26], // SE
                meta:   vec![0x17, 0x00, 0x00, 0x63, 0xCD, 0x01, 0xC0, 0x03],
                //meta: vec![0x17, 0x00, 0x00, 0x64, 0x4B, 0x04, 0x80, 0x01, 0x40, 0x01, 0xF0, 0x01, 0xB0, 0x0F], // 25yds, 4th exit on roundabout
            };

            endpoint.send_isotp_payload_blocking(build_navi_pkg20());
            endpoint.send_isotp_payload_blocking(build_audio_pkg20());


            let mut audio_tracker = AgwPageState::default();
            let mut navi_tracker = AgwPageState::default();
            loop {
                if let Some(kombi_msg) = endpoint.poll_iso_tp_payload() {
                    let page = kombi_msg[0];
                    let pkg = kombi_msg[1];
                    if page == 0x03 {
                        // Ack PKG
                        // <PAGE> <PKG> <STATE>
                        if kombi_msg.len() == 3 {
                            if kombi_msg[2] == 0x06 {
                                log::warn!("AUDIO KOMBI response OK. package {:02X}!", pkg);
                                audio_tracker.ack_state = true;
                            } else if kombi_msg[2] == 0x15 {
                                log::warn!("AUDIO KOMBI response ERR. package {:02X}!", pkg);
                            } else {
                                log::warn!("AUDIO Unknown kombi ACK flag {:02X}", kombi_msg[2]);
                            }
                            if pkg == 0x26 {
                                audio_tracker.curr_pkg = KombiTxPkg::None;
                            }
                        } else {
                            // We need to ack it
                            endpoint.send_isotp_payload_blocking(vec![page, pkg, 0x06]);
                            if pkg == 0x21 {
                                // Send pkg 24!
                                send_pkg(
                                    &endpoint, 
                                    &mut audio_tracker, 
                                    KombiTxPkg::Pkg24, 
                                    build_audio_pkg24(audio_state.header_text.clone(), audio_state.symbol_top, audio_state.symbol_bottom)
                                );
                            } else if pkg == 0x25 {
                                send_pkg(
                                    &endpoint, 
                                    &mut audio_tracker, 
                                    KombiTxPkg::Pkg26, 
                                    build_audio_pkg26(audio_state.body_text.clone())
                                );
                            }
                        }
                    } else if page == 0x04 {
                        if kombi_msg.len() == 3 {
                            if kombi_msg[2] == 0x06 {
                                log::warn!("NAVI KOMBI response OK. package {:02X}!", pkg);
                                navi_tracker.ack_state = true;
                            } else if kombi_msg[2] == 0x15 {
                                log::warn!("NAVI KOMBI response ERR. package {:02X}!", pkg);
                            } else {
                                log::warn!("NAVI Unknown kombi ACK flag {:02X}", kombi_msg[2]);
                            }
                            if pkg == 0x26 {
                                navi_tracker.curr_pkg = KombiTxPkg::None;
                            }
                        } else {
                            // We need to ack it
                            endpoint.send_isotp_payload_blocking(vec![page, pkg, 0x06]);
                            if pkg == 0x21 {
                                // Send pkg 24!
                                endpoint.send_isotp_payload_blocking(build_navi_pkg24());
                                navi_tracker.curr_pkg = KombiTxPkg::Pkg24;
                                navi_tracker.ack_state  = false;
                                navi_tracker.send_time = Instant::now();
                            } else if pkg == 0x25 {
                                // Send pkg 26!
                                endpoint.send_isotp_payload_blocking(build_navi_pkg26(&nav_state));
                                navi_tracker.curr_pkg = KombiTxPkg::Pkg26;
                                navi_tracker.ack_state = false;
                                navi_tracker.send_time = Instant::now();
                            }
                        }
                    } else {
                        log::warn!("Ignoring kombi package {:02X} for page {:02X}", pkg, page);
                    }
                }

                if audio_tracker.curr_pkg != KombiTxPkg::None {
                    if !audio_tracker.ack_state && audio_tracker.send_time.elapsed().as_millis() > 5000 {
                        log::warn!("AGW Timeout, sending package {:?} for AUDIO", audio_tracker.curr_pkg);
                        audio_tracker.ack_state = false;
                        audio_tracker.send_time = Instant::now();
                        match audio_tracker.curr_pkg {
                            KombiTxPkg::Pkg20 => endpoint.send_isotp_payload_blocking(build_audio_pkg20()),
                            KombiTxPkg::Pkg24 => endpoint.send_isotp_payload_blocking(build_audio_pkg24(audio_state.header_text.clone(), audio_state.symbol_top, audio_state.symbol_bottom)),
                            KombiTxPkg::Pkg26 => endpoint.send_isotp_payload_blocking(build_audio_pkg26(audio_state.body_text.clone())),
                            KombiTxPkg::Pkg28 => endpoint.send_isotp_payload_blocking(build_audio_pkg28(audio_state.symbol_top, audio_state.symbol_bottom)),
                            KombiTxPkg::Pkg29 => endpoint.send_isotp_payload_blocking(build_audio_pkg29(audio_state.header_text.clone())),
                            _ => false
                        };
                    }
                }
                if navi_tracker.curr_pkg != KombiTxPkg::None {
                    if !navi_tracker.ack_state && navi_tracker.send_time.elapsed().as_millis() > 5000 {
                        log::warn!("AGW Timeout, sending package {:?} for NAVI", navi_tracker.curr_pkg);
                        navi_tracker.ack_state = false;
                        navi_tracker.send_time = Instant::now();
                        match navi_tracker.curr_pkg {
                            KombiTxPkg::Pkg20 => endpoint.send_isotp_payload_blocking(build_navi_pkg20()),
                            KombiTxPkg::Pkg24 => endpoint.send_isotp_payload_blocking(build_navi_pkg24()),
                            KombiTxPkg::Pkg26 => endpoint.send_isotp_payload_blocking(build_navi_pkg26(&nav_state)),
                            _ => false
                        };
                    }
                }
                if let Ok(command) = receiver.try_recv() {
                    match command {
                        AgwCommand::SetAudioBodyText(body) => {
                            audio_state.body_text = body;
                            if audio_tracker.curr_pkg == KombiTxPkg::None {
                                send_pkg(
                                    &endpoint, 
                                    &mut audio_tracker, 
                                    KombiTxPkg::Pkg26, 
                                    build_audio_pkg26(audio_state.body_text.clone())
                                );
                            }
                        },
                        AgwCommand::SetAudioHeaderText(header) => {
                            audio_state.header_text = header;
                            if audio_tracker.curr_pkg == KombiTxPkg::None {
                                send_pkg(
                                    &endpoint, 
                                    &mut audio_tracker, 
                                    KombiTxPkg::Pkg29, 
                                    build_audio_pkg29(audio_state.header_text.clone())
                                );
                            }
                        },
                        AgwCommand::SetAudioSymbols(top, bottom) => {
                            audio_state.symbol_top = top;
                            audio_state.symbol_bottom = bottom;
                            if audio_tracker.curr_pkg == KombiTxPkg::Pkg28 {
                                send_pkg(
                                    &endpoint, 
                                    &mut audio_tracker, 
                                    KombiTxPkg::Pkg28, 
                                    build_audio_pkg28(top, bottom)
                                );
                            }
                        },
                        AgwCommand::SetNaviCurrentRoad(rd) => {
                            nav_state.current_road = rd;
                        },
                        AgwCommand::SetNaviTargetRoad(rd) => {
                            nav_state.next_road = rd;
                        },
                        AgwCommand::SetNaviCompassHeading(hd) => {
                            nav_state.current_road = "HEADING".into()
                        },
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        let bt = BluetoothManager::new();
        Self {
            bluetooth_handler: Some(bt),
            sender
        }
    }

    pub fn set_audio_text(&self) {

    }


}