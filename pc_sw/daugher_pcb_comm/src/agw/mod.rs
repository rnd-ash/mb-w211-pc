use std::{
    sync::mpsc::{self, Sender},
    time::{Instant, Duration},
    vec,
};

use crate::{
    canbus::{isotp::IsoTpEndpoint, CanStorage},
    mcu_comm::{CanBus, MCUComm},
};
use bitflags::bitflags;

use self::{bluetooth_manager::BluetoothManager, keys::WheelKeyManager};

mod bluetooth_manager;
mod keys;
mod pages;

use crate::agw::audio::{AudioPage, AudioPageCmd, AudioPageState, AudioSymbol};
use crate::agw::keys::KombiPage;
pub use pages::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct NavPageState {
    current_road: String,
    next_road: String,
    meta: Vec<u8>,
}

fn build_navi_pkg20() -> Vec<u8> {
    return vec![0x04, 0x20, 0x02, 0x11, 0xC2];
}

fn build_navi_pkg24() -> Vec<u8> {
    return vec![
        0x04, 0x24, 0x03, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8E,
    ];
}

fn build_navi_pkg26(state: &NavPageState) -> Vec<u8> {
    let mut buf = vec![0x04, 0x26, 0x01, 0x00, 0x03];
    // First string
    buf.push(3 + state.next_road.len() as u8);
    buf.push(0x10); // Todo format
    buf.extend_from_slice(state.next_road.as_bytes());
    buf.push(0x00);
    // Second string
    buf.push(3 + state.current_road.len() as u8);
    buf.push(0x10); // Todo format
    buf.extend_from_slice(state.current_road.as_bytes());
    buf.push(0x00);
    // Symbol data
    buf.push(2 + state.meta.len() as u8);
    buf.push(0x80);
    buf.extend_from_slice(&state.meta);
    buf.push(0x00);
    //let cs = build_agw_packet_checksum(&buf);
    //buf.push(cs);
    buf
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
    E,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgwCommand {
    SetAudioPage(AudioPageState),
    SetAudioBodyText(IcText),
    SetAudioHeaderText(IcText),
    SetAudioSymbols(AudioSymbol, AudioSymbol),
    SetNaviCurrentRoad(String),
    SetNaviTargetRoad(String),
    SetNaviCompassHeading(NaviHeading),
}

pub struct AgwEmulator {
    bluetooth_handler: BluetoothManager,
    key_manager: WheelKeyManager,
    sender: Sender<AgwCommand>,
}

impl AgwEmulator {
    pub fn new(mcu: &mut MCUComm, can_db: CanStorage) -> Self {
        let endpoint = IsoTpEndpoint::new(0x01A4, 0x01D0, 0, 0x28, CanBus::B);
        mcu.register_endpoint(&endpoint);
        let (sender, receiver) = mpsc::channel::<AgwCommand>();
        let (tx_isotp, rx_isotp) = mpsc::sync_channel::<Vec<u8>>(10);
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(500));
            let audio_page = AudioPage::new();
            let (a_page, a_msg, a_ack, a_cmd) = AgwPageWrapper::new(tx_isotp.clone(), audio_page);
            loop {
                if let Some(ic_pkg) = endpoint.poll_iso_tp_payload() {
                    if let Ok(page) = AgwPageId::try_from(ic_pkg[0]) {
                        let pkgid = ic_pkg[1];
                        if ic_pkg.len() == 3 {
                            if let Ok(status) = KombiAck::try_from(ic_pkg[2]) {
                                log::debug!(
                                    "Response {:?} for pkg 0x{:02X?} page {:?}",
                                    status,
                                    pkgid,
                                    page
                                );
                                if page == AgwPageId::Audio {
                                    a_ack.send((pkgid, status));
                                }
                            } else {
                                log::debug!("Payload {:02X?} for page {:?}", &ic_pkg[1..], page);
                                if page == AgwPageId::Audio {
                                    a_msg.send(ic_pkg[1..].to_vec());
                                }
                            }
                        } else {
                            // It is a payload
                            log::debug!("Payload {:02X?} for page {:?}", &ic_pkg[1..], page);
                            if page == AgwPageId::Audio {
                                a_msg.send(ic_pkg[1..].to_vec());
                            }
                        }
                    } else {
                        log::error!(
                            "Unknown page 0x{:02X}!. Payload was {:02X?}",
                            ic_pkg[0],
                            ic_pkg
                        )
                    }
                }
                if let Ok(to_send) = rx_isotp.try_recv() {
                    endpoint.send_isotp_payload_blocking(to_send);
                }
                if let Ok(cmd) = receiver.try_recv() {
                    match cmd {
                        AgwCommand::SetAudioPage(p) => {
                            a_cmd.send(AudioPageCmd::SetPage(p));
                        }
                        AgwCommand::SetAudioBodyText(t) => {
                            a_cmd.send(AudioPageCmd::SetBody(t));
                        }
                        AgwCommand::SetAudioHeaderText(t) => {
                            a_cmd.send(AudioPageCmd::SetHeader(t));
                        }
                        AgwCommand::SetAudioSymbols(u, d) => {
                            a_cmd.send(AudioPageCmd::SetIcons(u, d));
                        }
                        AgwCommand::SetNaviCurrentRoad(_) => {}
                        AgwCommand::SetNaviTargetRoad(_) => {}
                        AgwCommand::SetNaviCompassHeading(_) => {}
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        let bt = BluetoothManager::new(sender.clone());
        Self {
            bluetooth_handler: bt,
            key_manager: WheelKeyManager::new(can_db),
            sender,
        }
    }

    pub fn send_agw_command(&self, cmd: AgwCommand) {
        self.sender.send(cmd);
    }
}
