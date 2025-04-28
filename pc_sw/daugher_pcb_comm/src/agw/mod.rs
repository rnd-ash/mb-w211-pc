use std::{sync::{Arc, RwLock}, time::Instant};

use self::{bluetooth_manager::{BluetoothManager, BtCommand}, navigation::{NaviPage, NaviPageCmd}, audio_control::AudioManager, keys::{WheelKeyManager, W213WheelKey}, audio::AudioCfgSettings};

mod bluetooth_manager;
mod keys;
mod pages;
mod audio_control;

use crate::{agw::audio::{AudioPage, AudioPageCmd, AudioPageState, AudioSymbol}, custom_display_format::{CDMIsoTp, ToneType, ToneRepeatType}};
use crate::agw::keys::KombiPage;
pub use pages::*;
use tokio::{runtime::{Handle, Runtime}, sync::mpsc::UnboundedSender};

pub mod char_map;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgwCommand {
    Wakeup,
    TrackUpdate(String),
    SetAudioPage(AudioPageState),
    SetAudioBodyText(IcText),
    SetAudioHeaderText(IcText),
    SetAudioSymbols(AudioSymbol, AudioSymbol),
    SendNaviData(NaviPageCmd),
    ShowCustomDisplay(String, u32),
    HideCustomDisplay,
    StopBuzzer,
    SoundBuzze(ToneType, ToneRepeatType)
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
    sender_keys: UnboundedSender<W213WheelKey>,
    sender: UnboundedSender<AgwCommand>,
}


impl AgwEmulator {
    pub fn new(rt: &Runtime, can_name: String, vlad: CDMIsoTp, audio_settings: AudioCfgSettings) -> Self {
        let endpoint = w211_can::canbus::CanBus::create_isotp_socket_with_name(&can_name, 0x1D0, 0x1A4, 50, 0);
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<AgwCommand>();
        let (tx_isotp, mut rx_isotp) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
        let current_page = Arc::new(RwLock::new(KombiPage::Other));
        let current_page_c = current_page.clone();
        // Alert IC that AGW has woken up
        rt.spawn(async move {
            let handle = Handle::current();
            let audio_page = AudioPage::new(audio_settings);
            let (a_page, a_msg, a_ack, a_cmd) = AgwPageWrapper::new(tx_isotp.clone(), &handle, audio_page);
            let nav_page = NaviPage::new();
            let (n_page, n_msg, n_ack, n_cmd) = AgwPageWrapper::new(tx_isotp.clone(), &handle, nav_page);
            let _last_time_send_time = Instant::now();
            let mut ic_awake = false;
            loop {

                tokio::select! {
                    Ok(ic_pkg) = endpoint.read_packet().unwrap() => {
                        if let Ok(page) = AgwPageId::try_from(ic_pkg[0]) {
                            let pkgid = ic_pkg[1];
                            // 3	5	4	F5
                            if pkgid == 0x04 && ic_pkg[2] == 0xF5 {
                                ic_awake = true;
                                // Special package. Ack
                                log::info!("IC HAS WOKEN UP!");
                                a_page.reset();
                                n_page.reset();
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
                    },
                    Some(to_send) = rx_isotp.recv() => {
                        let _ = endpoint.write_packet(&to_send).unwrap().await;
                    },
                    Some(cmd) = receiver.recv() => {
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
                            AgwCommand::ShowCustomDisplay(text, duration_ms) => {
                                vlad.show_display(text, duration_ms)
                            },
                            AgwCommand::HideCustomDisplay => {
                                vlad.stop_display()
                            },
                            AgwCommand::StopBuzzer => {
                                vlad.stop_buzzer()
                            },
                            AgwCommand::SoundBuzze(tone, repeat) => {
                                vlad.sound_buzzer(tone, repeat)
                            },
                        };
                    }
                }
                if ic_awake {
                    let _ = endpoint.write_packet(&[0x05, 0x04, 0x06]).unwrap().await;
                    ic_awake = false;
                }
            }
        });
        let bt = BluetoothManager::new(sender.clone(), rt.handle());
        let bt_c = bt.clone();
        let (mut key_manager, ext_event_sender) = WheelKeyManager::new(can_name.clone());
        //let key_manager_c = key_manager.clone();
        let _t_plus = Instant::now();
        let _t_minus = Instant::now();

        let mixer_data : Arc<RwLock<(f32,f32,f32,f32)>> = Arc::new(RwLock::new((1.0, 1.0, 1.0, 1.0)));
        let _mixer_data_c = mixer_data.clone();

        rt.spawn(async move {
            let mut mgr = AudioManager::new();
            let mut muted = false;
            loop {
                let page = key_manager.current_page();
                *current_page_c.write().unwrap() = page;
                if let Some(key) = key_manager.event().await {
                    match key {
                        W213WheelKey::VolUp => {
                            muted = false;
                            mgr.offset_volume(0.01);
                        },
                        W213WheelKey::VolDown => {
                            muted = false;
                            mgr.offset_volume(-0.01);
                        },
                        W213WheelKey::Mute => {
                            muted = !muted;
                            mgr.set_mute(muted);
                        },
                        W213WheelKey::UnMute => {
                            mgr.set_mute(false);
                            muted = false;
                        }
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
            }
        });

        let s = Self {
            _bluetooth_handler: bt,
            sender_keys: ext_event_sender,
            sender
        };
        let _ = s.sender_keys.send(W213WheelKey::UnMute);
        s
    }

    pub fn send_agw_command(&self, cmd: AgwCommand) {
        let _ = self.sender.send(cmd);
    }

    pub fn wakeup(&self) {
        let _ = self.sender.send(AgwCommand::Wakeup);
        let _ = self.sender_keys.send(W213WheelKey::UnMute);
    }
}
