use std::{
    sync::{mpsc::{self, Sender}, atomic::Ordering},
    time::{Instant, Duration},
    vec,
};

use crate::{
    canbus::{isotp::IsoTpEndpoint, CanStorage},
    mcu_comm::{CanBus, MCUComm},
};
use bitflags::bitflags;

use self::{bluetooth_manager::{BluetoothManager, BtCommand}, keys::WheelKeyManager, navigation::{NaviHeading, NaviPage, NaviPageCmd}};

mod bluetooth_manager;
mod keys;
mod pages;

use crate::agw::audio::{AudioPage, AudioPageCmd, AudioPageState, AudioSymbol};
use crate::agw::keys::KombiPage;
pub use pages::*;

static mut volume: i32 = 30;

fn get_volume() -> i32 {
    unsafe {volume}
}

fn set_volume(d: i32) {
    unsafe {volume += d };
    if get_volume() > 75 {
        unsafe {volume = 75 };
    } else if get_volume() < 0 {
        unsafe {volume = 0 };
    }
    log::debug!("Setting volume to {}%", get_volume());
    for sink in 1..=3 {
        std::process::Command::new("pactl")
            .args([
                "set-sink-volume",
                &format!("{}", sink),
                &format!("{}%", get_volume()),
            ])
            .output();
    }
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
            let audio_page = AudioPage::new();
            let (a_page, a_msg, a_ack, a_cmd) = AgwPageWrapper::new(tx_isotp.clone(), audio_page);
            let nav_page = NaviPage::new();
            let (n_page, n_msg, n_ack, n_cmd) = AgwPageWrapper::new(tx_isotp.clone(), nav_page);
            loop {
                if let Some(ic_pkg) = endpoint.poll_iso_tp_payload() {
                    if let Ok(page) = AgwPageId::try_from(ic_pkg[0]) {
                        let pkgid = ic_pkg[1];
                        if ic_pkg.len() == 3 {
                            if let Ok(status) = KombiAck::try_from(ic_pkg[2]) {
                                match page {
                                    AgwPageId::Audio => { a_ack.send((pkgid, status)); },
                                    AgwPageId::Navigation => { n_ack.send((pkgid, status)); },
                                    _ => {},
                                }
                            } else {
                                match page {
                                    AgwPageId::Audio => { a_msg.send(ic_pkg[1..].to_vec()); },
                                    AgwPageId::Navigation => { n_msg.send(ic_pkg[1..].to_vec()); },
                                    _ => {},
                                }
                            }
                        } else {
                            // It is a payload
                            match page {
                                AgwPageId::Audio => { a_msg.send(ic_pkg[1..].to_vec()); },
                                AgwPageId::Navigation => { n_msg.send(ic_pkg[1..].to_vec()); },
                                _ => {},
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
                        AgwCommand::SetNaviCurrentRoad(cr) => {
                            n_cmd.send(NaviPageCmd::CurrentRoad(cr));
                        }
                        AgwCommand::SetNaviTargetRoad(tr) => {
                            n_cmd.send(NaviPageCmd::CurrentRoad(tr));
                        }
                        AgwCommand::SetNaviCompassHeading(nch) => {
                            n_cmd.send(NaviPageCmd::CompassHeading(nch));
                        }
                        _ => {}
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
        let bt = BluetoothManager::new(sender.clone());
        let bt_c = bt.clone();
        let key_manager = WheelKeyManager::new(can_db);
        let key_manager_c = key_manager.clone();
        std::thread::spawn(move|| {
            let mut up = false;
            let mut down = false;
            let mut minus = false;
            let mut plus = false;
            let mut answer = false;
            let mut decline = false;
            unsafe { volume = 30 };
            set_volume(0);
            loop {
                let up_now = key_manager_c.up();
                let down_now = key_manager_c.down();
                let minus_now = key_manager_c.minus();
                let plus_now = key_manager_c.plus();
                let answer_now = key_manager_c.answer();
                let decline_now = key_manager_c.decline();
                let page = key_manager_c.current_page();

                if plus && !plus_now { // Key release
                    set_volume(5);
                } else if minus && !minus_now {
                    set_volume(-5);
                }
                if page == KombiPage::Audio {
                    if up && !up_now { // Key release
                        bt_c.send_media_control(BtCommand::Next);
                    } else if down && !down_now {
                        bt_c.send_media_control(BtCommand::Prev);
                    } 
                }

                // Set vars
                up = up_now;
                down = down_now;
                plus = plus_now;
                minus = minus_now;
                answer = answer_now;
                decline = decline_now;


                std::thread::sleep(std::time::Duration::from_millis(40));
            }
        });

        Self {
            bluetooth_handler: bt,
            key_manager,
            sender,
        }
    }

    pub fn send_agw_command(&self, cmd: AgwCommand) {
        self.sender.send(cmd);
    }
}
