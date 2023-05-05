use std::{path::Path, time::Duration, thread, sync::{Arc, atomic::{AtomicU32, Ordering}}};

use clap::Parser;
use packed_struct::{prelude::{PrimitiveEnum_u8, PackedStruct}, PackedStructSlice};
use serial_rs::SerialPortSettings;
use serial_rs::*;
use socketcan::{CanSocket, Socket, CanFrame, EmbeddedFrame, CanDataFrame, Id, StandardId};

#[derive(Debug, Clone, Parser)]
pub struct AppSettings {
    port: String,
    baud: u32
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[derive(PrimitiveEnum_u8)]
pub enum CanBus {
    C = 67,
    B = 66,
    E = 69,
}

impl std::fmt::Display for CanBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanBus::C => f.write_str("Can_C"),
            CanBus::B => f.write_str("Can_B"),
            CanBus::E => f.write_str("Can_E"),
        }
    }
}

impl CanBus {
    pub fn get_iface_name(&self) -> &'static str {
        match self {
            CanBus::C => "vcan_c",
            CanBus::B => "vcan_b",
            CanBus::E => "vcan_e",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, PackedStruct)]
#[packed_struct(bit_numbering="msb0")]
pub struct PCCanFrame {
    #[packed_field(bits="0..8",ty="enum")]
    pub can_bus_tag: CanBus,
    #[packed_field(endian = "lsb")]
    pub can_id: u16,
    pub dlc: u8,
    pub data: [u8; 8]
}

fn from_can_to_pc_frame(f: &CanFrame, bus: CanBus) -> PCCanFrame {
    let mut data: [u8; 8] = [0; 8];
    for x in 0..f.dlc() {
        data[x] = f.data()[x];
    }

    let id = if let Id::Standard(s) = f.id() {
        s.as_raw()
    } else {
        0x7FF
    };

    PCCanFrame { 
        can_bus_tag: bus, 
        can_id: id, 
        dlc: f.dlc() as u8, 
        data,
    }
}

fn from_pc_to_can_frame(f: &PCCanFrame) -> CanFrame {
    CanFrame::Data(
        CanDataFrame::new(
            Id::Standard(unsafe { StandardId::new_unchecked(f.can_id) }), 
            &f.data[..f.dlc as usize]
        ).unwrap()
    )
}


fn make_can_channel(iface: CanBus) -> (Arc<CanSocket>, Arc<CanSocket>) {
    let can = Arc::new(CanSocket::open(iface.get_iface_name()).unwrap());
    can.set_filter_accept_all().unwrap();
    can.set_nonblocking(true).unwrap();
    (can.clone(), can)
}

#[allow(non_snake_case)]
fn main() {
    let settings = AppSettings::parse();
    println!("Waiting for port to be available");
    while !Path::new(&settings.port).exists() {
        thread::sleep(Duration::from_millis(500));
    }
    println!("Port ready!");

    let (CAN_B, CAN_B_R) = make_can_channel(CanBus::B);
    let (CAN_C, CAN_C_R) = make_can_channel(CanBus::C);
    let (CAN_E, CAN_E_R) = make_can_channel(CanBus::E);

    let port_settings = SerialPortSettings::default()
        .baud(settings.baud)
        .read_timeout(Some(2000))
        .write_timeout(Some(2000))
        .set_flow_control(FlowControl::None);
    

    let mut port = serial_rs::new_from_path(
        &settings.port,
        Some(port_settings)
    ).unwrap();

    port.clear_input_buffer().unwrap();
    port.clear_output_buffer().unwrap();
    let mut port_clone = port.try_clone().unwrap();

    let error_counter = Arc::new(AtomicU32::new(0));
    let error_counter_writer = error_counter.clone();

    const MAX_ERRORS: u32 = 10;

    std::thread::spawn(move || {
        println!("Writer thread running");
        while error_counter_writer.load(Ordering::Relaxed) < MAX_ERRORS {
            if let Ok(f) = CAN_B.read_frame() {
                let f = from_can_to_pc_frame(&f, CanBus::B);
                port.write(&f.pack().unwrap()).unwrap();
            }
            if let Ok(f) = CAN_C.read_frame() {
                let f = from_can_to_pc_frame(&f, CanBus::B);
                port.write(&f.pack().unwrap()).unwrap();
            }
            if let Ok(f) = CAN_E.read_frame() {
                let f = from_can_to_pc_frame(&f, CanBus::B);
                port.write(&f.pack().unwrap()).unwrap();
            }

            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        eprintln!("Transmitter thread terminating");
    });
    let reader_thread = std::thread::spawn(move || {
        println!("Reader thread running");
        while error_counter.load(Ordering::Relaxed) < MAX_ERRORS {
            let mut buf: [u8; 12] = [0; 12];
            match port_clone.read_exact(&mut buf) {
                Ok(_) => {
                    println!("READ OK");
                    match PCCanFrame::unpack_from_slice(&buf) {
                        Ok(f) => {
                            let cf = from_pc_to_can_frame(&f);
                            match f.can_bus_tag {
                                CanBus::C => CAN_C_R.write_frame(&cf).unwrap(),
                                CanBus::B => CAN_B_R.write_frame(&cf).unwrap(),
                                CanBus::E => CAN_E_R.write_frame(&cf).unwrap(),
                            }
                            error_counter.store(0, Ordering::Relaxed);
                        },
                        Err(e) => {
                            error_counter.fetch_add(1, Ordering::Relaxed);
                            port_clone.clear_input_buffer().unwrap();
                            eprintln!("Serialize error. buf was {:02X?}", buf);
                        }
                    }
                }
                Err(e) => {
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
