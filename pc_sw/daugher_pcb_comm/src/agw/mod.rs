use std::{
    sync::{mpsc::{self, Sender}, atomic::Ordering},
    time::{Instant, Duration, SystemTime},
    vec,
};

use crate::{w211can::{CanBus}};
use bitflags::bitflags;
use chrono::{Datelike, Timelike};

use self::{bluetooth_manager::{BluetoothManager, BtCommand}, keys::WheelKeyManager, navigation::{NaviHeading, NaviPage, NaviPageCmd}};

mod bluetooth_manager;
mod keys;
mod pages;

use crate::agw::audio::{AudioPage, AudioPageCmd, AudioPageState, AudioSymbol};
use crate::agw::keys::KombiPage;
pub use pages::*;

static mut volume: i32 = 10000;
const MAX_VOLUME: i32 = 40000;

static mut sink_name: Option<String> = None;

fn get_volume() -> i32 {
    unsafe {volume}
}

fn get_sink() -> Option<String> {
    if let Some(name) = unsafe { &sink_name } {
        Some(name.clone())
    } else {
        if let Ok(s) = std::process::Command::new("pactl").args(["get-default-sink"]).output().map(|x| x.stdout) {
            let str = match String::from_utf8(s) {
                Ok(s) => { 
                    s[..s.len()-1].to_string()
                }
                Err(e) => return None
            };
            if str.contains("USB_Sound_Device") {
                unsafe { sink_name = Some(str.clone()) }
            }
        }
        unsafe { sink_name.clone() } 
    }
}

fn set_volume(d: i32) {
    if let Some(s_name) = get_sink() {
        let mut start = get_volume();
        let mut end = start + d;
        log::debug!("Setting volume from {} to {}. Sink is '{:?}'", start, end, s_name);
        if end < 0 {
            end = 0;
        } else if end > MAX_VOLUME {
            end = MAX_VOLUME;
        }
        if start == end {
            return;
        }
        unsafe { volume = end }
        std::process::Command::new("pactl")
            .args([
                "set-sink-volume",
                &format!("{}", s_name),
                &format!("{}", end),
            ])
            .output();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgwCommand {
    Wakeup,
    SetAudioPage(AudioPageState),
    SetAudioBodyText(IcText),
    SetAudioHeaderText(IcText),
    SetAudioSymbols(AudioSymbol, AudioSymbol),
    SetNaviCurrentRoad(String),
    SetNaviTargetRoad(String),
    SetNaviCompassHeading(NaviHeading),
}

/// Audio gateway emulator master
/// The houses the following functions:
/// * Audio page display
/// * Telephone page display
/// * Navigation page display
/// * Bluetooth communication with hands-free phone (Bluez)
/// * A generic dispatch system for other modules to send commands for AGW
///   to tweak the display of either Audio, Telephone or Navigation
/// * Wheel key input manager
pub struct AgwEmulator {
    /// Bluetooth manager
    bluetooth_handler: BluetoothManager,
    /// Wheel key (MRM) input layer
    key_manager: WheelKeyManager,
    sender: Sender<AgwCommand>
}


impl AgwEmulator {
    pub fn new() -> Self {
        // quickly set volume


        let mut endpoint = CanBus::B.create_iso_tp_socket(0x01A4, 0x01D0, 0, 0x28).unwrap();
        endpoint.set_nonblocking(true);
        let (sender, receiver) = mpsc::channel::<AgwCommand>();
        let (tx_isotp, rx_isotp) = mpsc::sync_channel::<Vec<u8>>(10);
        // Alert IC that AGW has woken up
        std::thread::spawn(move || {
            let audio_page = AudioPage::new();
            let (a_page, a_msg, a_ack, a_cmd) = AgwPageWrapper::new(tx_isotp.clone(), audio_page);
            let nav_page = NaviPage::new();
            let (n_page, n_msg, n_ack, n_cmd) = AgwPageWrapper::new(tx_isotp.clone(), nav_page);
            let mut last_time_send_time = Instant::now();
            let mut ic_awake = false;
            loop {
                /*
                if last_time_send_time.elapsed().as_millis() > 250 {
                    last_time_send_time = Instant::now();
                    let mut data = [0u8,0,0,0,0,0,0,0];
                    let time = chrono::Utc::now();
                    data[0] = (time.year() as u16 >> 8) as u8;
                    data[1] = (time.year() as u16 & 0xFF) as u8;
                    data[2] = time.month() as u8;
                    data[3] = time.day() as u8;
                    data[4] = time.hour() as u8;
                    data[5] = time.minute() as u8;
                    data[6] = ((time.second() * 100) as u16 >> 8) as u8;
                    data[7] = ((time.second() * 100) as u16 & 0xFF) as u8;
                    mcu.send_frame(PCCanFrame {
                        can_bus_tag: CanBus::B,
                        can_id: 0x339,
                        dlc: 8,
                        data,
                    });
                }
                */
                if let Ok(ic_pkg) = endpoint.read() {
                    if let Ok(page) = AgwPageId::try_from(ic_pkg[0]) {
                        let pkgid = ic_pkg[1];
                        // 3	5	4	F5
                        if pkgid == 0x04 && ic_pkg[2] == 0xF5 {
                            ic_awake = true;
                            // Special package. Ack
                            log::info!("IC HAS WOKEN UP!");
                            a_page.reset();
                            n_page.reset();
                            //a_msg.send(vec![0x20, 0x02, 0x11]);
                            //n_msg.send(vec![0x20, 0x02, 0x11]);
                        } else if ic_pkg.len() == 3 {
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
                if ic_awake {
                    endpoint.write(&[0x05, 0x04, 0x06]);
                    ic_awake = false;
                }
                if let Ok(to_send) = rx_isotp.try_recv() {
                    if endpoint.write(&to_send).is_ok() {
                        std::thread::sleep(std::time::Duration::from_millis(80))
                    }
                }
                if let Ok(cmd) = receiver.try_recv() {
                    match cmd {
                        AgwCommand::Wakeup => {
                        }
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
        let key_manager = WheelKeyManager::new();
        let key_manager_c = key_manager.clone();
        let mut t_plus = Instant::now();
        let mut t_minus = Instant::now();
        std::thread::spawn(move|| {
            let mut up = false;
            let mut down = false;
            let mut minus = false;
            let mut plus = false;
            let mut answer = false;
            let mut decline = false;
            let mut v_inc = 500;
            loop {
                let up_now = key_manager_c.up();
                let down_now = key_manager_c.down();
                let minus_now = key_manager_c.minus();
                let plus_now = key_manager_c.plus();
                let answer_now = key_manager_c.answer();
                let decline_now = key_manager_c.decline();
                let page = key_manager_c.current_page();
                if plus_now && !plus {
                    t_plus = Instant::now();
                }
                if minus_now && !minus {
                    t_minus = Instant::now();
                }

                if plus_now && plus && t_plus.elapsed().as_millis() > 500 {
                    set_volume(200);
                }
                if minus_now && minus && t_minus.elapsed().as_millis() > 500 {
                    set_volume(-200);
                } 
                if plus && !plus_now && t_plus.elapsed().as_millis() < 500 { // Key release
                    if (get_volume() > 15000) {
                        set_volume(v_inc);
                    } else {
                        set_volume(v_inc);
                    }
                    v_inc += 50;
                } else if minus && !minus_now  && t_minus.elapsed().as_millis() < 500 { // Key release
                    if (get_volume() > 15000) {
                        set_volume(-v_inc);
                    } else {
                        set_volume(-v_inc);
                    }
                    v_inc += 50;
                } else {
                    v_inc = 500;
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
            sender
        }
    }

    pub fn send_agw_command(&self, cmd: AgwCommand) {
        self.sender.send(cmd);
    }

    pub fn wakeup(&self) {
        self.sender.send(AgwCommand::Wakeup);
    }
}
