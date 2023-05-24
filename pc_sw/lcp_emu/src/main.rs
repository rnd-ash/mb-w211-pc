use std::{sync::{atomic::{AtomicBool, Ordering, AtomicU8, AtomicU64, AtomicU32, AtomicU16}, Arc, Mutex}, time::{Duration, Instant}, borrow::BorrowMut};

use eframe::{NativeOptions, epaint::{Vec2, Color32, FontId, Shape, PathShape, Stroke, Mesh, TextureId, mutex::RwLock, Pos2}, egui::{Button, CentralPanel, Sense, self, Ui}, emath::{Align2, lerp}};
use egui_extras::{StripBuilder, Size};
use soloud::{audio, Wav, AudioExt, LoadExt, Soloud};
use w211_can::canbus::CanBus;

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
    esp: Arc<AtomicBool>,

    click1: audio::Wav,
    click2: audio::Wav,
    sl: Arc<RwLock<Soloud>>,
    fuel: Arc<AtomicU64>,
    fuel_flow: Arc<AtomicU64>,
    bps_b: Arc<AtomicU64>,
    bps_c: Arc<AtomicU64>,
}

fn make_pair() -> (Arc<AtomicBool>, Arc<AtomicBool>) {
    let c = Arc::new(AtomicBool::new(false));
    (c.clone(), c)
}

fn make_pair_status() -> (Arc<AtomicU8>, Arc<AtomicU8>) {
    let c = Arc::new(AtomicU8::new(0xFF));
    (c.clone(), c)
}

fn make_pair_counter() -> (Arc<AtomicU64>, Arc<AtomicU64>) {
    let c = Arc::new(AtomicU64::new(0));
    (c.clone(), c)
}

impl LowerControlPanelUI {
    pub fn new() -> Self {
        let sl = Arc::new(RwLock::new(Soloud::default().unwrap()));
        let sl_c = sl.clone();
        let can = CanBus::B;
        let can_b = loop {
            if let Ok(c)= can.create_can_socket(&[0x00E, 0x210]) {
                break c;
            } else {
                std::thread::sleep(Duration::from_millis(100));
            }
        };

        let can_b_monitor = can.create_can_socket(&[]).unwrap();

        let can = CanBus::C;
        let can_c = loop {
            if let Ok(c)= can.create_can_socket(&[0x608]) {
                break c;
            } else {
                std::thread::sleep(Duration::from_millis(100));
            }
        };
        let can_c_monitor = can.create_can_socket(&[]).unwrap();

        let (heater_right, heater_right_c) = make_pair();
        let (heater_left, heater_left_c) = make_pair();
        let (chiller_right, chiller_right_c) = make_pair();
        let (chiller_left, chiller_left_c) = make_pair();
        let (blind, blind_c) = make_pair();
        let (headrests, headrests_c) = make_pair();
        let (lock, lock_c) = make_pair();
        let (unlock, unlock_c) = make_pair();
        let (esp, esp_c) = make_pair();

        let (heater_right_status, heater_right_status_c) = make_pair_status();
        let (heater_left_status, heater_left_status_c) = make_pair_status();
        let (chiller_right_status, chiller_right_status_c) = make_pair_status();
        let (chiller_left_status, chiller_left_status_c) = make_pair_status();
        let (fuel, fuel_c) = make_pair_counter();
        let (fuel_flow, fuel_flow_c) = make_pair_counter();
        let (bps_c, bps_c_c) = make_pair_counter();
        let (bps_b, bps_b_c) = make_pair_counter();
        std::thread::spawn(move|| {
            loop {
                if let Some((f, data)) = can_c.read_frame() {
                    if f == 0x608 { // MS608 (10 updates/sec)
                        let f = ((data[5] as u16) << 8) | data[6] as u16; // Consumption over last 250ms
                        fuel_flow_c.store(f as u64, Ordering::Relaxed); // ul/sec
                        fuel_c.fetch_add((f as f64 / 50.0) as u64, Ordering::Relaxed);
                    }
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        });

        std::thread::spawn(move|| {
            let mut c = 0;
            let mut b = 0;
            let mut measure = Instant::now();
            loop {
                if let Some(f) = can_b_monitor.read_frame() {
                    b += 15 + (8*f.1.len() as u64)
                }
                if let Some (f) = can_c_monitor.read_frame() {
                    c += 15 + (8*f.1.len() as u64)
                }

                if measure.elapsed().as_millis() >= 1000 {
                    bps_c_c.store(c, Ordering::Relaxed);
                    bps_b_c.store(b, Ordering::Relaxed);
                    c = 0;
                    b = 0;
                    measure = Instant::now();
                }
                std::thread::sleep(Duration::from_micros(1));
            }
        });

        std::thread::spawn(move|| {
            let mut tick = audio::Wav::default();
            let mut tock = audio::Wav::default();
            tick.load_mem(include_bytes!("../indicator_tick.mp3")).unwrap();
            tock.load_mem(include_bytes!("../indicator_tock.mp3")).unwrap();
            const PC_CTRL_CAN_ID: u16 = 0x2D;
            let mut last_tx_time = Instant::now();
            let mut blinker_on_time: u16 = 0;
            let mut blinker_update: Instant = Instant::now();
            let mut right_on: bool = false;
            let mut left_on: bool = false;
            loop {
                if last_tx_time.elapsed().as_millis() > 100 {
                    let mut data: [u8; 2] = [0x00, 0x00];
                    if chiller_left_c.load(Ordering::Relaxed) {
                        data[0] |= 0x80;
                    }
                    if heater_left_c.load(Ordering::Relaxed) {
                        data[0] |= 0x40;
                    }
                    if chiller_right_c.load(Ordering::Relaxed) {
                        data[0] |= 0x20;
                    }
                    if heater_right_c.load(Ordering::Relaxed) {
                        data[0] |= 0x10;
                    }
                    if lock_c.load(Ordering::Relaxed) {
                        data[0] |= 0x08;
                    }
                    if unlock_c.load(Ordering::Relaxed) {
                        data[0] |= 0x04;
                    }
                    if blind_c.load(Ordering::Relaxed) {
                        data[0] |= 0x02;
                    }
                    if headrests_c.load(Ordering::Relaxed) {
                        data[0] |= 0x01;
                    }
                    if esp_c.load(Ordering::Relaxed) {
                        data[1] |= 0x80;
                    }
                    can_b.send_frame(PC_CTRL_CAN_ID, &data);
                    last_tx_time = Instant::now();
                }
                if let Some((id, frame)) = can_b.read_frame() {
                    if id == 0x00E {
                        // 40 - Left
                        // 80 - Right
                        // E0 - Hazads
                        left_on = frame[0] == 0x40;
                        right_on = frame[0] == 0x80;
                        if frame[0] == 0xE0 {
                            left_on = true;
                            right_on = true;
                        }

                        blinker_on_time = frame[1] as u16 * 10;
                        blinker_update = Instant::now();
                        if frame[1] != 0x00 { 
                            let mut lck = sl_c.write();
                            if left_on {
                                let h = lck.play_3d(&tick, -10.0, 10.0, 0.0);
                                lck.set_volume(h, 4.0);
                            }
                            if right_on {
                                let h = lck.play_3d(&tick, 10.0, 10.0, 0.0);
                                lck.set_volume(h, 4.0);
                            }
                        }
                    } else { // Seat heater status
                        let front_left = frame[0];
                        let front_right = frame[1];

                        heater_left_status_c.store((front_left & 0b11000) >> 3, Ordering::Relaxed);
                        chiller_left_status_c.store((front_left & 0b11), Ordering::Relaxed);

                        heater_right_status_c.store((front_right & 0b11000) >> 3, Ordering::Relaxed);
                        chiller_right_status_c.store((front_right & 0b11), Ordering::Relaxed);
                    }
                }
                if (right_on || left_on) && blinker_update.elapsed().as_millis() > blinker_on_time as u128 {
                    let mut lck = sl_c.write();
                    if right_on {
                        let h = lck.play_3d(&tock, 10.0, 0.0, 0.0);
                        lck.set_volume(h, 4.0);
                        right_on = false;
                    }
                    if left_on { // Left on
                        let h = lck.play_3d(&tock, -10.0, 0.0, 0.0);
                        lck.set_volume(h, 4.0);
                        left_on = false;
                    }
                }
                std::thread::sleep(Duration::from_millis(5));
            }
        });

        let mut s = Self {
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
            esp,
            click1: audio::Wav::default(),
            click2: audio::Wav::default(),
            sl: sl,
            fuel,
            fuel_flow,
            bps_b,
            bps_c
        };

        s.click1.load_mem(include_bytes!("../click1.mp3")).unwrap();
        s.click2.load_mem(include_bytes!("../click2.mp3")).unwrap();
        s
    }


    pub fn make_seat_status_text(&self, x: &Arc<AtomicU8>) -> String {
        match x.load(Ordering::Relaxed) {
            0 => "OFF",
            1 => "1",
            2 => "2",
            3 => "3",
            _ => "??"
        }.to_string()
    }

}


fn faded_color(color: Color32) -> Color32 {
    use egui::Rgba;
    egui::lerp(Rgba::from(color)..=Rgba::from(Color32::WHITE), 0.8).into()
}

fn big_button(text: &str, ui: &mut Ui, bg_color: Color32, store: &Arc<AtomicBool>, sl: &Arc<RwLock<Soloud>>, wav: &Wav) {
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
    if response.clicked() {
        sl.read().play_3d(wav, 0.0, 0.0, 0.0);
    }

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
                                big_button(&format!("SEAT HEATER - {}", self.make_seat_status_text(&self.heater_left_status)), ui, Color32::RED, &self.heater_left, &self.sl, &self.click1)
                            });
                            col.cell(|ui| {
                                big_button(&format!("SEAT COOLER - {}", self.make_seat_status_text(&self.chiller_left_status)), ui, Color32::BLUE,&self.chiller_left, &self.sl, &self.click1)
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("BLIND", ui, Color32::DARK_GRAY, &self.blind, &self.sl, &self.click1)
                            });
                            col.cell(|ui| {
                                // Net speeds
                                let dimens = ui.available_rect_before_wrap();
                                let center = dimens.center();
                                let top = Pos2::new(center.x, center.y-20.0);
                                let bottom = Pos2::new(center.x, center.y+20.0);
                                ui.painter().text(top, Align2::CENTER_CENTER, format!("C_B: {:.2}kbps", self.bps_b.load(Ordering::Relaxed) as f32 / 1000.0), FontId::monospace(20.0), Color32::WHITE);
                                ui.painter().text(bottom, Align2::CENTER_CENTER, format!("C_C: {:.2}kbps", self.bps_c.load(Ordering::Relaxed) as f32 / 1000.0), FontId::monospace(20.0), Color32::WHITE);
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("HEADREST", ui, Color32::DARK_GREEN,&self.headrests, &self.sl, &self.click1)
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
                        ui.painter().text(top, Align2::CENTER_CENTER, "Fuel price 1.38GBP/L", FontId::monospace(25.0), Color32::WHITE);
                        ui.painter().text(center, Align2::CENTER_CENTER, format!("Money rate:\n{:.2} GBP/Min", flow_rate*1.38), FontId::monospace(25.0), Color32::WHITE);
                        ui.painter().text(bottom, Align2::CENTER_CENTER, format!("Fuel rate:\n{:.3} L/Min", flow_rate), FontId::monospace(25.0), Color32::WHITE);


                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("LOCK", ui, Color32::DARK_GRAY,&self.lock, &self.sl, &self.click1)
                            });
                            col.cell(|ui| {
                                big_button("UNLOCK", ui, Color32::DARK_GRAY, &self.unlock, &self.sl, &self.click1)
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("⚠", ui, Color32::GOLD,&self.esp, &self.sl, &self.click1)
                            });
                            col.cell(|ui| {
                                
                            });
                        });
                    });

                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button(&format!("SEAT HEATER - {}", self.make_seat_status_text(&self.heater_right_status)), ui, Color32::RED,&self.heater_right, &self.sl, &self.click1)
                            });
                            col.cell(|ui| {
                                big_button(&format!("SEAT COOLER - {}", self.make_seat_status_text(&self.chiller_right_status)), ui, Color32::BLUE,&self.chiller_right, &self.sl, &self.click1)
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
    native_options.vsync = true;
    eframe::run_native(
        "Lower control panel emulator",
        native_options,
        Box::new(|cc| Box::new(app)),
    );
}
