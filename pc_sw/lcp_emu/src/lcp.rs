use std::{io::Cursor, sync::{Arc, RwLock}, time::Duration};

use ambisonic::{rodio::{Decoder, Source}, Ambisonic, AmbisonicBuilder};
use bitflags::bitflags;
use eframe::egui::Color32;
use futures_util::{SinkExt, StreamExt};
use tokio::{runtime::Runtime, sync::mpsc::UnboundedSender};
use tokio_socketcan::CANFilter;
use w211_can::{canb::{self, PC_CTRL_PANEL_CAN_ID, SHZ_A1}, canbus::{frame_to_u64, u64_to_frame}};

use crate::ButtonType;

#[derive(Debug, Clone, Copy)]
pub enum LcpButton {
    Esp,
    Headrest,
    Blind,
    Reboot,
    Lock,
    Unlock,
    ChillerR,
    ChillerL,
    HeaterR,
    HeaterL,
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct ButtonPressFlags: u32 {
        const ESP      = 0b00000000_00000001;
        const HEADREST = 0b00000000_00000010;
        const BLIND    = 0b00000000_00000100;
        const LOCK     = 0b00000000_00001000;
        const UNLOCK   = 0b00000000_00010000;
        const CHILL_R  = 0b00000000_00100000;
        const CHILL_L  = 0b00000000_01000000;
        const HEAT_R   = 0b00000000_10000000;
        const HEAT_L   = 0b00000001_00000000;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonLEDMatrix {
    h_l: ButtonType,
    h_r: ButtonType,
    c_l: ButtonType,
    c_r: ButtonType,
}

impl Default for ButtonLEDMatrix {
    fn default() -> Self {
        Self { 
            h_l: ButtonType::Level { bar_colour: Color32::BLACK, lit_bars: 0, total_bars: 3 }, 
            h_r: ButtonType::Level { bar_colour: Color32::BLACK, lit_bars: 0, total_bars: 3 }, 
            c_l: ButtonType::Level { bar_colour: Color32::BLACK, lit_bars: 0, total_bars: 3 }, 
            c_r: ButtonType::Level { bar_colour: Color32::BLACK, lit_bars: 0, total_bars: 3 },
        }
    }
}



pub struct Lcp {
    seat_colour_matrix: Arc<RwLock<ButtonLEDMatrix>>,
    press_matrix: ButtonPressFlags,
    sender: UnboundedSender<ButtonPressFlags>,
    audio_scene: Ambisonic,
}

impl Lcp {
    pub fn new(rt: Arc<Runtime>, can_b_name: &'static str) -> Self {
        let button_matrix = Arc::new(RwLock::new(ButtonLEDMatrix::default()));
        let button_matrix_c = button_matrix.clone();
        
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<ButtonPressFlags>();

        let audio = AmbisonicBuilder::new().build();

        rt.clone().spawn(async move {
            let b = loop {
                match tokio_socketcan::CANSocket::open(can_b_name) {
                    Ok(s) => break s,
                    Err(_) => {
                        continue;
                    }
                }
            };
            b.set_filter(&[CANFilter::new(w211_can::canb::SHZ_A1_CAN_ID as u32, 0xFFF).unwrap()]).unwrap();
            let (mut can_sink, mut can_stream) = b.split();
            rt.clone().spawn(async move {
                loop {
                    if let Some(Ok(frame)) = can_stream.next().await {
                        let mut matrix = button_matrix_c.read().unwrap().clone();
                        let (data, _) = frame_to_u64(&frame);
                        let shz = SHZ_A1::new(data);
                        if let Some(state) = shz.get_SH_VL_ST() { // Front left heater
                            let bars = match state {
                                w211_can::canb::SHZ_A1_SH_VL_ST::SHZ_ST_AUS => 0,
                                w211_can::canb::SHZ_A1_SH_VL_ST::SHZ_STEP1 => 1,
                                w211_can::canb::SHZ_A1_SH_VL_ST::SHZ_STEP2 => 2,
                                w211_can::canb::SHZ_A1_SH_VL_ST::SHZ_STEP3 => 3,
                            };
                            matrix.h_l = ButtonType::Level { bar_colour: Color32::RED, lit_bars: bars, total_bars: 3 }
                        }

                        if let Some(state) = shz.get_SH_VR_ST() { // Front right heater
                            let bars = match state {
                                w211_can::canb::SHZ_A1_SH_VR_ST::SHZ_ST_AUS => 0,
                                w211_can::canb::SHZ_A1_SH_VR_ST::SHZ_STEP1 => 1,
                                w211_can::canb::SHZ_A1_SH_VR_ST::SHZ_STEP2 => 2,
                                w211_can::canb::SHZ_A1_SH_VR_ST::SHZ_STEP3 => 3,
                            };
                            matrix.h_r = ButtonType::Level { bar_colour: Color32::RED, lit_bars: bars, total_bars: 3 }
                        }

                        if let Some(state) = shz.get_SBLFT_VL_ST() { // Front left cooler
                            let bars = match state {
                                w211_can::canb::SHZ_A1_SBLFT_VL_ST::SBL_ST_AUS => 0,
                                w211_can::canb::SHZ_A1_SBLFT_VL_ST::SBL_STEP1 => 1,
                                w211_can::canb::SHZ_A1_SBLFT_VL_ST::SBL_STEP2 => 2,
                                w211_can::canb::SHZ_A1_SBLFT_VL_ST::SBL_STEP3 => 3,
                            };
                            matrix.c_l = ButtonType::Level { bar_colour: Color32::BLUE,lit_bars: bars, total_bars: 3 }
                        }

                        if let Some(state) = shz.get_SBLFT_VR_ST() { // Front right cooler
                            let bars = match state {
                                w211_can::canb::SHZ_A1_SBLFT_VR_ST::SBL_ST_AUS => 0,
                                w211_can::canb::SHZ_A1_SBLFT_VR_ST::SBL_STEP1 => 1,
                                w211_can::canb::SHZ_A1_SBLFT_VR_ST::SBL_STEP2 => 2,
                                w211_can::canb::SHZ_A1_SBLFT_VR_ST::SBL_STEP3 => 3,
                            };
                            matrix.c_r = ButtonType::Level { bar_colour: Color32::BLUE, lit_bars: bars, total_bars: 3 }
                        }

                        *button_matrix_c.write().unwrap() = matrix;
                    }
                }
            });

            // Now handle button presses
            let mut pc = canb::PC_CTRL_PANEL::new(0);
            let mut ticker = tokio::time::interval(Duration::from_millis(40));
            let mut idle_counter = 0;
            let mut current_state = ButtonPressFlags::empty();
            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        let mut tx = true;
                        if current_state != ButtonPressFlags::empty() { // User is pressing button
                            // Send data now
                            pc.set_BLIND(current_state.contains(ButtonPressFlags::BLIND));
                            pc.set_HEADREST(current_state.contains(ButtonPressFlags::HEADREST));
                            pc.set_LOCK(current_state.contains(ButtonPressFlags::LOCK));
                            pc.set_UNLOCK(current_state.contains(ButtonPressFlags::UNLOCK));

                            pc.set_PASS_HEATER_PRESSED(current_state.contains(ButtonPressFlags::HEAT_L));
                            pc.set_DRIVER_HEATER_PRESSED(current_state.contains(ButtonPressFlags::HEAT_R));

                            pc.set_DRIVER_COOLER_PRESSED(current_state.contains(ButtonPressFlags::CHILL_R));
                            pc.set_PASS_COOLER_PRESSED(current_state.contains(ButtonPressFlags::CHILL_L));

                            pc.set_ESPOFF(current_state.contains(ButtonPressFlags::ESP));
                        } else if idle_counter != 0 { // Idle state, and we have to Tx a couple frames
                            pc = canb::PC_CTRL_PANEL::new(0);
                            idle_counter -= 1;
                        } else {
                            tx = false;
                        }
                        if tx {
                            // We need to Tx to CAN
                            let frame = u64_to_frame(PC_CTRL_PANEL_CAN_ID, pc.0, 2);
                            let _ = can_sink.send(frame).await;
                        }
                    },
                    Some(state) = rx.recv() => {
                        current_state = state;
                        if current_state == ButtonPressFlags::empty() {
                            idle_counter = 2;
                        }
                    }
                }
            }
        });

        Self {
            seat_colour_matrix: button_matrix,
            press_matrix: ButtonPressFlags::empty(),
            sender: tx,
            audio_scene: audio,
        }
    }

    pub fn on_press(&mut self, button: LcpButton) {
        let bit = match button {
            LcpButton::Esp => ButtonPressFlags::ESP,
            LcpButton::Headrest => ButtonPressFlags::HEADREST,
            LcpButton::Blind => ButtonPressFlags::BLIND,
            LcpButton::Lock => ButtonPressFlags::LOCK,
            LcpButton::Unlock => ButtonPressFlags::UNLOCK,
            LcpButton::ChillerR => ButtonPressFlags::CHILL_R,
            LcpButton::ChillerL => ButtonPressFlags::CHILL_L,
            LcpButton::HeaterR => ButtonPressFlags::HEAT_R,
            LcpButton::HeaterL => ButtonPressFlags::HEAT_L,
            LcpButton::Reboot => {
                let _ = std::process::Command::new("/usr/bin/sudo").args(["reboot"]).output();
                return
            }
        };
        let matrix_now = self.press_matrix;
        self.press_matrix.insert(bit);
        if matrix_now != self.press_matrix {
            // Notify
            let _ = self.sender.send(self.press_matrix);
            let click_noise = include_bytes!("../click1.mp3").to_vec();
            let click_corsor = Cursor::new(click_noise);
            let r = ambisonic::rodio::Decoder::new_mp3(click_corsor).unwrap();
            self.audio_scene.play_at(r.convert_samples(), [0.0, 0.0, 1.0]);
        }
    }

    pub fn on_release(&mut self, button: LcpButton) {
        let bit = match button {
            LcpButton::Esp => ButtonPressFlags::ESP,
            LcpButton::Headrest => ButtonPressFlags::HEADREST,
            LcpButton::Blind => ButtonPressFlags::BLIND,
            LcpButton::Lock => ButtonPressFlags::LOCK,
            LcpButton::Unlock => ButtonPressFlags::UNLOCK,
            LcpButton::ChillerR => ButtonPressFlags::CHILL_R,
            LcpButton::ChillerL => ButtonPressFlags::CHILL_L,
            LcpButton::HeaterR => ButtonPressFlags::HEAT_R,
            LcpButton::HeaterL => ButtonPressFlags::HEAT_L,
            _ => return,
        };
        let matrix_now = self.press_matrix;
        self.press_matrix.remove(bit);

        if matrix_now != self.press_matrix {
            // Notify
            let _ = self.sender.send(self.press_matrix);
        }
    }

    pub fn get_state(&self, button: LcpButton) -> ButtonType {
        let matrix = self.seat_colour_matrix.read().unwrap().clone();
        match button {
            LcpButton::ChillerR => matrix.c_r,
            LcpButton::ChillerL => matrix.c_l,
            LcpButton::HeaterR => matrix.h_r,
            LcpButton::HeaterL => matrix.h_l,
            _ => ButtonType::Static
        }
    }
}