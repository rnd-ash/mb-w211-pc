use std::{sync::{atomic::{AtomicBool, Ordering, AtomicU64, AtomicU32}, Arc}, time::Duration, process::{Command, self}};

use can_monitor::CanMonitor;
use eframe::{egui::{self, include_image, CentralPanel, ImageSource, Sense, Ui}, emath::Align2, epaint::{Color32, FontId, Pos2, Rect, Vec2}, NativeOptions};
use egui_extras::{StripBuilder, Size};
use futures_util::StreamExt;
use lcp::{Lcp, LcpButton};
use tokio::runtime::Runtime;
use tokio_socketcan::CANFilter;
use w211_can::canbus::CanBus;

mod can_monitor;
mod lcp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonType {
    Static,
    OnOff(bool),
    Level {
        bar_colour: Color32,
        lit_bars: usize,
        total_bars: usize
    },
}

pub struct LowerControlPanelUI {
    fuel: Arc<AtomicU64>,
    fuel_flow: Arc<AtomicU64>,
    monitor: CanMonitor,
    lcp: Lcp,
    eq_running: Arc<AtomicBool>,
    volume: Arc<AtomicU32>,
    runtime: Arc<Runtime>
}

fn make_pair<T>(init: T) -> (Arc<T>, Arc<T>) {
    let c = Arc::new(init);
    (c.clone(), c)
}

pub const AUDIO_OUTPUT: &str = "@DEFAULT_AUDIO_SINK@";

impl LowerControlPanelUI {
    pub fn new(ctx: &egui::Context, runtime: Runtime) -> Self {
        egui_extras::install_image_loaders(ctx);
        let runtime_arc = Arc::new(runtime);

        let can = CanBus::C;
        
        let (fuel, fuel_c) = make_pair(AtomicU64::new(0));
        let (fuel_flow, fuel_flow_c) = make_pair(AtomicU64::new(0));
        let eq_running = Arc::new(AtomicBool::new(false));
        
        runtime_arc.spawn(async move {
            let mut can_c =  can.create_can_socket().unwrap();
            let _ = can_c.set_filter(&[CANFilter::new(0x608, 0xFFF).unwrap()]);
            loop {
                if let Some(Ok(frame)) = can_c.next().await {
                    if frame.id() == 0x608 {
                        let f = ((frame.data()[5] as u16) << 8) | frame.data()[6] as u16; // Consumption over last 250ms
                        fuel_flow_c.store(f as u64, Ordering::Relaxed); // ul/sec
                        fuel_c.fetch_add((f as f64 / 50.0) as u64, Ordering::Relaxed);
                    }
                }
            }
        });

        let volume = Arc::new(AtomicU32::new(0));
        let volume_c = volume.clone();

        std::thread::spawn(move|| {
            loop { 
                if let Ok(output) = process::Command::new("/usr/bin/wpctl")
                    .args(&[
                        "get-volume",
                        AUDIO_OUTPUT
                    ]).output().map(|x| String::from_utf8(x.stdout).unwrap()) {
                        let parts: Vec<&str> = output.split(": ").collect();
                        if let Some(v) = parts.get(1) {
                            let r = v.replace("\n", "");
                            if let Ok(as_int) = r.parse::<f32>() {
                                let v_max = 0.8;
                                let v_now = as_int * 100.0;
                                volume_c.store((v_now / v_max) as u32, Ordering::Relaxed);
                            }
                        }
                    }
                std::thread::sleep(Duration::from_millis(20));
            }
        });

        Self {
            fuel,
            fuel_flow,
            monitor: CanMonitor::new("vcan_b", "vcan_c", runtime_arc.clone()),
            lcp: Lcp::new(runtime_arc.clone(), "vcan_b"),
            eq_running,
            volume,
            runtime: runtime_arc,
        }
    }
}


fn faded_color(color: Color32) -> Color32 {
    use egui::Rgba;
    egui::lerp(Rgba::from(color)..=Rgba::from(Color32::WHITE), 0.8).into()
}

fn big_button(text: &str, ui: &mut Ui, bg_color: Color32, msg: LcpButton, lcp: &mut Lcp) {
    let size = if text.len() <= 3 {
        100.0
    } else {
        30.0
    };
    big_button_with_txt_size(size, text, ui, bg_color, msg, lcp);
}

fn big_image_button(size: f32, icon: ImageSource<'_>, ui: &mut Ui, button_id: LcpButton, lcp: &mut Lcp, mirrored: bool) -> bool {
    let dimens = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(dimens, Sense::click_and_drag());
    if response.is_pointer_button_down_on() {
        lcp.on_press(button_id);
    } else {
        lcp.on_release(button_id);
    }
    let mut ret = false;
    let (c, text_c) = match response.is_pointer_button_down_on()  {
        true => {
            (faded_color(Color32::DARK_GRAY), Color32::DARK_GRAY)
        },
        false => {
            (Color32::DARK_GRAY, Color32::WHITE)
        }
    };

    ui.painter().rect_filled(
        dimens,
        10.0,
        c,
    );
    let disp = lcp.get_state(button_id);
    if let ButtonType::OnOff(on) = disp {
        let bar_y = dimens.center().y + 30.0;
        let bar_dimens = Rect::from_two_pos(
            Pos2::new(dimens.left()+20.0, bar_y),
            Pos2::new(dimens.right()-20.0, bar_y+10.0)
        );
        ui.painter().rect_filled(
            bar_dimens,
            5.0,
            if on { Color32::GREEN } else { Color32::BLACK },
        );
    } else if let ButtonType::Level { bar_colour, lit_bars, total_bars } = disp {
        // Width of each bar
        let bar_w = dimens.width()/4.0;
        let padding = dimens.height()/10.0;

        // Start Y coord to draw from
        let bar_area_y_top = dimens.top() + padding;
        let bar_area_y_bottom = dimens.bottom() - padding;
        let bar_area_x_left = if mirrored {
            dimens.left() + padding
        } else {
            dimens.right() - padding - bar_w
        };
        let bar_area_x_right = if mirrored {
            dimens.left() + padding + bar_w
        } else {
            dimens.right() - padding
        };

        let bar_area = Rect::from_two_pos(Pos2::new(bar_area_x_left, bar_area_y_top), Pos2::new(bar_area_x_right, bar_area_y_bottom));

        let space_per_bar = bar_area.height()/((total_bars+(total_bars-1)) as f32);
        // [BAR, SPACE, BAR, SPACE, BAR]
        let mut draw_order = Vec::new();
        for _ in 0..total_bars {
            draw_order.push(true);
            draw_order.push(false);
        }
        draw_order.remove(draw_order.len()-1);
        let mut bar_id = 0;
        let mut start_y = bar_area.bottom();
        for entry in draw_order {
            if entry {
                let bar_col = if lit_bars >= (bar_id+1) {
                    bar_colour
                } else {
                    Color32::BLACK
                };

                // Drawable bar
                let rect = Rect::from_two_pos(Pos2::new(bar_area_x_left, start_y), Pos2::new(bar_area_x_right, start_y-space_per_bar));
                let rounding = rect.height()/2.0;
                ui.painter().rect_filled(
                    rect,
                    rounding,
                    bar_col,
                );
                bar_id += 1;
            } // else -> Space
            start_y -= space_per_bar;
        }
    }
    let c = dimens.center();
    let icon_space = Vec2::new(dimens.height()*0.75, dimens.height()*0.75);
    let draw_area = Rect::from_center_size(c,icon_space);
    egui::Image::new(icon)
        .paint_at(ui, draw_area);
    ret
}

fn big_button_with_txt_size(size: f32, text: &str, ui: &mut Ui, bg_color: Color32, button_id: LcpButton, lcp: &mut Lcp) {
    let dimens = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(dimens, Sense::click_and_drag());
    if response.is_pointer_button_down_on() {
        lcp.on_press(button_id);
    } else {
        lcp.on_release(button_id);
    }
    let (c, text_c) = match response.is_pointer_button_down_on()  {
        true => {
            (faded_color(bg_color), Color32::DARK_GRAY)
        },
        false => {
            (bg_color, Color32::WHITE)
        }
    };

    ui.painter().rect_filled(
        dimens,
        10.0,
        c,
    );
    let font = FontId::monospace(size);
    ui.painter().text(dimens.center(), Align2::CENTER_CENTER, text, font, text_c);
    let disp = lcp.get_state(button_id);
    if let ButtonType::OnOff(on) = disp {
        let bar_y = dimens.center().y + 30.0;
        let bar_dimens = Rect::from_two_pos(
            Pos2::new(dimens.left()+20.0, bar_y),
            Pos2::new(dimens.right()-20.0, bar_y+10.0)
        );
        ui.painter().rect_filled(
            bar_dimens,
            5.0,
            if on { Color32::GREEN } else { Color32::BLACK },
        );
    } else if let ButtonType::Level { bar_colour, lit_bars, total_bars } = disp {
        let bar_y = dimens.center().y + 30.0;

        let l = dimens.left() + 20.0;
        let r = dimens.right() - 20.0;
        const BAR_PADDING: f32 = 5.0;

        let bar_width = (r-l)/(total_bars as f32) - BAR_PADDING;

        let mut s = l;
        for i in 1..=total_bars {
            let c = if i <= lit_bars {
                bar_colour
            } else {
                Color32::BLACK
            };

            let bar_dimens = Rect::from_two_pos(
                Pos2::new(s, bar_y),
                Pos2::new(s+bar_width, bar_y+10.0)
            );
            s+=bar_width + BAR_PADDING;
            ui.painter().rect_filled(
                bar_dimens,
                5.0,
                c,
            );
        }
    }
}

fn press_key(key: &str) {
    let k = key.clone().to_string();
    std::thread::spawn(move|| {
        Command::new("/usr/bin/xdotool")
            .args(&["key", "--clearmodifiers", "--repeat", "20", "--repeat-delay", "1", &k])
            .env("DISPLAY", ":0")
            .output();
    });
}

fn tap_key(key: &str) {
    let k = key.clone().to_string();
    std::thread::spawn(move|| {
        Command::new("/usr/bin/xdotool")
            .args(&["key", &k])
            .env("DISPLAY", ":0")
            .output();
    });
}

fn hold_key(key: &str, pressed: bool) {
    let k = key.clone().to_string();
    Command::new("/usr/bin/xdotool")
        .args(&[if pressed { "keydown" } else { "keyup" }, &k])
        .env("DISPLAY", ":0")
        .output();
}

impl eframe::App for LowerControlPanelUI {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.185)) // Passenger seats
                .size(Size::relative(0.1)) // Blind
                .size(Size::relative(0.1)) // Headrests
                .size(Size::relative(0.2)) // Hazards
                .size(Size::relative(0.1)) // Lock/unlock
                .size(Size::relative(0.1)) // ESP 
                .size(Size::relative(0.185)) // Driver seats
                .horizontal(|mut strip| {
                    
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_image_button(100.0, include_image!("../resources/seat_heater_l.png"), ui, LcpButton::HeaterL, &mut self.lcp, false);
                                //big_button("SEAT HEATER", ui, Color32::RED, LcpButton::HeaterL, &mut self.lcp)
                            });
                            col.cell(|ui| {
                                big_image_button(100.0, include_image!("../resources/seat_cooler_l.png"), ui, LcpButton::ChillerL, &mut self.lcp, false);
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_image_button(100.0, include_image!("../resources/blind.png"), ui, LcpButton::Blind, &mut self.lcp, false);
                                //big_button("BLIND", ui, Color32::DARK_GRAY, LcpButton::Blind, &mut self.lcp)
                            });
                            col.cell(|ui| {
                                big_button("REBOOT", ui, Color32::RED, LcpButton::Reboot, &mut self.lcp)
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_image_button(100.0, include_image!("../resources/headrests.png"), ui, LcpButton::Headrest, &mut self.lcp, false);
                                //big_button("HEADREST", ui, Color32::DARK_GREEN,LcpButton::Headrest, &mut self.lcp)
                            });
                            col.strip(|s| {
                                s.sizes(Size::remainder(), 2).vertical(|mut c| {
                                    c.cell(|ui| {
                                        let dimens = ui.available_rect_before_wrap();
                                        let center = dimens.center();
                                        ui.painter().text(center, Align2::CENTER_CENTER, format!("VOL {:.3}%", self.volume.load(Ordering::Relaxed)), FontId::monospace(40.0), Color32::WHITE);
                                    });
                                    c.cell(|ui| {
                                        let sto = Arc::new(AtomicBool::new(false));
                                        /*
                                        if big_button_with_txt_size(40.0,"GAME", ui, Color32::GOLD,LcpButton::Game, &mut self.lcp) {
                                            if !self.eq_running.load(Ordering::Relaxed) {
                                                self.eq_running.store(true, Ordering::Relaxed);
                                                let c = self.eq_running.clone();
                                                let c_input = self.eq_running.clone();
                                                std::thread::spawn(move|| {
                                                    let process = std::process::Command::new("/usr/bin/dolphin-emu")
                                                        .args(&["--config","\"Dolphin.Display.Fullscreen=True\"", "-b", "--exec", "/home/mercedes/mario.wbfs"])
                                                        .env("DISPLAY", ":0")
                                                        .output();
                                                    println!("{:?}", process);
                                                    c.store(false, Ordering::Relaxed);
                                                });
                                                std::thread::spawn(move|| {
                                                    println!("Input running");
                                                    // Wii controls are as follows for dolphin
                                                    // ( CUSTOM BINDINGS )
                                                    // A - Q
                                                    // B - B
                                                    // 1 - 1
                                                    // 2 - 2
                                                    // - - Q
                                                    // + - E
                                                    // HOME - ENTER
                                                    //
                                                    // DPAD U - W
                                                    // DPAD D - S
                                                    // DPAD L - A
                                                    // DPAD R - D
                                                    //
                                                    // NUNCHUK DISABLED
                                                    // MOTION EMULATION:
                                                    // FWD - Up
                                                    // BACK - Down
                                                    // LEFT - Left
                                                    // RIGHT - Right

                                                    let canc = w211_can::canbus::CanBus::C;
                                                    let canb = w211_can::canbus::CanBus::B;

                                                    let wrapper = canb.create_can_socket();
                                                    wrapper.set_filters(&[(0x0236, 0xFFFF)]);
                                                    let wrapper_pw = canc.create_can_socket();
                                                    wrapper_pw.set_filters(&[(0x0210, 0xFFFF)]).unwrap();
                                                    let wrapper_brake = canc.create_can_socket();
                                                    wrapper_brake.set_filters(&[(0x0200, 0xFFFF)]).unwrap();
                                                    let wrapper_b = canb.create_can_socket();
                                                    wrapper_b.set_filters(&[(0x1A8, 0xFFFF)]).unwrap();
                                                    let mut pedal = false;
                                                    let mut left = false;
                                                    let mut right = false;
                                                    let mut brake = false;
                                                    let mut plus = false;
                                                    let mut minus = false;
                                                    while c_input.load(Ordering::Relaxed) {
                                                        if let Some((id, (raw, dlc))) = read_recent_frame_as_u64(&wrapper) {
                                                            if (id == LRW_236::get_canid()) {
                                                                let mrm = LRW_236::new(raw);
                                                                let angle = mrm.get_LRW();
                                                                /*
                                                                if angle > 4106 {
                                                                    press_key("Down");
                                                                } else if (angle < 4086) {
                            
                                                                        press_key("Up");
                                                                       
                                                                }
                                                                */
                                                                if angle > 4106 {
                                                                    if !left {
                                                                        hold_key("Down", true);
                                                                        left = true;
                                                                    }
                                                                    if right {
                                                                        hold_key("Up", false);
                                                                        right = false;
                                                                    }
                                                                } else if angle < 4086 {
                                                                    if !right {
                                                                        hold_key("Up", true);
                                                                        right = true;
                                                                    }
                                                                    if left {
                                                                        hold_key("Down", false);
                                                                        left = false;
                                                                    }
                                                                } else {
                                                                    if left {
                                                                        hold_key("Down", false);
                                                                        left = false;
                                                                    }
                                                                    if right {
                                                                        hold_key("Up", false);
                                                                        right = false;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if let Some((id, (raw, dlc))) = read_recent_frame_as_u64(&wrapper_pw) {
                                                            let ms = MS_210::new(raw);
                                                            if ms.get_PW() != 0 {
                                                                if !pedal {
                                                                    hold_key("2", true);
                                                                    pedal = true;
                                                                }
                                                            } else {
                                                                if pedal {
                                                                    hold_key("2", false);
                                                                    pedal = false;
                                                                }
                                                            }
                                                        }
                                                        if let Some((id, (raw, dlc))) = read_recent_frame_as_u64(&wrapper_brake) {
                                                            let ms = BS_200::new(raw);
                                                            if ms.get_BLS().unwrap_or( BS_200h_BLS::BREMSE_NBET) != BS_200h_BLS::BREMSE_NBET {
                                                                if !brake {
                                                                    hold_key("b", true);
                                                                    brake = true;
                                                                }
                                                            } else {
                                                                if brake {
                                                                    hold_key("b", false);
                                                                    brake = false;
                                                                }
                                                            }
                                                        }
                                                        if let Some((id, data)) = read_recent_frame_bytes(&wrapper_b) {
                                                            if (data[0] == 0x10 && data[1] == 0x04) {
                                                                let process = std::process::Command::new("/usr/bin/pkill")
                                                        .args(&["-f", "dolphin-emu"])
                                                        .env("DISPLAY", ":0")
                                                        .output();
                                                                c_input.store(false, Ordering::Relaxed);
                                                            } else if (data[0] == 0x10 && data[1] == 0x0E) { // -
                                                                if !minus {
                                                                    hold_key("d", true);
                                                                    minus = true;
                                                                }
                                                                if plus {
                                                                    hold_key("a", false);
                                                                    plus = false;
                                                                }
                                                            } else if (data[0] == 0x10 && data[1] == 0x0F) { // +
                                                                if !plus {
                                                                    hold_key("a", true);
                                                                    plus = true;
                                                                }
                                                                if minus {
                                                                    hold_key("d", false);
                                                                    minus = false;
                                                                }
                                                            } else if (data[0] == 0x00 && data[1] == 0x00) {
                                                                if plus {
                                                                    hold_key("a", false);
                                                                    plus = false;
                                                                }
                                                                if minus {
                                                                    hold_key("d", false);
                                                                    minus = false;
                                                                }
                                                            }
                                                        }
                                                        std::thread::sleep(Duration::from_millis(10));
                                                    }
                                                    println!("Input Stopped");
                                                    hold_key("2", false);
                                                    
                                                });
                                            }
                                            println!("Cliked!")
                                        }*/
                                    });
                                });
                            });
                        });
                    });
                    strip.cell(|ui| {
                        // Fuel usage
                        let dimens = ui.available_rect_before_wrap();
                        let center = dimens.center();
                        let top = Pos2::new(center.x, center.y-60.0);
                        let bottom = Pos2::new(center.x, center.y+60.0);
                        let flow_rate = if self.fuel_flow.load(Ordering::Relaxed) == 0 {
                            0.0
                        } else {
                            (self.fuel_flow.load(Ordering::Relaxed) as f64 / (1000.0*1000.0)) * 60.0 // L/Min;
                        };
                        ui.painter().text(top, Align2::CENTER_CENTER, "Fuel price 1.32GBP/L", FontId::monospace(25.0), Color32::WHITE);
                        ui.painter().text(center, Align2::CENTER_CENTER, format!("Money rate:\n{:.2} GBP/Min", flow_rate*1.32), FontId::monospace(25.0), Color32::WHITE);
                        ui.painter().text(bottom, Align2::CENTER_CENTER, format!("Fuel rate:\n{:.3} L/Min", flow_rate), FontId::monospace(25.0), Color32::WHITE);


                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                //big_button("LOCK", ui, Color32::DARK_GRAY,LcpButton::Lock, &mut self.lcp)
                                big_image_button(100.0, include_image!("../resources/lock.png"), ui, LcpButton::Lock, &mut self.lcp, false);
                            });
                            col.cell(|ui| {
                                //big_button("UNLOCK", ui, Color32::DARK_GRAY, LcpButton::Unlock, &mut self.lcp)
                                big_image_button(100.0, include_image!("../resources/unlock.png"), ui, LcpButton::Unlock, &mut self.lcp, false);
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_image_button(100.0, include_image!("../resources/esp_off.png"), ui, LcpButton::Esp, &mut self.lcp, false);
                            });
                            col.cell(|ui| {
                                // Net speeds
                                let dimens = ui.available_rect_before_wrap();
                                let center = dimens.center();
                                let top = Pos2::new(center.x, center.y-20.0);
                                let bottom = Pos2::new(center.x, center.y+20.0);
                                ui.painter().text(top, Align2::CENTER_CENTER, format!("C_B: {:.2}kbps", self.monitor.data_rate_b() as f32 / 1000.0), FontId::monospace(20.0), Color32::WHITE);
                                ui.painter().text(bottom, Align2::CENTER_CENTER, format!("C_C: {:.2}kbps", self.monitor.data_rate_c() as f32 / 1000.0), FontId::monospace(20.0), Color32::WHITE);
                            });
                        });
                    });

                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_image_button(100.0, include_image!("../resources/seat_heater_r.png"), ui, LcpButton::HeaterR, &mut self.lcp, true);
                            });
                            col.cell(|ui| {
                                big_image_button(100.0, include_image!("../resources/seat_cooler_r.png"), ui, LcpButton::ChillerR, &mut self.lcp, true);
                            });
                        });
                    });

                })
        });
        ctx.request_repaint();
    }
}

fn main() {
    let mut native_options = NativeOptions::default();
    // 2048x1536
    native_options.vsync = true;

    let mut builder = native_options.viewport;
    builder = builder.with_decorations(false);
    builder = builder.with_max_inner_size(Vec2::new(2048.0, 300.0));
    builder = builder.with_inner_size(Vec2::new(2048.0, 300.0));
    native_options.viewport = builder;

    let _ = eframe::run_native(
        "Lower control panel emulator",
        native_options,
        Box::new(|cc| {
            let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build().unwrap();

            let app = LowerControlPanelUI::new(&cc.egui_ctx, rt);
            Box::new(app)
        }),
    );
}
