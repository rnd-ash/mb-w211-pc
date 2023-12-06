use std::{
    io::{stdin, BufRead},
    time::{Duration, Instant},
};

pub mod agw;
use custom_display_format::CDMIsoTp;
use w211_can::{*, canbus::{CanBus, frame_to_u64}, socketcan_isotp::{Id, StandardId}, canb::EZS_A1, socketcan::{SocketOptions, CanFilter, Socket}};

const EZS_A1_ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(EZS_A1::get_canid()) });

pub mod custom_display_format;

fn main() {
    env_logger::init();
    let can_name = CanBus::B.get_net_name().to_string(); // Runs on bus B
    let agw_socket = w211_can::canbus::CanBus::B.create_isotp_socket(0x1A4, 0x1D0, 50, 0);

    let mut vlad = CDMIsoTp::new(can_name.clone());
    let agw = agw::AgwEmulator::new(can_name, vlad);
    let mut next_down = false;
    let mut prev_down = false;
    let mut stdin = stdin();

    let ezs_can = CanBus::B.create_can_socket();
    ezs_can.set_filters(&[CanFilter::new(EZS_A1::get_canid() as u32, 0xFFF)]);

    let mut key_in_ezs = true;

    while let Ok(frame) = ezs_can.read_frame() {
        let wrapped = EZS_A1::new(frame_to_u64(&frame).0);
        if wrapped.get_KL_15R_EIN() {
            if !key_in_ezs {
                agw.wakeup();
            }
            key_in_ezs = true;
        } else {
            key_in_ezs = false
        }
    }
}
