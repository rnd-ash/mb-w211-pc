use std::{process::exit, thread, time::Duration};

use clap::Parser;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc::{UnboundedReceiver, UnboundedSender}};
use tokio_serial::{SerialPort, SerialPortBuilderExt, SerialPortInfo, SerialPortType};
use tokio_socketcan::{CANFrame, CANSocket};
use w211_can::{canbus::CanBus};
use futures_util::{stream::StreamExt, TryFutureExt};

#[derive(Debug, Clone, Parser)]
pub struct AppSettings {
    baud: u32
}

pub fn can_frame_from_bytes(b: &[u8; 16]) -> Option<(CanBus, CANFrame)> {
    let bus_tag = match b[0] & 0b11 {
        0b01 => CanBus::B,
        0b10 => CanBus::C,
        0b11 => CanBus::E,
        _ => return None
    };
    let id = ((b[2] as u16) << 8) | ((b[1] as u16));
    if id > 0x7FF {
        return None;
    }
    let dlc = b[3];
    if dlc > 8 {
        return None;
    }
    if
        b[12] != 0xDE ||
        b[13] != 0xAD ||
        b[14] != 0xBE ||
        b[15] != 0xEF
    {
        return None;
    }
    // Can Frame OK!
    Some((bus_tag,
        CANFrame::new(id as u32, &b[4..4+dlc as usize], false, false).unwrap()
    ))

} 

fn from_can_to_pc_frame(f: &CANFrame, bus: CanBus, buf: &mut [u8; 16]) {
    let id = f.id();
    buf[0] = bus as u8;
    buf[1] = (id & 0xFF) as u8;
    buf[2] = ((id >> 8) & 0xFF) as u8;
    buf[3] = f.data().len() as u8;
    buf[4..4+f.data().len()].copy_from_slice(&f.data());
    buf[12] = 0xDE;
    buf[13] = 0xAD;
    buf[14] = 0xBE;
    buf[15] = 0xEF;
}

pub const VID: u16 = 0x03eb;
pub const PID: u16 = 0x2175;

#[allow(non_snake_case)]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let settings = AppSettings::parse();
    println!("Waiting for port to be available");
    let port_info: SerialPortInfo;
    'search: loop {
        for port in tokio_serial::available_ports().unwrap_or_default() {
            if let SerialPortType::UsbPort(usb) = &port.port_type {
                if usb.vid == VID && usb.pid == PID {
                    port_info = port.clone();
                    println!("Found {:04X}:{:04X} on {}", VID, PID, port.port_name);
                    break 'search;
                }
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
    println!("Port ready!");

    let canb = tokio_socketcan::CANSocket::open("vcan_b").unwrap();
    let canc = tokio_socketcan::CANSocket::open("vcan_c").unwrap();
    let cane = tokio_socketcan::CANSocket::open("vcan_e").unwrap();
    let (to_canb, from_canb) = tokio::sync::mpsc::unbounded_channel::<CANFrame>();
    let (to_canc, from_canc) = tokio::sync::mpsc::unbounded_channel::<CANFrame>();
    let (to_cane, from_cane) = tokio::sync::mpsc::unbounded_channel::<CANFrame>();
    let (to_port, mut from_bus) = tokio::sync::mpsc::unbounded_channel::<[u8; 16]>();

    // Spawn handlers for each bus
    let canb_sender = to_port.clone();
    let canc_sender = to_port.clone();
    let cane_sender = to_port.clone();

    let spawn_can_thread = |mut receiver: UnboundedReceiver<CANFrame>, sender: UnboundedSender<[u8; 16]>, mut bus: CANSocket, tag: CanBus| {
        tokio::spawn(async move {
            let mut buf: [u8; 16] = [0; 16];
            loop {
                tokio::select! {
                    Some(f) = receiver.recv() => {
                        bus.write_frame(f).unwrap().await.unwrap();
                    }
                    Some(Ok(frame)) = bus.next() => {
                        from_can_to_pc_frame(&frame, tag, &mut buf);
                        sender.send(buf.clone()).unwrap();
                    }
                }
            }
        });
    };

    spawn_can_thread(from_canb, canb_sender, canb, CanBus::B);
    spawn_can_thread(from_canc, canc_sender, canc, CanBus::C);
    spawn_can_thread(from_cane, cane_sender, cane, CanBus::E);

    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let mut port = tokio_serial::new(port_info.port_name, settings.baud)
        .timeout(Duration::from_millis(1000))
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::None)
        .open_native_async()
        .unwrap();
    port.set_exclusive(true).unwrap();
    port.clear(tokio_serial::ClearBuffer::All).unwrap();
    let (mut port_read, mut port_write) = tokio::io::split(port);
    println!("Starting workers");
    runtime.spawn(async move {
        let mut buf: [u8; 16] = [0; 16];
        loop {
            match port_read.read_exact(&mut buf).await {
                Ok(16) => {
                    match can_frame_from_bytes(&buf) {
                        Some((bus, frame)) => {
                            match bus {
                                CanBus::B => to_canb.send(frame).unwrap(),
                                CanBus::C => to_canc.send(frame).unwrap(),
                                CanBus::E => to_cane.send(frame).unwrap(),
                            };
                        },
                        None => {
                            eprintln!("CAN Decode error! Buffer was {buf:02X?}");
                            let mut b = 0x00;
                            let mut counter = 0;
                            loop {
                                b = port_read.read_u8().await.unwrap();
                                println!("Read {b:02X?}");
                                if b == 0xEF {
                                    break;
                                } else {
                                    counter+=1;
                                }
                                if counter == 16 {
                                    eprintln!("MAX RETRIES REACHED");
                                    exit(1);
                                }
                            }
                        }
                    }
                },
                Ok(x) => {
                    eprintln!("Could not read full 16 bytes, only {x}!");
                }
                Err(e) => {
                    println!("Device disconnected! Exiting: {}", e.to_string());
                    exit(1);
                }
            }
        }
    });
    let handle = runtime.spawn(async move {
        loop {
            if let Some(to_write) = from_bus.recv().await {
                port_write.write_all(&to_write).await.unwrap();
            }
        }
    });
    handle.await.unwrap();
}