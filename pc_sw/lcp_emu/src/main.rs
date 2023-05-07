use std::{sync::{atomic::{AtomicBool, Ordering, AtomicU8}, Arc}, time::{Duration, Instant}};

use eframe::{NativeOptions, epaint::{Vec2, Color32, FontId, Shape, PathShape, Stroke, Mesh, TextureId}, egui::{Button, CentralPanel, Sense, self, Ui}, emath::Align2};
use egui_extras::{StripBuilder, Size};
mod w211can;

#[derive(Default)]
pub struct LowerControlPanelUI {
    heater_right: Arc<AtomicBool>,
    heater_right_status: Arc<AtomicU8>,

    heater_left: Arc<AtomicBool>,
    heater_left_status: Arc<AtomicU8>,

    chiller_right: Arc<AtomicBool>,
    chiller_right_status: Arc<AtomicU8>,

    chiller_left: Arc<AtomicBool>,
    chiller_left_status: Arc<AtomicU8>,

    blind: Arc<AtomicBool>,
    headrests: Arc<AtomicBool>,
    lock: Arc<AtomicBool>,
    unlock: Arc<AtomicBool>,
    hazards: Arc<AtomicBool>,
    hazards_active: Arc<AtomicBool>,
    esp: Arc<AtomicBool>,
}

fn make_pair() -> (Arc<AtomicBool>, Arc<AtomicBool>) {
    let c = Arc::new(AtomicBool::new(false));
    (c.clone(), c)
}

fn make_pair_status() -> (Arc<AtomicU8>, Arc<AtomicU8>) {
    let c = Arc::new(AtomicU8::new(0xFF));
    (c.clone(), c)
}

impl LowerControlPanelUI {
    pub fn new() -> Self {
        let can = w211can::CanBus::B;
        let can_b = loop {
            if let Ok(c)= can.create_can_socket(&[0x230]) {
                break c;
            } else {
                std::thread::sleep(Duration::from_millis(100));
            }
        };

        let can = w211can::CanBus::C;
        let can_c = loop {
            if let Ok(c)= can.create_can_socket(&[0x00E]) {
                break c;
            } else {
                std::thread::sleep(Duration::from_millis(100));
            }
        };

        let (heater_right, heater_right_c) = make_pair();
        let (heater_left, heater_left_c) = make_pair();
        let (chiller_right, chiller_right_c) = make_pair();
        let (chiller_left, chiller_left_c) = make_pair();
        let (blind, blind_c) = make_pair();
        let (headrests, headrests_c) = make_pair();
        let (lock, lock_c) = make_pair();
        let (unlock, unlock_c) = make_pair();
        let (hazards, hazards_c) = make_pair();
        let (hazards_active, hazards_active_c) = make_pair();
        let (esp, esp_c) = make_pair();

        let (heater_right_status, heater_right_status_c) = make_pair_status();
        let (heater_left_status, heater_left_status_c) = make_pair_status();
        let (chiller_right_status, chiller_right_status_c) = make_pair_status();
        let (chiller_left_status, chiller_left_status_c) = make_pair_status();

        std::thread::spawn(move|| {
            const OBF_CAN_ID: u16 = 0x2C;
            let mut hazards_on = false;
            let mut hazards_changed = false;
            let mut last_tx_time = Instant::now();
            let mut hazards_on_time: u8 = 0;
            let mut hazards_update: Instant = Instant::now();
            loop {
                if last_tx_time.elapsed().as_millis() > 80 {
                    let mut data: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
                    if blind_c.load(Ordering::Relaxed) {
                        data[0] |= 0x20;
                    }
                    if headrests_c.load(Ordering::Relaxed) {
                        data[0] |= 0x08;
                    }
                    if lock_c.load(Ordering::Relaxed) {
                        data[0] |= 0x80;
                    }
                    if unlock_c.load(Ordering::Relaxed) {
                        data[0] |= 0x40;
                    }
                    if esp_c.load(Ordering::Relaxed) {
                        data[0] |= 0x02;
                    }

                    if hazards_c.load(Ordering::Relaxed) {
                        if !hazards_changed {
                            hazards_changed = true;
                            hazards_on = !hazards_on;
                        }
                    } else {
                        hazards_changed = false;
                    }
                    if (hazards_on) {
                        // This is a toggle!
                        data[1] |= 0x20;
                    }

                    if heater_right_c.load(Ordering::Relaxed) {
                        data[2] |= 0x80;
                    }
                    if chiller_right_c.load(Ordering::Relaxed) {
                        data[2] |= 0x40;
                    }
                    if heater_left_c.load(Ordering::Relaxed) {
                        data[2] |= 0x08;
                    }
                    if chiller_left_c.load(Ordering::Relaxed) {
                        data[2] |= 0x04;
                    }
                    can_b.send_frame(OBF_CAN_ID, &data);
                    last_tx_time = Instant::now();
                }
                if let Some(frame) = can_b.read_frame(0x230) {
                    hazards_active_c.store(frame[0] == 0x20, Ordering::Relaxed);//0ffset 2
                    hazards_on_time = frame[1];
                    hazards_update = Instant::now();
                }
                if hazards_active_c.load(Ordering::Relaxed) && hazards_update.elapsed().as_millis() > hazards_on_time as u128 {
                    hazards_active_c.store(false, Ordering::Relaxed);
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        });

        Self {
            heater_right,
            heater_right_status,
            heater_left,
            heater_left_status,
            chiller_right,
            chiller_right_status,
            chiller_left,
            chiller_left_status,
            blind,
            headrests,
            lock,
            unlock,
            hazards,
            hazards_active,
            esp,
        }
    }


    pub fn make_seat_status_text(&self, x: &Arc<AtomicU8>) -> String {
        match x.load(Ordering::Relaxed) {
            0 => "OFF",
            1 => "ON(1)",
            2 => "ON(2)",
            3 => "ON(3)",
            _ => "??"
        }.to_string()
    }

}


fn faded_color(color: Color32) -> Color32 {
    use egui::Rgba;
    egui::lerp(Rgba::from(color)..=Rgba::from(Color32::WHITE), 0.8).into()
}

fn big_button(text: &str, ui: &mut Ui, bg_color: Color32, store: &Arc<AtomicBool>) {
    let dimens = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(dimens, Sense::click_and_drag());
    store.store(response.is_pointer_button_down_on(), Ordering::Relaxed);
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
    let font = if text.len() <= 3 {
        FontId::monospace(100.0)
    } else {
        FontId::monospace(30.0)
    };
    ui.painter().text(dimens.center(), Align2::CENTER_CENTER, text, font, text_c);
}

fn hazards_button(ui: &mut Ui, lights_on: &Arc<AtomicBool>, store: &Arc<AtomicBool>) {
    let dimens = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(dimens, Sense::click_and_drag());

    store.store(response.is_pointer_button_down_on(), Ordering::Relaxed);
    let bg_color = match lights_on.load(Ordering::Relaxed) {
        true => Color32::RED,
        false => Color32::DARK_RED
    };
    let (c, text_c) = match response.is_pointer_button_down_on()  {
        true => {
            (faded_color(bg_color), Color32::DARK_GRAY)
        },
        false => {
            (bg_color, Color32::WHITE)
        }
    };

    ui.painter().add(
        Shape::Path(
            PathShape::closed_line(
                vec![
                    dimens.left_bottom(),
                    dimens.right_bottom(),
                    dimens.center_top()
                ], 
                Stroke::new(10.0, c))
        )
    );
    ui.painter().text(dimens.center(), Align2::CENTER_CENTER, "HAZARDS", FontId::monospace(30.0), text_c);
}

impl eframe::App for LowerControlPanelUI {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
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
                                big_button(&format!("SEAT HEATER - {}", self.make_seat_status_text(&self.heater_right_status)), ui, Color32::RED, &self.heater_left)
                            });
                            col.cell(|ui| {
                                big_button(&format!("SEAT COOLER - {}", self.make_seat_status_text(&self.chiller_right_status)), ui, Color32::BLUE,&self.chiller_left)
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("BLIND", ui, Color32::DARK_GRAY, &self.blind)
                            });
                            col.cell(|ui| {
                                
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("HEADREST", ui, Color32::DARK_GREEN,&self.headrests)
                            });
                            col.cell(|ui| {
                                
                            });
                        });
                    });
                    strip.cell(|ui| {
                        hazards_button( ui, &self.hazards_active, &self.hazards)
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("LOCK", ui, Color32::DARK_GRAY,&self.lock)
                            });
                            col.cell(|ui| {
                                big_button("UNLOCK", ui, Color32::DARK_GRAY, &self.unlock)
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("⚠", ui, Color32::GOLD,&self.esp)
                            });
                            col.cell(|ui| {
                                
                            });
                        });
                    });

                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button(&format!("SEAT HEATER - {}", self.make_seat_status_text(&self.heater_left_status)), ui, Color32::RED,&self.heater_right)
                            });
                            col.cell(|ui| {
                                big_button(&format!("SEAT COOLER - {}", self.make_seat_status_text(&self.chiller_left_status)), ui, Color32::BLUE,&self.chiller_right)
                            });
                        });
                    });

                })
        });
        ctx.request_repaint();
    }
}


fn main() {
    let mut app = LowerControlPanelUI::new();
    let mut native_options = NativeOptions::default();
    // 2048x1536
    native_options.initial_window_size = Some(Vec2::new(2048.0, 300.0));
    native_options.max_window_size = Some(Vec2::new(2048.0, 300.0));
    native_options.decorated = false;
    eframe::run_native(
        "Lower control panel emulator",
        native_options,
        Box::new(|cc| Box::new(app)),
    );
}
