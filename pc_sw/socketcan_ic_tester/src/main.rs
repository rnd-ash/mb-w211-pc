use std::{time::Duration, thread};

use agw_lib::w211_can::{self, canb::EZS_A1, canbus::u64_to_frame};

use w211_can::socketcan::*;
use clap::Parser;
use eframe::{App, NativeOptions};

mod app;

#[derive(Debug, Parser)]
pub struct TesterSettings {
    can_if: String
}

fn main() {
    let settings = TesterSettings::parse();

    let app = app::App::new(settings.can_if.clone());

    let na = NativeOptions::default();

    let can = w211_can::canbus::CanBus::create_can_socket_with_name(&settings.can_if);
    // EZS awake frame

    let mut ezs_a1 = EZS_A1::new(0);
    ezs_a1.set_KL_15R_EIN(true);
    ezs_a1.set_KL_15C_EIN(true);
    ezs_a1.set_KL_15X_EIN(true);
    ezs_a1.set_KL_15_EIN(true);
    let data = u64_to_frame(EZS_A1::get_canid(), ezs_a1.0, 8);

    std::thread::spawn(move|| {
        loop {
            thread::sleep(Duration::from_millis(200));
            let _ = can.write_frame(&data);
        }
    });

    eframe::run_native("IC AGW Tester", na, Box::new(|cc| Box::new(app)));
}

