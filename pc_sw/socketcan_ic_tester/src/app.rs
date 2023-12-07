use std::time::Duration;

use agw_lib::{agw::{AgwEmulator, self, TextFmtFlags, navigation::{DistanceDisplay, NaviPageCmd, DistanceUnit}}, w211_can::{self, socketcan::{CanSocket, Socket}, canb::{MRM_A1, MRM_A2}}, custom_display_format::CDMIsoTp};
use eframe::egui::{CentralPanel, DragValue, ComboBox};

pub struct App {
    agw: AgwEmulator,
    canb: CanSocket,
    audio_body: String,
    audio_header: String,
    // Navi info
    distance: DistanceDisplay,
    current_road: String,
    target_road: String
}

impl App {
    pub fn new(can: String) -> Self {

        let vlad_socket = w211_can::canbus::CanBus::create_isotp_socket_with_name(&can, 0x3E1, 0x1A1, 50, 8);
        let agw_socket = w211_can::canbus::CanBus::create_isotp_socket_with_name(&can, 0x1D0, 0x1A4, 50, 0);

        let vlad = CDMIsoTp::new(can.clone());

        let agw = AgwEmulator::new(can.clone(), vlad);

        let canb = w211_can::canbus::CanBus::create_can_socket_with_name(&can);
        println!("CAN name is {}", can);
        Self{
            canb,
            agw,
            audio_body: String::default(),
            audio_header: String::default(),
            distance: DistanceDisplay::default(),
            current_road: "This road".into(),
            target_road: "That road".into(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Navigation buttons");
            
            let mut mrm = MRM_A2::default();
            if ui.button("Up").clicked() {
                mrm.set_WIPPE_1_1(true);
            }
            if ui.button("Down").clicked() {
                mrm.set_WIPPE_1_2(true);
            }
            if ui.button("Page+").clicked() {
                mrm.set_WIPPE_2_1(true);
            }
            if ui.button("Page-").clicked() {
                mrm.set_WIPPE_2_2(true);
            }

            if mrm.0 != 0 {
                for i in 0..2 {
                    let f = w211_can::canbus::u64_to_frame(MRM_A2::get_canid(), mrm.0, 2);
                    self.canb.write_frame(&f);
                    std::thread::sleep(Duration::from_millis(10));
                }
                for i in 0..2 {
                    let f = w211_can::canbus::u64_to_frame(MRM_A2::get_canid(), 0, 2);
                    self.canb.write_frame(&f);
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
            ui.collapsing("AUDIO PAGE", |ui| {
                ui.horizontal(|r| {
                    r.label("AUDIO Header:");
                    r.text_edit_singleline(&mut self.audio_header);
                    if r.button("Send").clicked() {
                        self.agw.send_agw_command(agw::AgwCommand::SetAudioHeaderText(
                            agw::IcText { format: TextFmtFlags::empty(), text: self.audio_header.clone() }
                        ))
                    }
                });
                ui.horizontal(|r| {
                    r.label("AUDIO Body:");
                    r.text_edit_singleline(&mut self.audio_body);
                    if r.button("Send").clicked() {
                        self.agw.send_agw_command(agw::AgwCommand::SetAudioBodyText(
                            agw::IcText { format: TextFmtFlags::empty(), text: self.audio_body.clone() }
                        ))
                    }
                });
            });

            ui.collapsing("NAVI PAGE", |ui| {
                let mut cmd = None;
                ui.horizontal(|r| {
                    r.label("Current road:");
                    r.text_edit_singleline(&mut self.current_road);
                    if r.button("Send").clicked() {
                        cmd = Some(NaviPageCmd::CurrentRoad(self.current_road.clone()));
                    }
                });
                ui.horizontal(|r| {
                    r.label("Target road:");
                    r.text_edit_singleline(&mut self.target_road);
                    if r.button("Send").clicked() {
                        cmd = Some(NaviPageCmd::TargetRoad(self.current_road.clone()));
                    }
                });

                let distance_now = self.distance;
                ui.label("Heading info");
                ui.checkbox(&mut self.distance.show_text, "Show distance text");
                if self.distance.show_text {
                    ComboBox::new("dist_unit_sel", "Distance unit")
                        .selected_text(format!("{:?}", self.distance.unit))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.distance.unit, DistanceUnit::Ft, "Feet");
                            ui.selectable_value(&mut self.distance.unit, DistanceUnit::Mi, "Mile");

                            ui.selectable_value(&mut self.distance.unit, DistanceUnit::M, "Meter");
                            ui.selectable_value(&mut self.distance.unit, DistanceUnit::Km, "Kilometer");
                        });

                    ui.label("Distance value:");
                    ui.add(DragValue::new(&mut self.distance.distance).clamp_range(0..=1000));
                    ui.checkbox(&mut self.distance.show_bar, "Show distance bar");
                    if self.distance.show_bar {
                        ui.label("Bar fill:");
                        ui.add(DragValue::new(&mut self.distance.bar_fill).clamp_range(0..=0xFF));
                    } else {
                        self.distance.bar_fill = 0;
                    }
                }
                if self.distance != distance_now {
                    cmd = Some(NaviPageCmd::DistanceData(self.distance));
                }

                if let Some(navi_cmd) = cmd {
                    self.agw.send_agw_command(agw::AgwCommand::SendNaviData(navi_cmd));
                }
            });
        });
    }
}