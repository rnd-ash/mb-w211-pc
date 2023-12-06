use crate::agw::{AgwCommand, AudioPageState, AudioSymbol, IcText, TextFmtFlags};
use ::futures::{pin_mut, stream::SelectAll, StreamExt};
use bluer::{Adapter, AdapterEvent, Address, Device, DeviceEvent};
use dbus::arg::{PropMap, RefArg};
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::{blocking::Connection, channel::MatchingReceiver, message::MatchRule, Error, Message};
use tokio::{task, time};
use std::sync::mpsc::{Sender, self};
use std::sync::{Arc, RwLock};
use std::{future, time::Duration};
use tokio::sync::futures;

const SERVICE: &str = "org.bluez";
const PATH: &str = "/org/bluez";
const INTERFACE: &str = "org.bluez.Adapter1";
const ADAPTER: &str = "hci0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BtCommand {
    Next,
    Prev
}

#[derive(Debug, Clone)]
pub struct BluetoothManager {
    connected_device: Arc<RwLock<Option<String>>>,
    sender: mpsc::Sender<BtCommand>
}

impl BluetoothManager {
    pub fn new(sender: Sender<AgwCommand>) -> Self {
        let connected_device = Arc::new(RwLock::new(None));
        let connected_device_t = connected_device.clone();

        let (sender_bt, receiver_bt) = mpsc::channel::<BtCommand>();
        std::thread::spawn(move || {
            let BT_IDLE_STATE: AudioPageState = AudioPageState {
                header_text: IcText {
                    format: TextFmtFlags::LEFT,
                    text: "Bluetooth".to_string(),
                },
                body_text: IcText {
                    format: TextFmtFlags::CENTER,
                    text: "No device".to_string(),
                },
                symbol_top: AudioSymbol::None,
                symbol_bottom: AudioSymbol::None,
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            log::debug!("Waiting for bluetooth");
            let adapter = rt.block_on(async {
                let session = bluer::Session::new().await.unwrap();
                loop {
                    if let Ok(a) = session.default_adapter().await {
                        return a;
                    }
                }
            });
            log::info!("Bluetooth now up!");
            sender.send(AgwCommand::SetAudioPage(BT_IDLE_STATE.clone()));
            let connection = Connection::new_system().unwrap();
            let mut rule = MatchRule::new();
            let mut dev_name: Option<String> = None;
            let mut track_name: Option<String> = None;
            loop {
                let mut act = false;
                let mut dev: Option<Device> = None;
                rt.block_on(async {
                    for addr in adapter.device_addresses().await.unwrap() {
                        if let Ok(d) = adapter.device(addr) {
                            if d.is_connected().await.unwrap_or(false) {
                                dev_name = d.name().await.unwrap();
                                dev = Some(d);
                                break;
                            }
                        }
                    }
                });
                if *connected_device_t.read().unwrap() != dev_name {
                    log::info!("Now connected to {:?}", dev_name);
                    if connected_device_t.read().unwrap().is_some() != dev_name.is_some() {
                        if dev_name.is_none() {
                            sender.send(AgwCommand::SetAudioPage(BT_IDLE_STATE.clone()));
                            track_name = None;
                        } else {
                            sender.send(AgwCommand::SetAudioPage(AudioPageState {
                                header_text: IcText {
                                    format: TextFmtFlags::LEFT,
                                    text: "Bluetooth".to_string(),
                                },
                                body_text: IcText {
                                    format: TextFmtFlags::CENTER,
                                    text: format!("Connected to {}", dev_name.clone().unwrap()),
                                },
                                symbol_top: AudioSymbol::None,
                                symbol_bottom: AudioSymbol::None,
                            }));
                        }
                    } else {
                        if dev_name.is_none() {
                            sender.send(AgwCommand::SetAudioPage(BT_IDLE_STATE.clone()));
                        } else {
                            // Just device name changed
                            sender.send(AgwCommand::SetAudioBodyText(IcText {
                                format: TextFmtFlags::CENTER,
                                text: format!("Connected to {}", dev_name.clone().unwrap()),
                            }));
                        }
                    }

                    *connected_device_t.write().unwrap() = dev_name.clone();

                    sender.send(AgwCommand::SetAudioBodyText(IcText {
                        format: TextFmtFlags::CENTER,
                        text: dev_name.clone().unwrap_or_else(|| "No device".to_string()),
                    }));
                    sender.send(AgwCommand::SetAudioSymbols(
                        AudioSymbol::None,
                        AudioSymbol::None,
                    ));
                }
                if let Some(device) = dev {
                    let addr = device.address().to_string().replace(":", "_");
                    let proxy = connection.with_proxy(
                        SERVICE,
                        format!("/org/bluez/hci0/dev_{}/player0", addr),
                        Duration::from_millis(1000),
                    );
                    if let Ok(meta) = proxy.get::<PropMap>("org.bluez.MediaPlayer1", "Track") {
                        act = true;
                        if let Some(track) = meta.get_key_value("Title") {
                            let t = Some(track.1.as_str().unwrap().to_string());
                            if t != track_name {
                                track_name = t;
                                let name = track_name.clone().unwrap();
                                println!("New track {}", name);
                                sender.send(AgwCommand::SetAudioBodyText(IcText {
                                    format: TextFmtFlags::CENTER,
                                    text: name.clone(),
                                }));
                                
                                sender.send(AgwCommand::TrackUpdate(name.clone()));
                                if name.is_empty() {
                                    sender.send(AgwCommand::SetAudioSymbols(
                                        AudioSymbol::None,
                                        AudioSymbol::None,
                                    ));
                                } else {
                                    sender.send(AgwCommand::SetAudioSymbols(
                                        AudioSymbol::Up,
                                        AudioSymbol::Down,
                                    ));
                                }
                            }
                        }
                        if let Ok(cmd) = receiver_bt.try_recv() {
                            let _: Result<(), Error> = match cmd {
                                BtCommand::Next =>  proxy.method_call("org.bluez.MediaPlayer1", "Next", ()),
                                BtCommand::Prev =>  proxy.method_call("org.bluez.MediaPlayer1", "Previous", ())
                            };
                            println!("Bt command {:?}", cmd);
                        }
                    }
                }
                if act {
                    std::thread::sleep(std::time::Duration::from_millis(250));
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        });

        Self { 
            connected_device,
            sender: sender_bt
        }
    }

    pub fn connected_device(&self) -> Option<String> {
        self.connected_device.read().unwrap().clone()
    }

    pub fn send_media_control(&self, cmd: BtCommand) {
        self.sender.send(cmd);
    }

}
