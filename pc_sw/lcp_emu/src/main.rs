use std::{sync::{atomic::{AtomicBool, Ordering, AtomicU8, AtomicU64, AtomicU32, AtomicU16, AtomicI32}, Arc, Mutex}, time::{Duration, Instant}, borrow::BorrowMut, fmt::Display, process::{Command, self}};

use eframe::{NativeOptions, epaint::{Vec2, Color32, FontId, Shape, PathShape, Stroke, Mesh, TextureId, mutex::RwLock, Pos2, Rect}, egui::{Button, CentralPanel, Sense, self, Ui}, emath::{Align2, lerp}};
use egui_extras::{StripBuilder, Size};
use w211_can::{canbus::{CanBus, CanWrapper}, canb, canc::{LRW_236, MS_210, BS_200, BS_200h_BLS}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonType {
    Static,
    OnOff(bool),
    Level {
        lit_bars: usize,
        total_bars: usize
    }
}

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
    reboot: Arc<AtomicBool>,
    headrests: Arc<AtomicBool>,
    lock: Arc<AtomicBool>,
    unlock: Arc<AtomicBool>,
    esp: Arc<AtomicBool>,
    fuel: Arc<AtomicU64>,
    fuel_flow: Arc<AtomicU64>,
    bps_b: Arc<AtomicU64>,
    bps_c: Arc<AtomicU64>,
    eq_running: Arc<AtomicBool>,
    bsg_current: Arc<AtomicI32>,
    volume: Arc<AtomicU32>
}

fn make_pair<T>(init: T) -> (Arc<T>, Arc<T>) {
    let c = Arc::new(init);
    (c.clone(), c)
}

impl LowerControlPanelUI {
    pub fn new() -> Self {
        let can = CanBus::B;
        let can_b = loop {
            if let Ok(c)= can.create_can_socket(&[0x210]) {
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

        let (heater_right, heater_right_c) = make_pair(AtomicBool::new(false));
        let (reboot, reboot_c) = make_pair(AtomicBool::new(false));
        let (heater_left, heater_left_c) = make_pair(AtomicBool::new(false));
        let (chiller_right, chiller_right_c) = make_pair(AtomicBool::new(false));
        let (chiller_left, chiller_left_c) = make_pair(AtomicBool::new(false));
        let (blind, blind_c) = make_pair(AtomicBool::new(false));
        let (headrests, headrests_c) = make_pair(AtomicBool::new(false));
        let (lock, lock_c) = make_pair(AtomicBool::new(false));
        let (unlock, unlock_c) = make_pair(AtomicBool::new(false));
        let (esp, esp_c) = make_pair(AtomicBool::new(false));

        let (heater_right_status, heater_right_status_c) = make_pair(AtomicU8::new(0));
        let (heater_left_status, heater_left_status_c) = make_pair(AtomicU8::new(0));
        let (chiller_right_status, chiller_right_status_c) = make_pair(AtomicU8::new(0));
        let (chiller_left_status, chiller_left_status_c) = make_pair(AtomicU8::new(0));
        let (fuel, fuel_c) = make_pair(AtomicU64::new(0));
        let (fuel_flow, fuel_flow_c) = make_pair(AtomicU64::new(0));
        let (bps_c, bps_c_c) = make_pair(AtomicU64::new(0));
        let (bps_b, bps_b_c) = make_pair(AtomicU64::new(0));
        let (bsg, bsg_c) = make_pair(AtomicI32::new(0));
        let eq_running = Arc::new(AtomicBool::new(false));
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
        /*
        std::thread::spawn(move|| {
            let mut sock = w211_can::canbus::CanBus::B.create_iso_tp_socket(1842, 1266, 40, 8).unwrap();
            let cb = CanBus::B.create_can_socket(&[]).unwrap();
            let _ = sock.set_nonblocking(true);
            loop {
                let now = Instant::now();
                //let _ = cb.send_frame(0x001C, &[0x02, 0x10, 0x92, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC]);
                let _ = sock.write(&[0x31, 0x02, 0x00 , 0x64]);
                while now.elapsed().as_millis() < 1000 {
                    if let Ok(data) = sock.read() {
                        println!("Data from BSG211! {data:02X?}");
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                let elapsed = now.elapsed().as_millis();
                if elapsed < 1000 {
                    std::thread::sleep(Duration::from_millis((1000 - elapsed) as u64));
                }
            }
        });
        */

        let volume = Arc::new(AtomicU32::new(0));
        let volume_c = volume.clone();

        std::thread::spawn(move|| {
            let mut last_tx_time = Instant::now();
            let mut pc = canb::PC_CTRL_PANEL::new(0);
            loop {
                if last_tx_time.elapsed().as_millis() > 100 {
                    pc.set_DRIVER_COOLER_PRESSED(chiller_right_c.load(Ordering::Relaxed));
                    pc.set_DRIVER_HEATER_PRESSED(heater_right_c.load(Ordering::Relaxed));
                    pc.set_ESPOFF(esp_c.load(Ordering::Relaxed));
                    pc.set_HEADREST(headrests_c.load(Ordering::Relaxed));
                    pc.set_BLIND(blind_c.load(Ordering::Relaxed));
                    pc.set_LOCK(lock_c.load(Ordering::Relaxed));
                    pc.set_PASS_COOLER_PRESSED(chiller_left_c.load(Ordering::Relaxed));
                    pc.set_PASS_HEATER_PRESSED(heater_left_c.load(Ordering::Relaxed));
                    pc.set_UNLOCK(unlock_c.load(Ordering::Relaxed));
                    let f = w211_can::canbus::u64_to_frame(canb::PC_CTRL_PANEL::get_canid(), pc.0, 2);
                    can_b.send_frame_raw(f);
                    last_tx_time = Instant::now();
                }
                if let Some((id, frame)) = can_b.read_frame() {
                    let front_left = frame[0];
                    let front_right = frame[1];

                    heater_left_status_c.store((front_left & 0b11000) >> 3, Ordering::Relaxed);
                    chiller_left_status_c.store((front_left & 0b11), Ordering::Relaxed);

                    heater_right_status_c.store((front_right & 0b11000) >> 3, Ordering::Relaxed);
                    chiller_right_status_c.store((front_right & 0b11), Ordering::Relaxed);
                }
                if let Ok(output) = process::Command::new("/usr/bin/wpctl")
                    .args(&[
                        "get-volume",
                        "@DEFAULT_AUDIO_SINK@"
                    ]).output().map(|x| String::from_utf8(x.stdout).unwrap()) {
                        let v = output.replace("Volume: ", "").replace("\n", "");
                        if let Ok(f) = v.parse::<f32>() {
                            let v = 100.0 * (f/0.8);
                            volume_c.store(v as u32, Ordering::Relaxed);
                        }
                    }
                if reboot_c.load(Ordering::Relaxed) {
                    std::process::Command::new("/usr/bin/sudo").args(["reboot"]).output();
                }
                std::thread::sleep(Duration::from_millis(20));

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
            reboot,
            headrests,
            lock,
            unlock,
            esp,
            fuel,
            fuel_flow,
            bps_b,
            bps_c,
            eq_running,
            bsg_current: bsg,
            volume
        };
        s
    }
}


fn faded_color(color: Color32) -> Color32 {
    use egui::Rgba;
    egui::lerp(Rgba::from(color)..=Rgba::from(Color32::WHITE), 0.8).into()
}

fn big_button(text: &str, ui: &mut Ui, bg_color: Color32,store: &Arc<AtomicBool>, disp: ButtonType) {
    let size = if text.len() <= 3 {
        100.0
    } else {
        30.0
    };
    big_button_with_txt_size(size, text, ui, bg_color, store, disp);
}

fn big_button_with_txt_size(size: f32, text: &str, ui: &mut Ui, bg_color: Color32,store: &Arc<AtomicBool>, disp: ButtonType) -> bool {
    let dimens = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(dimens, Sense::click_and_drag());
    store.store(response.is_pointer_button_down_on(), Ordering::Relaxed);
    let mut ret = false;
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
    } else if let ButtonType::Level { lit_bars, total_bars } = disp {
        let bar_y = dimens.center().y + 30.0;
        let mut c = Color32::GREEN;
        if lit_bars > total_bars {
            c = Color32::WHITE
        }

        let l = dimens.left() + 20.0;
        let r = dimens.right() - 20.0;
        const BAR_PADDING: f32 = 5.0;

        let bar_width = (r-l)/(total_bars as f32) - BAR_PADDING;

        let mut s = l;
        for i in 1..=total_bars {
            c = if i <= lit_bars {
                Color32::GREEN
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
    ret

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

fn read_recent_frame_as_u64(can: &CanWrapper) -> Option<(u16, (u64, u8))> {
    let mut f = None;
    while let Some(s) = can.read_frame_as_u64() {
        f = Some(s);
    }
    f
}

fn read_recent_frame_bytes(can: &CanWrapper) -> Option<(u16, Vec<u8>)> {
    let mut f = None;
    while let Some(s) = can.read_frame() {
        f = Some(s);
    }
    f
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
                                big_button("SEAT HEATER", ui, Color32::RED, &self.heater_left, ButtonType::Level { lit_bars: self.heater_left_status.load(Ordering::Relaxed) as usize, total_bars: 3 })
                            });
                            col.cell(|ui| {
                                big_button("SEAT COOLER", ui, Color32::BLUE,&self.chiller_left, ButtonType::Level { lit_bars: self.chiller_left_status.load(Ordering::Relaxed) as usize, total_bars: 3 })
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("BLIND", ui, Color32::DARK_GRAY, &self.blind, ButtonType::Static)
                            });
                            col.cell(|ui| {
                                big_button("REBOOT", ui, Color32::RED, &self.reboot, ButtonType::Static)
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("HEADREST", ui, Color32::DARK_GREEN,&self.headrests, ButtonType::Static)
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
                                        if big_button_with_txt_size(40.0,"GAME", ui, Color32::GOLD,&sto, ButtonType::Static) {
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
                                                    let cs_mrm = w211_can::canbus::CanBus::C;
                                                    let wheel_can = w211_can::canbus::CanBus::B;
                                                    let wrapper = cs_mrm.create_can_socket(&[0x0236]).unwrap();
                                                    let wrapper_pw = cs_mrm.create_can_socket(&[0x210]).unwrap();
                                                    let wrapper_brake = cs_mrm.create_can_socket(&[0x200]).unwrap();
                                                    let wrapper_b = wheel_can.create_can_socket(&[0x01A8]).unwrap();
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
                                        }
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
                                big_button("LOCK", ui, Color32::DARK_GRAY,&self.lock, ButtonType::Static)
                            });
                            col.cell(|ui| {
                                big_button("UNLOCK", ui, Color32::DARK_GRAY, &self.unlock, ButtonType::Static)
                            });
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).vertical(|mut col| {
                            col.cell(|ui| {
                                big_button("⚠", ui, Color32::GOLD,&self.esp, ButtonType::Static)
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
                                big_button("SEAT HEATER", ui, Color32::RED,&self.heater_right, ButtonType::Level { lit_bars: self.heater_right_status.load(Ordering::Relaxed) as usize, total_bars: 3 })
                            });
                            col.cell(|ui| {
                                big_button("SEAT COOLER", ui, Color32::BLUE,&self.chiller_right, ButtonType::Level { lit_bars: self.chiller_right_status.load(Ordering::Relaxed) as usize, total_bars: 3 })
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
