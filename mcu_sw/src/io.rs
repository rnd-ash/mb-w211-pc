use atsamd_hal::{adc::{self, FutureAdc}, pac::Supc, prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin, rtic_time::Monotonic};
use fugit::{Duration, ExtU64, Instant};
use rtic_sync::channel::{Receiver, Sender};
use w211_can::canb::EZS_A1;

use crate::{
    Mono, PC_AWAKE, bsp::{self, AmpCurrentSense, CanCShutdown, PcCurrentSense, VSense}, can::{SerialCanFrame, frame_to_int}
};

#[derive(Default, Clone, Copy)]
pub struct SensorAccum {
    v_batt: u32,
    c_amp: u32,
    c_pc: u32,
    num_samples: u8,
}

pub struct BoardIO {
    pub amp_mosfet: bsp::AmpMosfet,
    pub pc_mosfet: bsp::PcMosfet,

    pub amp_mute: bsp::AmpMute,
    pub amp_standby: bsp::AmpStandby,
    pub can_c_shutdown: CanCShutdown,

    pub rx_ezs_a1: Receiver<'static, [u8; 8], 10>,
    pub rx_cane: Receiver<'static, SerialCanFrame, 10>,
    pub last_ezsa1_time: Instant<u64, 1, 32768>,
    pub last_consumption_time: Instant<u64, 1, 32768>,
    pub is_shutdown: bool,
    pub ezs_a1: w211_can::canb::EZS_A1,
    pub adc: FutureAdc<adc::Adc0, crate::Adc0Irqs>,
    pub pc_awake_time: u64,
    adc_vsense: VSense,
    adc_pc_csense: PcCurrentSense,
    adc_amp_csense: AmpCurrentSense,
    sensors_accum: SensorAccum,
    tx_cane: Sender<'static, SerialCanFrame, 200>
}

impl BoardIO {
    pub fn new(
        adc: FutureAdc<adc::Adc0, crate::Adc0Irqs>,
        amp_mosfet: bsp::AmpMosfet,
        pc_mosfet: bsp::PcMosfet,
        amp_mute: bsp::AmpMute,
        amp_standby: bsp::AmpStandby,
        can_c_shutdown: CanCShutdown,
        adc_vsense: VSense,
        adc_pc_csense: PcCurrentSense,
        adc_amp_csense: AmpCurrentSense,
        rx_ezs_a1: Receiver<'static, [u8; 8], 10>,
        rx_cane: Receiver<'static, SerialCanFrame, 10>,
        tx_cane: Sender<'static, SerialCanFrame, 200>,
        time: Instant<u64, 1, 32768>,
    ) -> Self {
        let mut s = Self {
            amp_mosfet,
            pc_mosfet,
            amp_mute,
            amp_standby,
            can_c_shutdown,
            rx_ezs_a1,
            rx_cane,
            last_ezsa1_time: time,
            is_shutdown: false,
            ezs_a1: EZS_A1::default(),
            adc,
            adc_vsense,
            adc_amp_csense,
            adc_pc_csense,
            last_consumption_time: time,
            tx_cane,
            sensors_accum: Default::default(),
            pc_awake_time: 0,
        };
        s.shutdown();
        s
    }

    pub fn shutdown(&mut self) {
        if !self.is_shutdown {
            defmt::info!("Shutting down");
            let _ = self.can_c_shutdown.set_high();
            let _ = self.amp_mute.set_low();
            let _ = self.amp_standby.set_low();
            let _ = self.amp_mosfet.set_low();
            let _ = self.pc_mosfet.set_low();
            self.sensors_accum = Default::default();
        }
        PC_AWAKE.store(false, core::sync::atomic::Ordering::Relaxed);
        self.is_shutdown = true;
    }

    pub async fn update(&mut self, supc: &mut Supc) -> bool {
        let mut can_alive = false;
        let mut key_in_ezs = false;
        let ezs_frame = self.rx_ezs_a1.try_recv();
        if let Ok(ezs_frame) = ezs_frame {
            self.last_ezsa1_time = Mono::now();
            self.ezs_a1 = EZS_A1::new(frame_to_int(&ezs_frame, 8));
            can_alive = true;
            if self.is_shutdown {
                defmt::info!("Waking up!");
                self.pc_awake_time = Mono::now().duration_since_epoch().to_millis();
            }
            self.is_shutdown = false;
            let _ = self.amp_mosfet.set_high();
            let _ = self.pc_mosfet.set_high();
            let _ = self.can_c_shutdown.set_low();
            // MUTE is not turned on here, it is turned on below by the CAN E events
        }
        if let Some(time_since_ezsa1) = Mono::now().checked_duration_since(self.last_ezsa1_time) {
            if time_since_ezsa1.to_millis() < 500 {
                // Assuming CAN is still active
                can_alive = true;
            } else if time_since_ezsa1.to_millis() > 5_000 {
                // Assuming CAN is dead

                // Shutdown
                self.shutdown();
            }
        } else {
            defmt::error!("Time is in the past!?");
        }
        if can_alive {
            key_in_ezs = self.ezs_a1.get_KL_15R_EIN();
            let _ = self.amp_standby.set_state(key_in_ezs.into());
            let _ = self.can_c_shutdown.set_state((!key_in_ezs).into()); // Inverse!

            // Check for the PC being awake
            let pc_active = PC_AWAKE.load(core::sync::atomic::Ordering::Relaxed);
            // Wait 2 minutes for PC to wake up before attempting restart
            if !pc_active && Mono::now().duration_since_epoch().to_millis() - self.pc_awake_time > 120_000 {
                defmt::info!("PC not responding, rebooting!");
                let _ = self.pc_mosfet.set_low();
                Mono::delay(2000u64.millis()).await;
                let _ = self.pc_mosfet.set_high();
                self.pc_awake_time = Mono::now().duration_since_epoch().to_millis();
            }


            // Process any incommming events from PC now
            if let Ok(pc_evt) = self.rx_cane.try_recv() {
                if pc_evt.id == 0x0001 {
                    let en = pc_evt.data[0] == 0x01;
                    let _ = self.amp_mute.set_state(en.into());
                }
            }

            // Read ADC on our channels and Tx the data to CAN-E
            let adc_raw_vsense = self.adc.read(&mut self.adc_vsense).await;
            let adc_raw_pc_curr = self.adc.read(&mut self.adc_pc_csense).await;
            let adc_raw_amp_curr = self.adc.read(&mut self.adc_amp_csense).await;
            let cpu_t_real = self.adc.read_cpu_temperature(supc).await;
            let cpu_t_can = match cpu_t_real {
                x if x < 0.0 => 0,
                x if x > 250.0 => 250,
                x => x as u8
            };

            // Convert to current
            const R_C_AMPLIFIED: f32 = 100.0 * 0.3;// (INA180_A3 factor * Resistance Ohms)

            #[inline]
            fn get_current(adc_raw: u16) -> u16 {
                // I = V_adc / (R_sense * Multi)
                let v = adc_raw as f32 / 4096.0 * 3.3;
                return ((v * R_C_AMPLIFIED)*100.0) as u16;
            }

            #[inline]
            fn get_in_voltage(adc_raw: u16) -> u16 {
                // I = V_adc / (R_sense * Multi)
                let v = adc_raw as f32 / 4096.0 * 3.3 * 1000.0;
                return (v * 5.54) as u16;
            }
            {
                let c_pc = get_current(adc_raw_pc_curr);
                let c_amp = get_current(adc_raw_amp_curr);
                let vsense = get_in_voltage(adc_raw_vsense);
                self.sensors_accum.c_amp += c_amp as u32;
                self.sensors_accum.c_pc += c_pc as u32;
                self.sensors_accum.v_batt += vsense as u32;
                self.sensors_accum.num_samples += 1;
            }
            

            if let Some(time_since_consumption_tx) = Mono::now().checked_duration_since(self.last_consumption_time) {
                if time_since_consumption_tx.to_millis() >= 1000 && self.sensors_accum.num_samples != 0 {
                    let sample_count = self.sensors_accum.num_samples as u32;
                    let c_amp = (self.sensors_accum.c_amp / sample_count) as u16;
                    let c_pc = (self.sensors_accum.c_pc / sample_count) as u16;
                    let vsense = (self.sensors_accum.v_batt / sample_count) as u16;
                    self.sensors_accum = Default::default();

                    let w_amp = (vsense as u32 * c_amp as u32) as f32 / (1000.0*1000.0);
                    let w_pc = (vsense as u32 * c_pc as u32) as f32 / (1000.0*1000.0);

                    defmt::info!("PC {}mA ({}W), AMP: {}mA ({}W)  Board: {}C", c_pc, w_pc, c_amp, w_amp, cpu_t_real);
                    let mut data = [0u8; 8];
                    data[0] = (c_pc & 0xFF) as u8;
                    data[1] = (c_pc >> 8) as u8;
                    data[2] = (c_amp & 0xFF) as u8;
                    data[3] = (c_amp >> 8) as u8;
                    data[4] = (vsense & 0xFF) as u8;
                    data[5] = (vsense >> 8) as u8;
                    data[6] = cpu_t_can;
                    let f_cane = SerialCanFrame::new(crate::can::CanNet::E, 0x0002, &data);
                    let _ = self.tx_cane.try_send(f_cane);
                    self.last_consumption_time = Mono::now();
                }   
            } else {
                defmt::error!("Time is in the past!?");
            }
        }

        key_in_ezs
    }
}
