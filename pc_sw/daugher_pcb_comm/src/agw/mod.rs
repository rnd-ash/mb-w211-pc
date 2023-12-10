use std::{
    sync::{mpsc::{self, Sender}, RwLock, Arc},
    time::Instant, thread::JoinHandle, borrow::BorrowMut,
};

use self::{bluetooth_manager::{BluetoothManager, BtCommand}, navigation::{NaviPage, NaviPageCmd}, audio_control::AudioManager, keys::{WheelKeyManager, W213WheelKey}};

mod bluetooth_manager;
mod keys;
mod pages;
mod audio_control;

use crate::{agw::audio::{AudioPage, AudioPageCmd, AudioPageState, AudioSymbol}, custom_display_format::CDMIsoTp};
use crate::agw::keys::KombiPage;
pub use pages::*;

pub mod char_map;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgwCommand {
    Wakeup,
    TrackUpdate(String),
    SetAudioPage(AudioPageState),
    SetAudioBodyText(IcText),
    SetAudioHeaderText(IcText),
    SetAudioSymbols(AudioSymbol, AudioSymbol),
    SendNaviData(NaviPageCmd)
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
    _bluetooth_handler: BluetoothManager,
    /// Wheel key (MRM) input layer
    //key_manager: WheelKeyManager,
    sender: Sender<AgwCommand>,
}


impl AgwEmulator {
    pub fn new(can_name: String, mut vlad: CDMIsoTp) -> Self {
        let mut endpoint = w211_can::canbus::CanBus::create_isotp_socket_with_name(&can_name, 0x1D0, 0x1A4, 40, 0);
        let _ = endpoint.set_nonblocking(true);
        let (sender, receiver) = mpsc::channel::<AgwCommand>();
        let (tx_isotp, rx_isotp) = mpsc::sync_channel::<Vec<u8>>(10);
        let current_page = Arc::new(RwLock::new(KombiPage::Other));
        let current_page_c = current_page.clone();
        // Alert IC that AGW has woken up
        std::thread::spawn(move || {
            let audio_page = AudioPage::new();
            let (a_page, a_msg, a_ack, a_cmd) = AgwPageWrapper::new(tx_isotp.clone(), audio_page);
            let nav_page = NaviPage::new();
            let (n_page, n_msg, n_ack, n_cmd) = AgwPageWrapper::new(tx_isotp.clone(), nav_page);
            let _last_time_send_time = Instant::now();
            let mut ic_awake = false;
            loop {
                vlad.update();
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
                                let _ = match page {
                                    AgwPageId::Audio => { a_ack.send((pkgid, status)) },
                                    AgwPageId::Navigation => { n_ack.send((pkgid, status)) },
                                    _ => Ok(()),
                                };
                            } else {
                                let _ = match page {
                                    AgwPageId::Audio => { a_msg.send(ic_pkg[1..].to_vec()) },
                                    AgwPageId::Navigation => { n_msg.send(ic_pkg[1..].to_vec()) },
                                    _ => Ok(()),
                                };
                            }
                        } else {
                            // It is a payload
                            let _ = match page {
                                AgwPageId::Audio => { a_msg.send(ic_pkg[1..].to_vec()) },
                                AgwPageId::Navigation => { n_msg.send(ic_pkg[1..].to_vec()) },
                                _ => Ok(()),
                            };
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
                    let _ = endpoint.write(&[0x05, 0x04, 0x06]);
                    ic_awake = false;
                }
                if let Ok(to_send) = rx_isotp.try_recv() {
                    if endpoint.write(&to_send).is_ok() {
                        std::thread::sleep(std::time::Duration::from_millis(40))
                    }
                }
                if let Ok(cmd) = receiver.try_recv() {
                    let _ = match cmd {
                        AgwCommand::Wakeup => {
                        }
                        AgwCommand::SetAudioPage(p) => {
                            let _ = a_cmd.send(AudioPageCmd::SetPage(p));
                        }
                        AgwCommand::SetAudioBodyText(t) => {
                            let _ = a_cmd.send(AudioPageCmd::SetBody(t));
                        }
                        AgwCommand::SetAudioHeaderText(t) => {
                            let _ = a_cmd.send(AudioPageCmd::SetHeader(t));
                        }
                        AgwCommand::SetAudioSymbols(u, d) => {
                            let _ = a_cmd.send(AudioPageCmd::SetIcons(u, d));
                        }
                        AgwCommand::SendNaviData(cr) => {
                            let _ = n_cmd.send(cr);
                        }
                        AgwCommand::TrackUpdate(name) => {
                            if *current_page.read().unwrap() != KombiPage::Audio {
                                vlad.notify_track_change(&name);
                            }
                        }
                    };
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
        let bt = BluetoothManager::new(sender.clone());
        let bt_c = bt.clone();
        let key_manager = WheelKeyManager::new(can_name.clone());
        //let key_manager_c = key_manager.clone();
        let _t_plus = Instant::now();
        let _t_minus = Instant::now();


        let mixer_data : Arc<RwLock<(f32,f32,f32,f32)>> = Arc::new(RwLock::new((1.0, 1.0, 1.0, 1.0)));
        let _mixer_data_c = mixer_data.clone();

        std::thread::spawn(move|| {
            let mut mgr = AudioManager::new();
            let _v_inc = 500;
            let mut muted = false;
            loop {
                let page = key_manager.current_page();
                *current_page_c.write().unwrap() = page;
                if let Some(key) = key_manager.event() {
                    match key {
                        W213WheelKey::VolUp => {
                            muted = false;
                            mgr.offset_volume(0.005);
                        },
                        W213WheelKey::VolDown => {
                            muted = false;
                            mgr.offset_volume(-0.005);
                        },
                        W213WheelKey::Mute => {
                            muted = !muted;
                            mgr.set_mute(muted);
                        },
                        W213WheelKey::UpSwipe => {
                            if page == KombiPage::Audio {
                                bt_c.send_media_control(BtCommand::Next);
                            }
                        },
                        W213WheelKey::DownSwipe => {
                            if page == KombiPage::Audio {
                                bt_c.send_media_control(BtCommand::Prev);
                            }
                        },
                        W213WheelKey::DistronicPlus(1) => {
                            bt_c.send_media_control(BtCommand::Next);
                        },
                        W213WheelKey::DistronicMinus(1) => {
                            bt_c.send_media_control(BtCommand::Prev);
                        },
                        _ => {}
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        Self {
            _bluetooth_handler: bt,
            //key_manager,
            sender
        }
    }

    pub fn send_agw_command(&self, cmd: AgwCommand) {
        let _ = self.sender.send(cmd);
    }

    pub fn wakeup(&self) {
        let _ = self.sender.send(AgwCommand::Wakeup);
    }
}
