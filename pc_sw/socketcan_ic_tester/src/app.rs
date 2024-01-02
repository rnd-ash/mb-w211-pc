use std::{time::Duration, fmt::format};

use agw_lib::{agw::{AgwEmulator, self, TextFmtFlags, navigation::{DistanceDisplay, NaviPageCmd, DistanceUnit}, AgwCommand}, w211_can::{self, socketcan::{CanSocket, Socket}, canb::{MRM_A1, MRM_A2}}, custom_display_format::CDMIsoTp};
use eframe::egui::{CentralPanel, DragValue, ComboBox};
use agw_lib::agw::audio::AudioCfgSettings;

pub struct App {
    agw: AgwEmulator,
    canb: CanSocket,
    audio_body: String,
    audio_header: String,
    // Navi info
    distance: DistanceDisplay,
    current_road: String,
    target_road: String,
    custom_shown: bool,
    custom_buffer: String,
    audio_body_flags: TextFmtFlags
}

impl App {
    pub fn new(can: String) -> Self {
        let vlad = CDMIsoTp::new(can.clone());

        let audio_settings = AudioCfgSettings {
            auto_scroll: false,
        };

        let agw = AgwEmulator::new(can.clone(), vlad, audio_settings);

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
            custom_buffer: "HELLO".into(),
            custom_shown: false,
            audio_body_flags: TextFmtFlags::empty()
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

            if ui.button("Vol+").clicked() {
                mrm.set_WIPPE_3_1(true);
            }
            if ui.button("Vol-").clicked() {
                mrm.set_WIPPE_3_2(true);
            }

            if mrm.0 != 0 {
                self.custom_shown = false;
                for i in 0..2 {
                    let f = w211_can::canbus::u64_to_frame(MRM_A2::get_canid(), mrm.0, 2);
                    self.canb.write_frame(&f);
                    std::thread::sleep(Duration::from_millis(20));
                }
                for i in 0..2 {
                    let f = w211_can::canbus::u64_to_frame(MRM_A2::get_canid(), 0, 2);
                    self.canb.write_frame(&f);
                    std::thread::sleep(Duration::from_millis(20));
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

                    r.collapsing("Format", |ui| {

                        let mut x = self.audio_body_flags.contains(TextFmtFlags::LEFT);
                        ui.checkbox(&mut x, "Left");
                        self.audio_body_flags.set(TextFmtFlags::LEFT, x);

                        x = self.audio_body_flags.contains(TextFmtFlags::CENTER);
                        ui.checkbox(&mut x, "Center");
                        self.audio_body_flags.set(TextFmtFlags::CENTER, x);

                        x = self.audio_body_flags.contains(TextFmtFlags::RIGHT);
                        ui.checkbox(&mut x, "Right");
                        self.audio_body_flags.set(TextFmtFlags::RIGHT, x);

                        x = self.audio_body_flags.contains(TextFmtFlags::FLASH);
                        ui.checkbox(&mut x, "Flashing");
                        self.audio_body_flags.set(TextFmtFlags::FLASH, x);

                        x = self.audio_body_flags.contains(TextFmtFlags::HIGHLIGHT);
                        ui.checkbox(&mut x, "Highlighted");
                        self.audio_body_flags.set(TextFmtFlags::HIGHLIGHT, x);

                    });

                    if r.button("Send").clicked() {
                        self.agw.send_agw_command(agw::AgwCommand::SetAudioBodyText(
                            agw::IcText { format: self.audio_body_flags, text: self.audio_body.clone() }
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

            ui.collapsing("CUSTOM PAGE", |ui| {
                let now = self.custom_buffer.clone();
                ui.text_edit_singleline(&mut self.custom_buffer);

                let check_res = check_custom_string(&self.custom_buffer);
                if now != self.custom_buffer && self.custom_shown {
                    // Live update buffer
                    println!("{}", self.custom_buffer.clone());
                    if check_res.is_ok() {
                        self.agw.send_agw_command(AgwCommand::UpdateCustomDisplay(self.custom_buffer.clone()));
                    }
                    //self.agw.send_agw_command(AgwCommand::ShowCustomDisplay);
                }
                if let Err(e) = check_res {
                    ui.label(format!("String error: {e}"));
                }

                let shown_now = self.custom_shown;
                ui.checkbox(&mut self.custom_shown, "Show display");
                if shown_now != self.custom_shown {
                    match self.custom_shown {
                        true => self.agw.send_agw_command(AgwCommand::ShowCustomDisplay),
                        false => self.agw.send_agw_command(AgwCommand::HideCustomDisplay)
                    }
                }
            });

            
        });
    }
}

fn check_command(c: char, exp_len: usize, buffer: &str) -> Result<(), String> {
    if buffer.len() < exp_len+1 {
        return Err(format!("Command '{}' expects {} digit hex, found {}", c, exp_len, buffer.len() - 1))
    } else {
        let matching = &buffer[1..exp_len+1];
        u32::from_str_radix(matching, 16)
            .map(|_| ())
            .map_err(|_| format!("Arg '{}' for command '{}' is not valid hex", matching, c))
    }
}

pub fn check_custom_string(s: &str) -> Result<(), String> {
    let commands = s.split("~").collect::<Vec<&str>>();
    if commands.len() == 0 {
        return Err("Empty command string".into());
    }
    if !commands[0].is_empty() {
        return Err("String must start with format command string".into());
    }
    for command in &commands[1..] {
        if command.len() == 0 {
            return Err("Empty command".into());
        }
        let res = match command.chars().next().unwrap() {
            'P' => check_command('P', 4, command), // Pos xx yy
            'B' => check_command('B', 3, command), // Image xxx
            'N' => check_command('N', 3, command), // Navi image xxx
            'S' => check_command('S', 3, command), // String ID xxx
            'Q' => check_command('Q', 3, command), // Rect. Coords xxx
            'I' => check_command('I', 1, command), // Line state
            'C' => check_command('C', 1, command), // Screen clear
            'U' => check_command('U', 1, command), // Screen colour
            'G' => check_command('G', 1, command), // Font ID
            '*' => check_command('*', 3, command), // Radio str xxx
            'T' => check_command('T', 1, command), // Draw flags
            'Y' => check_command('Y', 1, command), // ART ring flags

            'F' => check_command('F', 2, command), // ??
            'H' => check_command('H', 2, command), // Height override
            'J' => check_command('J', 1, command),

            '=' => check_command('=', 2, command), // 
            '@' => check_command('@', 2, command), // 

            '<' => check_command('<', 4, command), // 
            '-' => check_command('-', 4, command), //
            'V' => check_command('V', 4, command), // 

            'Z' | 'R' | '/' | 'A' | 'E' => Ok(()), // Format commands (Standalone)
            x => Err(format!("Unknown command {x}"))
        };
        if res.is_err() {
            return res;
        }
    }
    Ok(())
}