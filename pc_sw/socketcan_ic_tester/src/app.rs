use std::{borrow::Borrow, fmt::format, sync::Arc, time::Duration};

use agw_lib::{agw::{self, navigation::{DistanceDisplay, DistanceUnit, NaviPageCmd}, AgwCommand, AgwEmulator, TextFmtFlags}, custom_display_format::{CDMIsoTp, ToneRepeatType, ToneType}, w211_can::{self, canb::{MRM_A1, MRM_A2}, tokio_socketcan::CANSocket}};
use eframe::egui::{CentralPanel, DragValue, ComboBox};
use agw_lib::agw::audio::AudioCfgSettings;
use tokio::runtime::Runtime;

//use crate::tetris::{GAME_GRID, GAME_W, GAME_H};

pub struct App {
    agw: AgwEmulator,
    canb: Arc<CANSocket>,
    audio_body: String,
    audio_header: String,
    // Navi info
    distance: DistanceDisplay,
    current_road: String,
    target_road: String,
    custom_shown: bool,
    custom_buffer: String,
    audio_body_flags: TextFmtFlags,
    tone: ToneType,
    tone_repeat: ToneRepeatType,
    tone_running: bool
}

impl App {
    pub fn new(runtime: &Runtime, can: String) -> Self {


        let vlad: CDMIsoTp = CDMIsoTp::new(&runtime, can.clone());

        let audio_settings = AudioCfgSettings {
            auto_scroll: false,
        };

        let agw = AgwEmulator::new(&runtime, can.clone(), vlad, audio_settings);

        let canb = w211_can::canbus::CanBus::create_can_socket_with_name(&can).unwrap();
        println!("CAN name is {}", can);
        Self{
            canb: Arc::new(canb),
            agw,
            audio_body: String::default(),
            audio_header: String::default(),
            distance: DistanceDisplay::default(),
            current_road: "This road".into(),
            target_road: "That road".into(),
            custom_buffer: "HELLO".into(),
            custom_shown: false,
            audio_body_flags: TextFmtFlags::empty(),
            tone: ToneType::Chime,
            tone_repeat: ToneRepeatType::None,
            tone_running: false
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
                let canb = self.canb.clone();
                tokio::spawn(async move {
                    for _ in 0..2 {
                        let f = w211_can::canbus::u64_to_frame(MRM_A2::get_canid(), mrm.0, 2);
                        canb.write_frame(f).unwrap().await;
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                    for _ in 0..2 {
                        let f = w211_can::canbus::u64_to_frame(MRM_A2::get_canid(), 0, 2);
                        canb.write_frame(f).unwrap().await;
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                });
            }
            let w = ui.available_width();
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
                ui.code_editor(&mut self.custom_buffer);
                ui.separator();
                ui.strong("Command Syntax");
                let syntax_parse_res = generate_syntax_from_custom_string(&self.custom_buffer);
                match syntax_parse_res.borrow() {
                    Ok(commands) => {
                        for (idx, cmd) in commands.iter().enumerate() {
                            ui.label(format!("{}. {}", idx+1, cmd));
                        }
                    },
                    Err(e) => {
                        ui.label(format!("String error: {e}"));
                    },
                }
                ui.separator();

                if now != self.custom_buffer && self.custom_shown {
                    // Live update buffer
                    println!("{}", self.custom_buffer.clone());
                    if syntax_parse_res.is_ok() {
                        self.agw.send_agw_command(AgwCommand::ShowCustomDisplay(self.custom_buffer.clone(), u32::MAX));
                    }
                    //self.agw.send_agw_command(AgwCommand::ShowCustomDisplay);
                }
                

                let shown_now = self.custom_shown;
                ui.checkbox(&mut self.custom_shown, "Show display");
                if shown_now != self.custom_shown {
                    match self.custom_shown {
                        true => self.agw.send_agw_command(AgwCommand::ShowCustomDisplay(self.custom_buffer.clone(), u32::MAX)),//self.agw.send_agw_command(AgwCommand::ShowCustomDisplay),
                        false => self.agw.send_agw_command(AgwCommand::HideCustomDisplay)
                    }
                }

                if self.tone_running {
                    if ui.button("Stop tone").clicked() {
                        self.tone_running = false;
                        self.agw.send_agw_command(AgwCommand::StopBuzzer)
                    }
                } else {
                    ui.horizontal(|row| {
                        row.selectable_value(&mut self.tone, ToneType::Chime, "Chime");
                        row.selectable_value(&mut self.tone, ToneType::ShortBeep, "Short beep");
                        row.selectable_value(&mut self.tone, ToneType::LongBeep, "Long beep");
                    });
                    ui.horizontal(|row| {
                        row.selectable_value(&mut self.tone_repeat, ToneRepeatType::None, "No repeat");
                        row.selectable_value(&mut self.tone_repeat, ToneRepeatType::Slow, "Slow");
                        row.selectable_value(&mut self.tone_repeat, ToneRepeatType::Middle, "Medium");
                        row.selectable_value(&mut self.tone_repeat, ToneRepeatType::Fast, "Fast");
                    });
                    if ui.button("Test tone").clicked() {
                        if self.tone_repeat != ToneRepeatType::None {
                            self.tone_running = true;
                        }
                        self.agw.send_agw_command(AgwCommand::SoundBuzze(self.tone, self.tone_repeat));
                    }
                }
            });

            
        });
    }
}

fn check_command(c: char, exp_len: usize, buffer: &str) -> Result<Vec<String>, String> {
    let mut syntax_list = Vec::new();
    if buffer.len() < exp_len+1 && exp_len != 0 {
        return Err(format!("Command '{}' expects {} digit hex, found {}", c, exp_len, buffer.len() - 1))
    } else {
        let arg_as_int = if exp_len == 0 {
            0
        } else {
            let matching = &buffer[1..exp_len+1];
            u32::from_str_radix(matching, 16)
                .map_err(|_| format!("Arg '{}' for command '{}' is not valid hex", matching, c))?
        };
        syntax_list.push(match c {
            'B' => format!("Draw image ID {arg_as_int}"),
            'C' => format!("Refresh display. Arg {arg_as_int:01X}"),
            'E' => format!("Set ASCII table to table ID '{arg_as_int}'"),
            'F' => {
                if arg_as_int == 0 {
                    format!("-- End of function apply")
                } else {
                    format!("-- Start of function apply (Function ID {arg_as_int})")
                }
            }
            'G' => format!("Set font to ID '{arg_as_int}'"),
            'H' => format!("Override text height to {arg_as_int} pixels"),
            'I' => {
                let mut s = format!("Status lines - Set rows to clear: ");
                let mut v = Vec::new();
                if arg_as_int & 0b1 != 0 {
                    v.push("Temperature")
                }
                if arg_as_int & 0b10 != 0 {
                    v.push("Trip")
                }
                if arg_as_int & 0b100 != 0 {
                    v.push("Odometer")
                }
                if arg_as_int & 0b100 != 0 {
                    v.push("Gear display")
                }
                s.push_str(&format!("{v:?}"));
                s
            },
            'L' => format!("New line"),
            'N' => format!("Draw navigation image ID {arg_as_int}"),
            'P' => {
                let x = u16::from_str_radix(&buffer[1..3], 16).unwrap();
                let y = u16::from_str_radix(&buffer[3..5], 16).unwrap();
                format!("Set cursor to position ({x},{y})")
            },
            'Q' => {
                let x = u16::from_str_radix(&buffer[1..3], 16).unwrap();
                let y = u16::from_str_radix(&buffer[3..5], 16).unwrap();
                format!("Draw rectange from previous cursor position to ({x},{y})")
            }
            'R' => format!("Set text justification to right"),
            'S' => format!("Show emedded string ID {arg_as_int}"),
            'T' => {
                let mut s = format!("Override drawing of next element: ");
                let mut v = Vec::new();
                if arg_as_int == 0 {
                    v.push("Normal draw")
                } else {
                    if arg_as_int & 0b1 != 0 {
                        v.push("Hide element")
                    }
                    if arg_as_int & 0b10 != 0 {
                        v.push("Highlight")
                    }
                    if arg_as_int & 0b100 != 0 {
                        v.push("Inver background")
                    }
                }
                s.push_str(&format!("{v:?}"));
                s
            }
            'V' => {
                let x = u16::from_str_radix(&buffer[1..3], 16).unwrap();
                let y = u16::from_str_radix(&buffer[3..5], 16).unwrap();
                format!("Draw line from previous cursor position to ({x},{y})")
            }
            'X' => format!("Move cursor horizontally by {arg_as_int} pixels"),
            'Z' => format!("Set text justification to center"),
            _ => format!("Command ~{c}")
        });
        if buffer.len() > exp_len + 1 {
            // Text string
            syntax_list.push(format!("Draw text '{}'", &buffer[exp_len+1..]))
        }

        Ok(syntax_list)
    }
}

pub fn generate_syntax_from_custom_string(s: &str) -> Result<Vec<String>, String> {
    let commands = s.split("~").collect::<Vec<&str>>();
    if commands.len() == 0 {
        return Err("Empty command string".into());
    }
    if !commands[0].is_empty() {
        return Err("String must start with format command string".into());
    }
    let mut syntax_list = Vec::new();
    for command in &commands[1..] {
        if command.len() == 0 {
            return Err("Empty command".into());
        }
        let c = command.chars().next().unwrap();
        let command_len = match c {
            //'E' | 'L' | 'R' | 'Z' | '/' => Ok(0),
            'C' | 'G' | 'I' | 'J' | 'O' | 'T'| '>' | ';' => Ok::<u32, String>(1),
            'F' | 'H' | '=' | '@' | 'X' => Ok(2),
            'B' | 'N' | 'S' => Ok(3),
            'D' | 'P' | 'Q' | 'V' | '<' | '-' | '?' => Ok(4),
            _ => Ok(0)
        }?;
        if command_len != 0 {
            // Check size
            for c in check_command(c, command_len as usize, command)? {
                syntax_list.push(c);
            }
        }
    }
    Ok(syntax_list)
}