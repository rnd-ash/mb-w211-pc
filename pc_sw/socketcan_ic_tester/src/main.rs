use agw_lib::w211_can;
use clap::Parser;
use eframe::{App, NativeOptions};

mod app;

#[derive(Debug, Parser)]
pub struct TesterSettings {
    can_if: String
}

fn main() {
    let settings = TesterSettings::parse();

    let app = app::App::new(settings.can_if);

    let na = NativeOptions::default();

    eframe::run_native("IC AGW Tester", na, Box::new(|cc| Box::new(app)));
}

