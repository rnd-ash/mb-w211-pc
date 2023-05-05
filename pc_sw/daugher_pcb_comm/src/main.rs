use std::{
    io::{stdin, BufRead},
    time::{Duration, Instant},
};

mod agw;
mod w211can;

fn main() {
    env_logger::init();
    let now = Instant::now();

    let agw = agw::AgwEmulator::new();
    let mut next_down = false;
    let mut prev_down = false;
    let mut stdin = stdin();

    let mrm_can = w211can::CanBus::B.create_can_socket(&[0x0000]).unwrap();
    const MRM_CAN_ID: u16 = 0x1A8;
    let mut mrm_data: [u8; 2] = [0x00, 0x00];

    /*
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.starts_with('a') {
            mrm_data[0] = 0x04;
            mrm_can.send_frame(MRM_CAN_ID, &mrm_data);
            std::thread::sleep(Duration::from_millis(20));
            mrm_can.send_frame(MRM_CAN_ID, &mrm_data);
            std::thread::sleep(Duration::from_millis(20));
            mrm_data[0] = 0;
            mrm_can.send_frame(MRM_CAN_ID, &mrm_data);
            std::thread::sleep(Duration::from_millis(20));
            mrm_can.send_frame(MRM_CAN_ID, &mrm_data);
        } else if l.starts_with('d') {
            mrm_data[0] = 0x08;
            mrm_can.send_frame(MRM_CAN_ID, &mrm_data);
            std::thread::sleep(Duration::from_millis(20));
            mrm_can.send_frame(MRM_CAN_ID, &mrm_data);
            std::thread::sleep(Duration::from_millis(20));
            mrm_data[0] = 0;
            mrm_can.send_frame(MRM_CAN_ID, &mrm_data);
            std::thread::sleep(Duration::from_millis(20));
            mrm_can.send_frame(MRM_CAN_ID, &mrm_data);
        }
    }
    */

    let mut key_in_ezs = true;
    loop {
        if let Some(f) = mrm_can.read_frame(0x0000) {
            if f[0] == 0x01 {
                key_in_ezs = false;
            } else {
                if !key_in_ezs {
                    agw.wakeup();
                }
                key_in_ezs = true;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
