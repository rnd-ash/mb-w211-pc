use std::{io::{self, BufReader, BufRead}, sync::{Arc, RwLock, mpsc::{Receiver, Sender, self}}, time::Instant};

use packed_struct::{prelude::{PrimitiveEnum_u8, PackedStruct}};
use serial_rs::{SerialPortSettings, FlowControl, SerialPort};

use crate::canbus::{isotp::IsoTpEndpoint, CanStorage};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[derive(PrimitiveEnum_u8)]
pub enum CanBus {
    C = 67,
    B = 66,
    E = 69,
    Loopback = 0xFF
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

#[derive(Debug, Clone)]
pub struct MCUComm {
    can_tx: Sender<PCCanFrame>,
    isotp_endpoints: Arc<RwLock<Vec<IsoTpEndpoint>>>,
}


impl MCUComm {
    pub fn new(path: &str, mut can_sto: CanStorage) -> io::Result<Self> {
        let mut port = serial_rs::new_from_path(
            path,
            Some(
                SerialPortSettings::default()
                    .baud(115200)
                    .read_timeout(Some(500))
                    .write_timeout(Some(500))
                    .set_flow_control(FlowControl::None),
            ),
        )?;
        port.clear_input_buffer()?;
        port.clear_output_buffer()?;
        let mut port_clone = port.try_clone().unwrap();

        let mut endpoints: Arc<RwLock<Vec<IsoTpEndpoint>>> = Arc::new(RwLock::new(Vec::new()));
        let endpoints_t = endpoints.clone();
        let endpoints_tx = endpoints.clone();
        let (tx_can, rx_can) = mpsc::channel::<PCCanFrame>();

        let (tx_loopback, rx_loopback) = mpsc::channel::<PCCanFrame>();

        let tx_thread = std::thread::spawn(move || {
            loop {
                for endpoint in endpoints_tx.read().unwrap().iter() {
                    if let Some(f) = endpoint.get_can_to_send() {
                        port.write_all(&f.pack().unwrap()).unwrap();
                    }
                    if let Ok(loopback) = rx_loopback.try_recv() {
                        println!("LOOPBACK!: {}", loopback);
                    }
                }
                loop {
                    if let Ok(f) = rx_can.try_recv() {
                        port.write_all(&f.pack().unwrap()).unwrap();
                    } else {
                        break;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        });

        let reader_thread = std::thread::spawn(move || {
            let mut buf_reader = BufReader::new(&mut port_clone);
            let mut line: String = String::new();
            loop {
                line.clear();
                if buf_reader.read_line(&mut line).is_ok() && !line.is_empty() {
                    let parts = line.split(" ").collect::<Vec<&str>>();
                    if parts.len() != 2 {
                        println!("Corrupt line '{}'", line);
                        continue;
                    }
                    if parts[0].len() != 5 {
                        println!("Corrupt line '{}'", line);
                        continue;
                    }
                    if parts[1].len() % 2 == 0 {
                        println!("Corrupt line '{}'", line);
                        continue;
                    }
                    // Valid frame, parse it
                    let bus = match parts[0].chars().next().unwrap() {
                        'B' => CanBus::B,
                        'C' => CanBus::C,
                        'E' => CanBus::E,
                        'L' => CanBus::Loopback,
                        _ => {println!("Corrupt line {}", line); continue}
                    };
                    let id = u16::from_str_radix(&parts[0][1..],16).unwrap();
                    let mut data: Vec<u8> = Vec::new();
                    let lim = (parts[1].len() - 1)/2;
                    for i in 0..lim {
                        let b = u8::from_str_radix(&parts[1][i*2..(i*2)+2], 16).unwrap();
                        data.push(b);
                    }

                    if data.len() == 8 {
                        for endpoint in endpoints_t.read().unwrap().iter() {
                            endpoint.on_can_read(id, &data)
                        }
                    }
                    can_sto.add_frame(bus, id, &data);
                }
            }
        });
        Ok(Self{
            can_tx: tx_can,
            isotp_endpoints: endpoints
        })
    }

    pub fn register_endpoint(&mut self, endpoint: &IsoTpEndpoint) {
        self.isotp_endpoints.write().unwrap().push(endpoint.clone());
    }

    pub fn send_frame(&mut self, frame: PCCanFrame) {
        self.can_tx.send(frame);
    }
}