use std::{path::Path, thread, time::{Duration, Instant}, io::{stdout, stdin, BufRead}};

use packed_struct::{prelude::{PrimitiveEnum_u8, PackedStruct}};

use crate::{mcu_comm::{MCUComm, PCCanFrame}, canbus::{isotp::IsoTpEndpoint, CanStorage}};
mod mcu_comm;
mod canbus;
mod agw;

fn main() {
    env_logger::init();
    
    // TODO path as launch arg
    while !Path::new("/dev/ttyACM0").exists() {
        thread::sleep(Duration::from_millis(500));
    }
    println!("SAME Connection found! Connecting!");

    let mut can_storage = CanStorage::default();
    let mut mcu = match MCUComm::new("/dev/ttyACM0", can_storage.clone()) {
        Ok(mcu) => {
            println!("MCU Comm Create OK!");
            mcu
        },
        Err(e) => {
            println!("MCU Comm Creation failed! {:?}", e);
            return;
        }
    };
    let now = Instant::now();
    let f = PCCanFrame {
        can_bus_tag: mcu_comm::CanBus::C,
        can_id: 0x321,
        dlc: 7,
        data: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
    };
    
    let agw = agw::AgwEmulator::new(&mut mcu);
    let mut next_down = false;
    let mut prev_down = false;
    let mut stdin = stdin();
    let mut mrm_frame = PCCanFrame { // MRM A2
        can_bus_tag: mcu_comm::CanBus::B,
        can_id: 0x1A8,
        dlc: 2,
        data: [0,0,0,0,0,0,0,0],
    };
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.starts_with('a') {
            mrm_frame.data[0] = 0x04;
            mcu.send_frame(mrm_frame);
            std::thread::sleep(Duration::from_millis(20));
            mcu.send_frame(mrm_frame);
            std::thread::sleep(Duration::from_millis(20));
            mrm_frame.data[0] = 0;
            mcu.send_frame(mrm_frame);
            std::thread::sleep(Duration::from_millis(20));
            mcu.send_frame(mrm_frame);
        } else if l.starts_with('d') {
            mrm_frame.data[0] = 0x08;
            mcu.send_frame(mrm_frame);
            std::thread::sleep(Duration::from_millis(20));
            mcu.send_frame(mrm_frame);
            std::thread::sleep(Duration::from_millis(20));
            mrm_frame.data[0] = 0;
            mcu.send_frame(mrm_frame);
            std::thread::sleep(Duration::from_millis(20));
            mcu.send_frame(mrm_frame);
        }
    }
    loop {
        std::thread::sleep(Duration::from_millis(10));
    }
}
