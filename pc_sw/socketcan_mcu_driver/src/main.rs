use std::{time::Duration, thread, sync::{Arc, atomic::{AtomicU32, Ordering}}, io::{Read, ErrorKind}, process::exit};

use clap::Parser;
use serial_rs::SerialPortSettings;
use serial_rs::*;
use w211_can::{canbus::CanBus, socketcan::{CanSocket, SocketOptions, Socket, CanDataFrame, EmbeddedFrame, CanFrame}, socketcan_isotp::{Id, StandardId}};

#[derive(Debug, Clone, Parser)]
pub struct AppSettings {
    baud: u32
}

pub fn can_frame_from_bytes(b: &[u8; 13]) -> Option<(CanBus, CanFrame)> {
    // Easiest check is CRC first
    let mut crc = 0xFFu8;
    for i in 0..12u8 {
        crc = crc.wrapping_sub(i);
        crc = crc.wrapping_sub(b[i as usize]);
    }
    if crc != b[12] {
        // CRC compare failed
        return None;
    }
    let bus_tag = match b[0] {
        67 => CanBus::C,
        66 => CanBus::B,
        69 => CanBus::E,
        _ => return None
    };
    let id = ((b[2] as u16) << 8) | b[1] as u16;
    if id > 0x7FF {
        return None;
    }
    let dlc = b[3];
    if dlc > 8 {
        return None;
    }
    // Can Frame OK!
    Some((bus_tag,
        CanFrame::Data(
            CanDataFrame::new(
                Id::Standard(unsafe { StandardId::new_unchecked(id) }),
            &b[4..4+dlc as usize]
            ).unwrap()
        ))
    )

} 

fn from_can_to_pc_frame(f: &CanFrame, bus: CanBus) -> Option<[u8; 13]> {
    if let Id::Standard(std_id) = f.id() {
        let mut ret = [0u8; 13];
        let id = std_id.as_raw();
        ret[0] = bus as u8;
        ret[1] = (id & 0xFF) as u8;
        ret[2] = ((id >> 8) & 0xFF) as u8;
        ret[3] = f.dlc() as u8;
        ret[4..4+f.dlc()].copy_from_slice(&f.data());

        // Sign
        let mut crc = 0xFFu8;
        for i in 0..12u8 {
            crc = crc.wrapping_sub(i);
            crc = crc.wrapping_sub(ret[i as usize]);
        }
        ret[12] = crc;
        Some(ret)
    } else {
        None
    }
}

fn make_can_channel(iface: CanBus) -> (Arc<CanSocket>, Arc<CanSocket>) {
    let can = Arc::new(iface.create_can_socket());
    can.set_filter_accept_all().unwrap();
    can.set_error_filter_drop_all().unwrap();
    can.set_nonblocking(true).unwrap();
    (can.clone(), can)
}

pub const VID: u16 = 0x03eb;
pub const PID: u16 = 0x2175;

#[allow(non_snake_case)]
fn main() {
    let settings = AppSettings::parse();
    println!("Waiting for port to be available");
    let port_name: String;
    'search: loop {
        if let Ok(list) = serial_rs::list_ports() {
            for port in list {
                if port.get_vid() == VID && port.get_pid() == PID {
                    port_name = port.get_port().to_string();
                    println!("Found {:04X}:{:04X} on {}", VID, PID, port.get_port());
                    break 'search;
                }
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
    println!("Port ready!");

    let (CAN_B, CAN_B_R) = make_can_channel(CanBus::B);
    let (CAN_C, CAN_C_R) = make_can_channel(CanBus::C);
    let (CAN_E, CAN_E_R) = make_can_channel(CanBus::E);

    let port_settings = SerialPortSettings::default()
        .baud(settings.baud)
        .read_timeout(Some(10000))
        .write_timeout(None)
        .set_flow_control(FlowControl::RtsCts);
    

    let mut port = serial_rs::new_from_path(
        &port_name,
        Some(port_settings)
    ).unwrap();

    let mut port_clone = port.try_clone().unwrap();
    let error_counter = Arc::new(AtomicU32::new(0));
    let error_counter_writer = error_counter.clone();

    const MAX_ERRORS: u32 = 100;

    std::thread::spawn(move || {
        println!("Writer thread running");
        port.clear_output_buffer().unwrap();
        while error_counter_writer.load(Ordering::Relaxed) < MAX_ERRORS {
            let mut data: Vec<u8> = Vec::new();
            while let Ok(f) = CAN_B.read_frame() {
                if let Some(cf) = from_can_to_pc_frame(&f, CanBus::B) {
                    data.extend_from_slice(&cf);
                }
            }
            while let Ok(f) = CAN_C.read_frame() {
                if let Some(cf) = from_can_to_pc_frame(&f, CanBus::C) {
                    data.extend_from_slice(&cf);
                }
            }
            while let Ok(f) = CAN_E.read_frame() {
                if let Some(cf) = from_can_to_pc_frame(&f, CanBus::E) {
                    data.extend_from_slice(&cf);
                }
            }

            if data.len() != 0 {
                port.write(&data).unwrap();
                port.flush().unwrap();
            } else {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }     
        }
        eprintln!("Transmitter thread terminating");
    });
    let reader_thread = std::thread::spawn(move || {
        println!("Reader thread running");
        port_clone.clear_input_buffer().unwrap();
        while error_counter.load(Ordering::Relaxed) < MAX_ERRORS {
            let mut buf: [u8; 13] = [0; 13];
            match port_clone.read_exact(&mut buf) {
                Ok(_) => {
                    match can_frame_from_bytes(&buf) {
                        Some((bus, frame)) => {
                            match bus {
                                CanBus::C => CAN_C_R.write_frame(&frame).unwrap(),
                                CanBus::B => CAN_B_R.write_frame(&frame).unwrap(),
                                CanBus::E => CAN_E_R.write_frame(&frame).unwrap(),
                            }
                            error_counter.store(0, Ordering::Relaxed);
                        },
                        None => {
                            error_counter.fetch_add(1, Ordering::Relaxed);
                            port_clone.clear_input_buffer().unwrap();
                            eprintln!("Serialize error. buf was {:02X?}", buf);
                        }
                    }
                },
                Err(e) => {
                    if ErrorKind::BrokenPipe == e.kind() {
                        exit(1); // Disconnected
                    }
                    println!("READ error {e:?}");
                    error_counter.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
        eprintln!("Receiver thread terminating");
    });
    println!("CAN Daemon is running");
    reader_thread.join().unwrap();
    println!("Critical error. Terminating");
}
