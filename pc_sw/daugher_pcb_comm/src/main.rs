use std::{
    io::{stdin, stdout, BufRead},
    path::Path,
    thread,
    time::{Duration, Instant},
};

use packed_struct::prelude::{PackedStruct, PrimitiveEnum_u8};

use crate::{
    canbus::{isotp::IsoTpEndpoint, CanStorage},
    mcu_comm::{MCUComm, PCCanFrame},
};
mod agw;
mod canbus;
mod mcu_comm;

const MCU_VID: u16 = 1003;
const MCU_PID: u16 = 8565;

fn main() {
    env_logger::init();

    let mut path = String::new();
    'outer: loop {
        for p in serial_rs::list_ports().unwrap_or_default() {
            if p.get_vid() == MCU_VID && p.get_pid() == MCU_PID {
                path = p.get_port().to_string();
                break 'outer;
            }
        }
        thread::sleep(Duration::from_millis(1000));
    }
    let m = serial_rs::list_ports();
    println!("SAME Connection found! Connecting!");
    let mut can_storage = CanStorage::default();
    let mut mcu = match MCUComm::new(&path, can_storage.clone()) {
        Ok(mcu) => {
            println!("MCU Comm Create OK!");
            mcu
        }
        Err(e) => {
            println!("MCU Comm Creation failed! {:?}", e);
            return;
        }
    };
    let now = Instant::now();

    let agw = agw::AgwEmulator::new(&mut mcu, can_storage.clone());
    let mut next_down = false;
    let mut prev_down = false;
    let mut stdin = stdin();
    let mut mrm_frame = PCCanFrame {
        // MRM A2
        can_bus_tag: mcu_comm::CanBus::B,
        can_id: 0x1A8,
        dlc: 2,
        data: [0, 0, 0, 0, 0, 0, 0, 0],
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
