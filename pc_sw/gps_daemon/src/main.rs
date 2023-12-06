use std::{time::Duration, thread, sync::{Arc, atomic::{AtomicU32, Ordering}}, io::{Read, ErrorKind, BufReader, BufRead}, process::exit};

use nmea0183::Parser;
use serial_rs::SerialPortSettings;
use serial_rs::*;
use w211_can::{canbus::CanBus, socketcan::{}, socketcan_isotp::{}};

pub const VID: u16 = 0x1546;
pub const PID: u16 = 0x01a7;

#[allow(non_snake_case)]
fn main() {
    println!("Waiting for port to be available");
    let port_name: String;
    'search: loop {
        if let Ok(list) = serial_rs::list_ports() {
            for port in list {
                if port.get_vid() == VID && port.get_pid() == PID {
                    port_name = port.get_port().to_string();
                    println!("FPS Found {:04X}:{:04X} on {}", VID, PID, port.get_port());
                    break 'search;
                }
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
    println!("Port ready!");
    let can_e = CanBus::E.create_can_socket();

    let port_settings = SerialPortSettings::default()
        .baud(9600)
        .read_timeout(Some(10000))
        .write_timeout(None)
        .set_flow_control(FlowControl::RtsCts);
    

    let mut port = serial_rs::new_from_path(
        &port_name,
        Some(port_settings)
    ).unwrap();

    let error_counter = 0;
    let mut br = BufReader::new(port);
    const MAX_ERRORS: u32 = 100;

    let mut parser = Parser::new();

    let mut buf = String::new();
    loop {
        buf.clear();
        if let Ok(amount) = br.read_line(&mut buf) {
            buf.push_str("\r\n");
            for r in parser.parse_from_bytes(buf.as_bytes()) {
                println!("{:?}", r);
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
