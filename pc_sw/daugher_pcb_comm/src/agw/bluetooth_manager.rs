use crate::agw::{AgwCommand, AudioPageState, AudioSymbol, IcText, TextFmtFlags};

use bluer::{Adapter, Device};
use dbus::arg::PropMap;
use dbus::message::MatchRule;
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
use dbus::nonblock::{LocalConnection, Proxy, SyncConnection};
use tokio::runtime::Handle;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use std::sync::{Arc, RwLock};
use std::time::Duration;


const SERVICE: &str = "org.bluez";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BtCommand {
    Next,
    Prev
}

#[derive(Debug, Clone)]
pub struct BluetoothManager {
    connected_device: Arc<RwLock<Option<String>>>,
    sender: UnboundedSender<BtCommand>
}

impl BluetoothManager {
    pub fn new(sender: UnboundedSender<AgwCommand>, handle: &Handle) -> Self {
        let connected_device = Arc::new(RwLock::new(None));
        let connected_device_t = connected_device.clone();

        let (sender_bt, mut receiver_bt) = unbounded_channel::<BtCommand>();
        handle.spawn(async move {
            let bt_idle_state: AudioPageState = AudioPageState {
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
            
            log::debug!("Waiting for bluetooth");
            let session = bluer::Session::new().await.unwrap();
            let adapter: Adapter = loop {
                if let Ok(a) = session.default_adapter().await {
                    break a;
                }
            };
            log::info!("Bluetooth now up!");
            let _ = sender.send(AgwCommand::SetAudioPage(bt_idle_state.clone()));
            let (resource, connection) = dbus_tokio::connection::new_system_sync().unwrap();
            let _handle = tokio::spawn(async {
                let err = resource.await;
                panic!("Lost connection to D-Bus: {}", err);
            });

            let mut track_name: Option<String> = None;
            let mut dbus: Option<Proxy<Arc<SyncConnection>>> = None;
            let mut last_device: Option<(String, String)> = None;
            loop {
                let mut device: Option<(String, String)> = None;
                for addr in adapter.device_addresses().await.unwrap() {
                    if let Ok(d) = adapter.device(addr) {
                        if d.is_connected().await.unwrap_or(false) {
                            let addr = d.address().to_string().replace(":", "_");
                            let name = d.name().await.unwrap().unwrap_or(addr.clone());
                            device = Some((addr, name));
                            break;
                        }
                    }
                }
                if last_device != device {
                    // Device change
                    if let Some((addr, name)) = &device {
                        log::info!("Now connected to {} at address {}", name, addr);
                        dbus = Some(
                            Proxy::new(
                                SERVICE, 
                                format!("/org/bluez/hci0/dev_{}/player0", addr), 
                                Duration::from_millis(2000), 
                                connection.clone()
                            )
                        );
                        let _ = sender.send(AgwCommand::SetAudioPage(AudioPageState {
                            header_text: IcText {
                                format: TextFmtFlags::LEFT,
                                text: "Bluetooth".to_string(),
                            },
                            body_text: IcText {
                                format: TextFmtFlags::CENTER,
                                text: format!("Connected to {}", name.clone()),
                            },
                            symbol_top: AudioSymbol::None,
                            symbol_bottom: AudioSymbol::None,
                        }));

                    } else {
                        // Disconnected
                        log::info!("Bluetooth disconnected");
                        let _ = sender.send(AgwCommand::SetAudioPage(bt_idle_state.clone()));
                        track_name = None;
                        dbus = None;
                    }
                }
                if let Some(proxy) = &dbus {
                    //proxy.get_all(interface_name)
                    //proxy.get_managed_objects()
                    tokio::select! {
                        Some(cmd) = receiver_bt.recv() => {
                            let _r: Result<(), _> = match cmd {
                                BtCommand::Next =>  proxy.method_call("org.bluez.MediaPlayer1", "Next", ()).await,
                                BtCommand::Prev =>  proxy.method_call("org.bluez.MediaPlayer1", "Previous", ()).await
                            };
                        },
                        Ok(meta) = proxy.get::<PropMap>("org.bluez.MediaPlayer1", "Track") => {
                            if let Some(track) = meta.get_key_value("Title") {
                                let t = Some(track.1.0.as_str().unwrap().to_string());
                                if t != track_name {
                                    track_name = t;
                                    let name = track_name.clone().unwrap();
                                    let _ = sender.send(AgwCommand::SetAudioBodyText(IcText {
                                        format: TextFmtFlags::CENTER,
                                        text: name.clone(),
                                    })).and_then(|_| {
                                        sender.send(AgwCommand::TrackUpdate(name.clone()))
                                    }).and_then(|_| {
                                        if name.is_empty() {
                                            sender.send(AgwCommand::SetAudioSymbols(
                                                AudioSymbol::None,
                                                AudioSymbol::None,
                                            ))
                                        } else {
                                            sender.send(AgwCommand::SetAudioSymbols(
                                                AudioSymbol::Up,
                                                AudioSymbol::Down,
                                            ))
                                        }
                                    });
                                } else {
                                    tokio::time::sleep(Duration::from_millis(500)).await;
                                }
                            }
                        }
                    }
                }
                last_device = device;
            }
        });

        Self { 
            connected_device,
            sender: sender_bt
        }
    }

    #[allow(unused)]
    pub fn connected_device(&self) -> Option<String> {
        self.connected_device.read().unwrap().clone()
    }
    
    pub fn send_media_control(&self, cmd: BtCommand) {
        let _ = self.sender.send(cmd);
    }

}
