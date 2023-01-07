use std::sync::mpsc::{Sender, self};

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
                text: "BT Play..".into() 
            }, 
            body_text: IcText { 
                format: TextFmtFlags::CENTER, 
                text: "Sour by GALXARA".into() 
            },  
            symbol_top: AudioSymbol::NextTrack, 
            symbol_bottom: AudioSymbol::PrevTrack, 
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

fn handle_ic_package(msg: &[u8], sender: &IsoTpEndpoint, a_page_state: &AudioPageState) {
    log::info!("Incomming KOMMI MSG: {:02X?}", msg);
    let page = msg[0];
    let pkg = msg[1];
    if page != 0x03 { 
        log::warn!("KOMBI wants use to respond for page {:02X?}, ignoring for now", page);
    }
    if *msg.last().unwrap() != 0x06 { // Its a package we have to acknowledge
        if page == 0x03 { // Audio page
            sender.send_isotp_payload(vec![page, pkg, 0x06]); // Acknowledge first
            std::thread::sleep(std::time::Duration::from_millis(20)); // Avoid spamming
            if pkg == 0x21 {
                // Send pkg 24!
                sender.send_isotp_payload(build_audio_pkg24(a_page_state.header_text.clone(), a_page_state.symbol_top, a_page_state.symbol_bottom));
            } else if pkg == 0x25 {
                // Send pkg 26!
                sender.send_isotp_payload(build_audio_pkg26(a_page_state.body_text.clone()));
            }
        } else {
            log::warn!("Unknown handle for page {:02X} (PKG {:02X})", page, pkg);
        }
    } else { // ACK from KOMBI
        log::info!("KOMBI Acknowledged package {:02X} for page {:02X}", pkg, page);
    }
    
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgwCommand {
    SetAudioBodyText(IcText),
    SetAudioHeaderText(IcText),
    SetAudioSymbols(AudioSymbol, AudioSymbol)
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
            loop {
                if let Some(kombi_msg) = endpoint.poll_iso_tp_payload() {
                    handle_ic_package(&kombi_msg, &endpoint, &audio_state);
                }
                if !init {
                    endpoint.send_isotp_payload(vec![0x03, 0x20, 0x02, 0x11, 0xC3]);
                    init = true;
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