use std::{future, time::Duration};
use dbus::{blocking::Connection, channel::MatchingReceiver, message::MatchRule, Message};
use ::futures::Stream;
use tokio::sync::futures;

const BLUEZ: &str = "org.bluez";
const ADAPTER: &str = "/hci0";

#[derive(Debug)]
pub struct BluetoothManager {
}

impl BluetoothManager {

    pub fn new() -> Self {
        let connection = Connection::new_system().unwrap();
        let mut rule = MatchRule::new();
        let media_name = connection.with_proxy(BLUEZ, ADAPTER, Duration::from_millis(1000));
        
        //let _ = media_name.match_signal(|h: ComExampleDbustestHelloHappened, _: &Connection, _: &Message| {
        //    println!("Hello happened from sender: {}", h.sender);
        //    true
        //});

        //todo!()
        Self{}
    }   
}

