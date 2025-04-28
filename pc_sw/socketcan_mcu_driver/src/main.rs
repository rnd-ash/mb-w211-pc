use std::{path::Path, process::exit, thread, time::Duration};

use clap::Parser;
use tokio::{io::{AsyncReadExt, AsyncWriteExt, ReadHalf}, sync::mpsc::{UnboundedReceiver, UnboundedSender}};
use tokio_serial::{SerialPort, SerialPortBuilderExt, SerialPortInfo, SerialPortType, SerialStream};
use tokio_socketcan::{CANFrame, CANSocket};
use w211_can::{canbus::CanBus};
use futures_util::{io::ReadExact, stream::StreamExt, TryFutureExt};

#[derive(Debug, Clone, Parser)]
pub struct AppSettings {
    binf: String,
    baud: u32,
}

pub fn can_frame_from_bytes(bytes: &[u8; 16]) -> Option<(CanBus, CANFrame)> {
    // Signature has already been checked
    let targ_crc = bytes[15];
    let mut my_crc = 0u8;
    for i in 0..15u8 {
        my_crc = my_crc.wrapping_add(i).wrapping_add(bytes[i as usize]);
    }
    if my_crc != targ_crc {
        log::error!("[Rx frame] CRC mismatch!. Calc: {:02X}. Buf: {:02X?}", my_crc, bytes);
        None
    } else {
        // CRC OK
        let id = bytes[5] as u16 | (bytes[6] as u16) << 8;
        let dlc = bytes[4] & 0x0F;
        let net = (bytes[4] & 0xF0) >> 4;
        if dlc == 0 || dlc > 8 {
            log::error!("[Rx frame] Invalid CAN DLC {}", dlc);
            None
        } else if id > 0x7FF {
            log::error!("[Rx frame] Invalid CAN ID: {:04X}", id);
            None
        } else if net == 0 || net > 3 {
            log::error!("[Rx frame] Invalid net ID: {}", net);
            None
        } else {
            // Valid!
            Some((unsafe { std::mem::transmute(net) },
                CANFrame::new(id as u32, &bytes[7..7+dlc as usize], false, false).unwrap()
            ))
        }

    }

} 

fn from_can_to_pc_frame(f: &CANFrame, bus: CanBus, buf: &mut [u8; 16]) {

    buf[0] = 0xDE;
    buf[1] = 0xAD;
    buf[2] = 0xBE;
    buf[3] = 0xEF;
    buf[4] = (bus as u8) << 4 | f.data().len() as u8;
    buf[5] = (f.id() & 0xFF) as u8;
    buf[6] = ((f.id() >> 8) & 0xFF) as u8;
    buf[7..7+f.data().len()].copy_from_slice(f.data());
    // Last byte is CRC
    let mut res = 0u8;
    for i in 0..15u8 {
        res = res.wrapping_add(i).wrapping_add(buf[i as usize]);
    }
    buf[15] = res;
}

#[allow(non_snake_case)]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    let settings = AppSettings::parse();
    println!("Waiting for port to be available");
    let port_name: String;
    let s =format!("/sys/bus/usb/devices/{}/", settings.binf);
    'search: loop {
        let p = Path::new(&s);
        println!("{} {}", p.exists(), p.is_dir());
        if p.exists() && p.is_dir() {
            if let Ok(children) = std::fs::read_dir(p) {
                for c in children {
                    if let Ok(c) = c {
                        if c.file_name().to_str().unwrap().contains("tty") {
                            port_name = format!("/dev/{}", c.file_name().to_str().unwrap());
                            break 'search;
                        }
                    }
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
    
    let mut port = tokio_serial::new(port_name, settings.baud)
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
        loop {

            #[inline]
            async fn try_read_frame(uart: &mut ReadHalf<SerialStream>) -> tokio::io::Result<Option<(CanBus, CANFrame)>> {
                let mut buf: [u8; 16] = [0; 16];
                // First, read the first 4 bytes (our signature)

                // We read the first byte alone so that the UART's RX buffer is rotated on
                // successive read failures until the magic byte is located and the rest
                // of the signature passes
                loop {
                    uart.read_exact(&mut buf[..1]).await?; // Byte 0 of sig
                    if (buf[0]) == 0xDE {
                        
                        break; // Break when we maybe have our first magic byte located
                    }
                }
                uart.read_exact(&mut buf[1..4]).await?; // Byte 1-4 of the sig
                if &buf[..4] != &[0xDE, 0xAD, 0xBE, 0xEF] { // Full signature we expect
                    // Invalid sig!
                    return Ok(None);
                }
                // Read the remaining 12 bytes
                uart.read_exact(&mut buf[4..]).await?;
                Ok(can_frame_from_bytes(&buf))
            }

            match try_read_frame(&mut port_read).await {
                Ok(Some((bus, frame))) => {
                    match bus {
                        CanBus::B => to_canb.send(frame).unwrap(),
                        CanBus::C => to_canc.send(frame).unwrap(),
                        CanBus::E => to_cane.send(frame).unwrap(),
                    };
                }
                // Read OK but invalid data
                Ok(None) => {
                    eprintln!("Invalid data read");
                }
                // UART error
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