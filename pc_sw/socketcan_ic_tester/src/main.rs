use std::{time::Duration, thread, cmp::{max, min}};

use agw_lib::{w211_can::{self, canb::EZS_A1, canbus::u64_to_frame}, custom_display_format::CDMIsoTp};

//use tetris::Tetris;
use clap::Parser;
use eframe::{App, NativeOptions};

use agw_lib::custom_display_format::{ToneRepeatType, ToneType};

mod app;
//mod tetris;
//mod blocks;

#[derive(Debug, Parser)]
pub struct TesterSettings {
    can_if: String
}

#[tokio::main]
async fn main() {
    let settings = TesterSettings::parse();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let app = app::App::new(&runtime, settings.can_if.clone());

    let na = NativeOptions::default();

    let can = w211_can::canbus::CanBus::create_can_socket_with_name(&settings.can_if).unwrap();
    // EZS awake frame

    let mut ezs_a1 = EZS_A1::new(0);
    ezs_a1.set_KL_15R_EIN(true);
    ezs_a1.set_KL_15C_EIN(true);
    ezs_a1.set_KL_15X_EIN(true);
    ezs_a1.set_KL_15_EIN(true);
    let data = u64_to_frame(EZS_A1::get_canid(), ezs_a1.0, 8);

    tokio::spawn(async move {
        loop {
            let _ = can.write_frame(data).unwrap().await;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });

    eframe::run_native("IC AGW Tester", na, Box::new(|cc| Box::new(app)));
    /*
    // WARNING
    let mut vlad = CDMIsoTp::new(settings.can_if);

    vlad.sound_buzzer(ToneType::LongBeep, ToneRepeatType::Middle);
    std::thread::sleep(Duration::from_millis(80));
    vlad.show_display(999999);
    for i in 0..10 {
        vlad.update_buffer_live("~C2~P0000~J2~Z~G2WARNING!!!~L~L~G1Danger to Manifold~P7890");
        std::thread::sleep(Duration::from_millis(750));
        vlad.update_buffer_live("~C2");
        std::thread::sleep(Duration::from_millis(250));
    }
    vlad.stop_display();
    std::thread::sleep(Duration::from_millis(40));

    vlad.stop_buzzer();
    */
    // Tetris
    /*
    let mut vlad = CDMIsoTp::new(settings.can_if);
    let mut tetris = Tetris::new(vlad);
    tetris.run();
    */
}

