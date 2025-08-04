use atsamd_hal::{adc::{self, Adc, FutureAdc}, pac::Supc, prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin};
use embassy_time::Instant;
use w211_can::canb::{EZS_A1, EZS_A1_CAN_ID};

use crate::{
    bsp::{self, AmpCurrentSense, CanCShutdown, PcCurrentSense, VSense}, can::{frame_to_int, CanNet, SerialCanFrame}, CANE_TO_BOARD_CHANNEL, CAN_RX_CHANNEL
};

#[derive(Default, Clone, Copy)]
pub struct SensorAccum {
    v_batt: u32,
    c_amp: u32,
    c_pc: u32,
    t_cpu: u32,
    num_samples: u8,
}

pub struct BoardIO {
    pub amp_mosfet: bsp::AmpMosfet,
    pub pc_mosfet: bsp::PcMosfet,

    pub amp_mute: bsp::AmpMute,
    pub amp_standby: bsp::AmpStandby,
    pub can_c_shutdown: CanCShutdown,

    
    pub last_ezsa1_time: Instant,
    pub last_consumption_time: Instant,
    pub is_shutdown: bool,
    pub ezs_a1: w211_can::canb::EZS_A1,
    pub adc: FutureAdc<adc::Adc0, crate::Adc0Irqs>,
    pub amp_on: bool,
    adc_vsense: VSense,
    adc_pc_csense: PcCurrentSense,
    adc_amp_csense: AmpCurrentSense,
    sensors_accum: SensorAccum,
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
        time: Instant
    ) -> Self {
        let mut s = Self {
            amp_mosfet,
            pc_mosfet,
            amp_mute,
            amp_standby,
            can_c_shutdown,
            //rx_ezs_a1,
            //rx_cane,
            last_ezsa1_time: time,
            is_shutdown: false,
            ezs_a1: EZS_A1::default(),
            adc,
            adc_vsense,
            adc_amp_csense,
            adc_pc_csense,
            last_consumption_time: time,
            //tx_cane,
            sensors_accum: Default::default(),
            amp_on: false,
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
            self.amp_on = false;
            self.sensors_accum = Default::default();
            //wfi();
        }
        self.is_shutdown = true;
    }

    pub async fn update(&mut self, supc: &mut Supc) -> bool {
        let mut key_in_ezs = false;
        let mut can_alive = false;
        while let Ok(msg) = CANE_TO_BOARD_CHANNEL.dyn_receiver().try_receive() {
            match (msg.net, msg.id) {
                (CanNet::E, 0x0001) => {
                    self.amp_on = msg.data[0] == 0x01;
                }
                (CanNet::B, EZS_A1_CAN_ID) => {
                    self.last_ezsa1_time = Instant::now();
                    self.ezs_a1 = EZS_A1::new(frame_to_int(&msg.data, 8));
                    can_alive = true;
                    if self.is_shutdown {
                        defmt::info!("Waking up!");
                    }
                    self.is_shutdown = false;
                    let _ = self.amp_mosfet.set_high();
                    let _ = self.pc_mosfet.set_high();
                    let _ = self.amp_standby.set_high();
                    let _ = self.can_c_shutdown.set_low();
                },
                _ => {
                    defmt::error!("Unhandled CAN Frame by BOARD IO: (Bus {}: ID 0x{:04X})", msg.net, msg.id)
                }
            }
        }

        if let Some(time_since_ezsa1) = Instant::now().checked_duration_since(self.last_ezsa1_time) {
            if time_since_ezsa1.as_millis() < 1000 {
                // Assuming CAN is still active
                can_alive = true;
            } else if time_since_ezsa1.as_millis() > 5_000 {
                // Assuming CAN is dead

                // Shutdown
                self.shutdown();
            }
        } else {
            defmt::error!("Time is in the past!?");
        }
        if can_alive {
            key_in_ezs = self.ezs_a1.get_KL_15R_EIN();

            let _ = self.amp_mute.set_state(key_in_ezs.into());
            let _ = self.can_c_shutdown.set_state((!key_in_ezs).into()); // Inverse!

            // Process any incommming events from PC now
            let _ = self.amp_mute.set_state((self.amp_on & key_in_ezs).into());

            // Read ADC on our channels and Tx the data to CAN-E
            let adc_raw_vsense = self.adc.read(&mut self.adc_vsense).await;
            let adc_raw_pc_curr = self.adc.read(&mut self.adc_pc_csense).await;
            let adc_raw_amp_curr = self.adc.read(&mut self.adc_amp_csense).await;
            let cpu_t = self.adc.read_cpu_temperature(supc).await as u8;

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
                let c_amp = get_current(adc_raw_amp_curr) as u32;
                let vsense = get_in_voltage(adc_raw_vsense);
                if c_amp > self.sensors_accum.c_amp {
                    self.sensors_accum.c_amp = c_amp;
                }
                //self.sensors_accum.c_amp += c_amp as u32;
                self.sensors_accum.c_pc += c_pc as u32;
                self.sensors_accum.v_batt += vsense as u32;
                self.sensors_accum.num_samples += 1;
            }
            

            if let Some(time_since_consumption_tx) = Instant::now().checked_duration_since(self.last_consumption_time) {
                if time_since_consumption_tx.as_millis() >= 250 && self.sensors_accum.num_samples != 0 {
                    let sample_count = self.sensors_accum.num_samples as u32;
                    let c_amp = (self.sensors_accum.c_amp / 1) as u16;
                    let c_pc = (self.sensors_accum.c_pc / sample_count) as u16;
                    let vsense = (self.sensors_accum.v_batt / sample_count) as u16;
                    self.sensors_accum = Default::default();

                    let w_amp = (vsense as u32 * c_amp as u32) as f32 / (1000.0*1000.0);
                    let w_pc = (vsense as u32 * c_pc as u32) as f32 / (1000.0*1000.0);

                    defmt::debug!("PC {}mA ({}W), AMP: {}mA ({}W)  Board: {}C", c_pc, w_pc, c_amp, w_amp, cpu_t);
                    let mut data = [0u8; 8];
                    data[0] = (c_pc & 0xFF) as u8;
                    data[1] = (c_pc >> 8) as u8;
                    data[2] = (c_amp & 0xFF) as u8;
                    data[3] = (c_amp >> 8) as u8;
                    data[4] = (vsense & 0xFF) as u8;
                    data[5] = (vsense >> 8) as u8;
                    data[6] = cpu_t;
                    let f_cane = SerialCanFrame::new(crate::can::CanNet::E, 0x0002, &data);
                    CAN_RX_CHANNEL.sender().send(f_cane).await;
                    self.last_consumption_time = Instant::now();
                }   
            } else {
                defmt::error!("Time is in the past!?");
            }
        }
        key_in_ezs
    }
}
