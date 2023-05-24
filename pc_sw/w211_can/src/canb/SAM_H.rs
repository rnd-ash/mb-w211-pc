
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'SAM_H'
*/
    
pub const SAM_H_A1_CAN_ID: u16 = 0x0004;
pub const SAM_H_A2_CAN_ID: u16 = 0x0090;
pub const SAM_H_A3_CAN_ID: u16 = 0x000E;
pub const SAM_H_A5_CAN_ID: u16 = 0x0230;
pub const SAM_H_A7_CAN_ID: u16 = 0x016C;
pub const SD_RS_SAM_H_CAN_ID: u16 = 0x07C3;


pub struct SAM_H_A1(u64);

impl SAM_H_A1 {

	/// Gets CAN ID of SAM_H_A1
	pub fn get_canid() -> u16 { SAM_H_A1_CAN_ID }
    /// Sets Panic alarm is active

    pub fn set_PNK_AKT(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Panic alarm is active
    pub fn get_PNK_AKT(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Terminal 54 hardware active

    pub fn set_KL54_RM(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Terminal 54 hardware active
    pub fn get_KL54_RM(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Trunk lid contact pressed

    pub fn set_HDK_BET(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Trunk lid contact pressed
    pub fn get_HDK_BET(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Boot lid is open

    pub fn set_HD_AUF(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Boot lid is open
    pub fn get_HD_AUF(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Rear right door is open

    pub fn set_THR_AUF(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Rear right door is open
    pub fn get_THR_AUF(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Rear left door is open

    pub fn set_THL_AUF(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Rear left door is open
    pub fn get_THL_AUF(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Front right door is open

    pub fn set_TVR_AUF(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Front right door is open
    pub fn get_TVR_AUF(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Front left door is open

    pub fn set_TVL_AUF(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Front left door is open
    pub fn get_TVL_AUF(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Bonnet is up

    pub fn set_MOT_AUF(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets Bonnet is up
    pub fn get_MOT_AUF(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets heat. Rear window is switched off due to undervoltage.

    pub fn set_HHS_ST_USPG(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets heat. Rear window is switched off due to undervoltage.
    pub fn get_HHS_ST_USPG(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets Heated rear window is switched on

    pub fn set_HHS_ST_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets Heated rear window is switched on
    pub fn get_HHS_ST_EIN(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets Rear lock in 90° position

    pub fn set_HSCHL_ZU(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets Rear lock in 90° position
    pub fn get_HSCHL_ZU(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets SAM/H passive

    pub fn set_SAM_H_PAS(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets SAM/H passive
    pub fn get_SAM_H_PAS(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets Switch on EDW interior light

    pub fn set_EDW_IL_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets Switch on EDW interior light
    pub fn get_EDW_IL_EIN(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets EDW sharpened

    pub fn set_EDW_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets EDW sharpened
    pub fn get_EDW_AKT(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets Activate EDW interior protection

    pub fn set_EDW_IRS_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets Activate EDW interior protection
    pub fn get_EDW_IRS_AKT(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets Activate EDW trailer monitoring

    pub fn set_EDW_AAG_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets Activate EDW trailer monitoring
    pub fn get_EDW_AAG_AKT(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets EDW alarm triggered

    pub fn set_EDW_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets EDW alarm triggered
    pub fn get_EDW_ALARM(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets ATA activated when driver's door is open

    pub fn set_EDW_FT_ENT(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets ATA activated when driver's door is open
    pub fn get_EDW_FT_ENT(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets EDW self-sharpening "Belgium" active

    pub fn set_EDW_AUTO_AKTIV(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets EDW self-sharpening "Belgium" active
    pub fn get_EDW_AUTO_AKTIV(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets Left license plate light defective

    pub fn set_KZL_DEF_L(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets Left license plate light defective
    pub fn get_KZL_DEF_L(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets Left reverse light defective

    pub fn set_RFL_DEF_L(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets Left reverse light defective
    pub fn get_RFL_DEF_L(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets Left brake light defective

    pub fn set_BL_DEF_L(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets Left brake light defective
    pub fn get_BL_DEF_L(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets Left tail light defective

    pub fn set_SL_DEF_L(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets Left tail light defective
    pub fn get_SL_DEF_L(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
    /// Sets Turn signal rear left defective

    pub fn set_BLI_DEF_HL(&mut self, value: bool){ self.0 = (self.0 & 0xfffffff7ffffffff) | ((value as u64) & 0x1) << 35; }

    /// Gets Turn signal rear left defective
    pub fn get_BLI_DEF_HL(&self) -> bool { (self.0 >> 35 & 0x1) != 0 }
        
    /// Sets Left rear fog light defective

    pub fn set_NSL_DEF_L(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffbffffffff) | ((value as u64) & 0x1) << 34; }

    /// Gets Left rear fog light defective
    pub fn get_NSL_DEF_L(&self) -> bool { (self.0 >> 34 & 0x1) != 0 }
        
    /// Sets 3rd brake light defective

    pub fn set_BL3_DEF(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffdffffffff) | ((value as u64) & 0x1) << 33; }

    /// Gets 3rd brake light defective
    pub fn get_BL3_DEF(&self) -> bool { (self.0 >> 33 & 0x1) != 0 }
        
    /// Sets Terminal 54 error

    pub fn set_KL_54_DEF(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffeffffffff) | ((value as u64) & 0x1) << 32; }

    /// Gets Terminal 54 error
    pub fn get_KL_54_DEF(&self) -> bool { (self.0 >> 32 & 0x1) != 0 }
        
    /// Sets Right license plate light defective

    pub fn set_KZL_DEF_R(&mut self, value: bool){ self.0 = (self.0 & 0xffffffff7fffffff) | ((value as u64) & 0x1) << 31; }

    /// Gets Right license plate light defective
    pub fn get_KZL_DEF_R(&self) -> bool { (self.0 >> 31 & 0x1) != 0 }
        
    /// Sets Right reversing light defective

    pub fn set_RFL_DEF_R(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffbfffffff) | ((value as u64) & 0x1) << 30; }

    /// Gets Right reversing light defective
    pub fn get_RFL_DEF_R(&self) -> bool { (self.0 >> 30 & 0x1) != 0 }
        
    /// Sets Right brake light defective

    pub fn set_BL_DEF_R(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffdfffffff) | ((value as u64) & 0x1) << 29; }

    /// Gets Right brake light defective
    pub fn get_BL_DEF_R(&self) -> bool { (self.0 >> 29 & 0x1) != 0 }
        
    /// Sets Right tail light defective

    pub fn set_SL_DEF_R(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffefffffff) | ((value as u64) & 0x1) << 28; }

    /// Gets Right tail light defective
    pub fn get_SL_DEF_R(&self) -> bool { (self.0 >> 28 & 0x1) != 0 }
        
    /// Sets Turn signal rear right defective

    pub fn set_BLI_DEF_HR(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffff7ffffff) | ((value as u64) & 0x1) << 27; }

    /// Gets Turn signal rear right defective
    pub fn get_BLI_DEF_HR(&self) -> bool { (self.0 >> 27 & 0x1) != 0 }
        
    /// Sets Right rear fog light defective

    pub fn set_NSL_DEF_R(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffbffffff) | ((value as u64) & 0x1) << 26; }

    /// Gets Right rear fog light defective
    pub fn get_NSL_DEF_R(&self) -> bool { (self.0 >> 26 & 0x1) != 0 }
        
    /// Sets Replacement brake light, rear right

    pub fn set_BL_ERS_HR(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffbfffff) | ((value as u64) & 0x1) << 22; }

    /// Gets Replacement brake light, rear right
    pub fn get_BL_ERS_HR(&self) -> bool { (self.0 >> 22 & 0x1) != 0 }
        
    /// Sets Replacement tail light rear right active

    pub fn set_SL_ERS_HR(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffdfffff) | ((value as u64) & 0x1) << 21; }

    /// Gets Replacement tail light rear right active
    pub fn get_SL_ERS_HR(&self) -> bool { (self.0 >> 21 & 0x1) != 0 }
        
    /// Sets Replacement indicator light, rear right active

    pub fn set_BLI_ERS_HR(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffefffff) | ((value as u64) & 0x1) << 20; }

    /// Gets Replacement indicator light, rear right active
    pub fn get_BLI_ERS_HR(&self) -> bool { (self.0 >> 20 & 0x1) != 0 }
        
    /// Sets Replacement brake light, rear left

    pub fn set_BL_ERS_HL(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffbffff) | ((value as u64) & 0x1) << 18; }

    /// Gets Replacement brake light, rear left
    pub fn get_BL_ERS_HL(&self) -> bool { (self.0 >> 18 & 0x1) != 0 }
        
    /// Sets Replacement tail light rear left active

    pub fn set_SL_ERS_HL(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets Replacement tail light rear left active
    pub fn get_SL_ERS_HL(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets Spare light indicator rear left active

    pub fn set_BLI_ERS_HL(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets Spare light indicator rear left active
    pub fn get_BLI_ERS_HL(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
    /// Sets Turn on HFS locator lights

    pub fn set_HFS_SB_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffefff) | ((value as u64) & 0x1) << 12; }

    /// Gets Turn on HFS locator lights
    pub fn get_HFS_SB_EIN(&self) -> bool { (self.0 >> 12 & 0x1) != 0 }
        
    /// Sets Right rear seat locked

    pub fn set_FS_VER_R(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffff7ff) | ((value as u64) & 0x1) << 11; }

    /// Gets Right rear seat locked
    pub fn get_FS_VER_R(&self) -> bool { (self.0 >> 11 & 0x1) != 0 }
        
    /// Sets Left rear seat locked

    pub fn set_FS_VER_L(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffbff) | ((value as u64) & 0x1) << 10; }

    /// Gets Left rear seat locked
    pub fn get_FS_VER_L(&self) -> bool { (self.0 >> 10 & 0x1) != 0 }
        
    /// Sets Right rear seat unlocked

    pub fn set_FS_ENT_R(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffdff) | ((value as u64) & 0x1) << 9; }

    /// Gets Right rear seat unlocked
    pub fn get_FS_ENT_R(&self) -> bool { (self.0 >> 9 & 0x1) != 0 }
        
    /// Sets Left rear seat unlocked

    pub fn set_FS_ENT_L(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffeff) | ((value as u64) & 0x1) << 8; }

    /// Gets Left rear seat unlocked
    pub fn get_FS_ENT_L(&self) -> bool { (self.0 >> 8 & 0x1) != 0 }
        
    /// Sets Turn on the dimmed right rear fog light

    pub fn set_NSL_R_D_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffdf) | ((value as u64) & 0x1) << 5; }

    /// Gets Turn on the dimmed right rear fog light
    pub fn get_NSL_R_D_EIN(&self) -> bool { (self.0 >> 5 & 0x1) != 0 }
        
    /// Sets Switch on left fog lamp dimmed

    pub fn set_NSL_L_D_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffef) | ((value as u64) & 0x1) << 4; }

    /// Gets Switch on left fog lamp dimmed
    pub fn get_NSL_L_D_EIN(&self) -> bool { (self.0 >> 4 & 0x1) != 0 }
        
    /// Sets Turn on the right rear fog light

    pub fn set_NSL_R_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffff7) | ((value as u64) & 0x1) << 3; }

    /// Gets Turn on the right rear fog light
    pub fn get_NSL_R_EIN(&self) -> bool { (self.0 >> 3 & 0x1) != 0 }
        
    /// Sets Turn on the left rear fog light

    pub fn set_NSL_L_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffb) | ((value as u64) & 0x1) << 2; }

    /// Gets Turn on the left rear fog light
    pub fn get_NSL_L_EIN(&self) -> bool { (self.0 >> 2 & 0x1) != 0 }
        
    /// Sets Turn on the right reversing light

    pub fn set_RFL_R_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffd) | ((value as u64) & 0x1) << 1; }

    /// Gets Turn on the right reversing light
    pub fn get_RFL_R_EIN(&self) -> bool { (self.0 >> 1 & 0x1) != 0 }
        
    /// Sets Turn on the left reversing light

    pub fn set_RFL_L_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffe) | ((value as u64) & 0x1) << 0; }

    /// Gets Turn on the left reversing light
    pub fn get_RFL_L_EIN(&self) -> bool { (self.0 >> 0 & 0x1) != 0 }
        
}
pub struct SAM_H_A2(u64);

impl SAM_H_A2 {

	/// Gets CAN ID of SAM_H_A2
	pub fn get_canid() -> u16 { SAM_H_A2_CAN_ID }
    /// Sets tank level. Conversion formula (To raw from real): y=(x-0.0)/0.50 (Unit: %)

    pub fn set_TANK_FS_B(&mut self, value: u8){ self.0 = (self.0 & 0x00ffffffffffffff) | ((value as u64) & 0xff) << 56; }

    /// Gets tank level. Conversion formula (To real from raw): y=(0.50x)+0.0 (Unit: %)
    pub fn get_TANK_FS_B(&self) -> u8 { (self.0 >> 56 & 0xff) as u8 }
        
    /// Sets Right tank sender value. Conversion formula (To raw from real): y=(x-0.0)/0.50 (Unit: %)

    pub fn set_TANK_GE_RE(&mut self, value: u8){ self.0 = (self.0 & 0xff00ffffffffffff) | ((value as u64) & 0xff) << 48; }

    /// Gets Right tank sender value. Conversion formula (To real from raw): y=(0.50x)+0.0 (Unit: %)
    pub fn get_TANK_GE_RE(&self) -> u8 { (self.0 >> 48 & 0xff) as u8 }
        
    /// Sets Tank sensor value on the left. Conversion formula (To raw from real): y=(x-0.0)/0.50 (Unit: %)

    pub fn set_TANK_GE_LI(&mut self, value: u8){ self.0 = (self.0 & 0xffff00ffffffffff) | ((value as u64) & 0xff) << 40; }

    /// Gets Tank sensor value on the left. Conversion formula (To real from raw): y=(0.50x)+0.0 (Unit: %)
    pub fn get_TANK_GE_LI(&self) -> u8 { (self.0 >> 40 & 0xff) as u8 }
        
}
pub struct SAM_H_A3(u64);

impl SAM_H_A3 {

	/// Gets CAN ID of SAM_H_A3
	pub fn get_canid() -> u16 { SAM_H_A3_CAN_ID }
    /// Sets Turn on the right turn signal

    pub fn set_BLI_RE_EIN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Turn on the right turn signal
    pub fn get_BLI_RE_EIN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Turn on the left turn signal

    pub fn set_BLI_LI_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Turn on the left turn signal
    pub fn get_BLI_LI_EIN(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Hazard warning lights active

    pub fn set_WARN_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Hazard warning lights active
    pub fn get_WARN_AKT(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets flashing light phase. Conversion formula (To raw from real): y=(x-0.0)/10.00 (Unit: ms)

    pub fn set_HELL_BLINK(&mut self, value: u8){ self.0 = (self.0 & 0xff00ffffffffffff) | ((value as u64) & 0xff) << 48; }

    /// Gets flashing light phase. Conversion formula (To real from raw): y=(10.00x)+0.0 (Unit: ms)
    pub fn get_HELL_BLINK(&self) -> u8 { (self.0 >> 48 & 0xff) as u8 }
        
}
pub struct SAM_H_A5(u64);

impl SAM_H_A5 {

	/// Gets CAN ID of SAM_H_A5
	pub fn get_canid() -> u16 { SAM_H_A5_CAN_ID }
    /// Sets Turn on fog lights

    pub fn set_NSW_EIN_EDW(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Turn on fog lights
    pub fn get_NSW_EIN_EDW(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Turn on low beam

    pub fn set_ABL_EIN_EDW(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Turn on low beam
    pub fn get_ABL_EIN_EDW(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Turn on tail light

    pub fn set_SL_EIN_EDW(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Turn on tail light
    pub fn get_SL_EIN_EDW(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Duration of light phase. Conversion formula (To raw from real): y=(x-0.0)/10.00 (Unit: ms)

    pub fn set_HELL_EDW(&mut self, value: u8){ self.0 = (self.0 & 0xff00ffffffffffff) | ((value as u64) & 0xff) << 48; }

    /// Gets Duration of light phase. Conversion formula (To real from raw): y=(10.00x)+0.0 (Unit: ms)
    pub fn get_HELL_EDW(&self) -> u8 { (self.0 >> 48 & 0xff) as u8 }
        
}
pub struct SAM_H_A7(u64);

impl SAM_H_A7 {

	/// Gets CAN ID of SAM_H_A7
	pub fn get_canid() -> u16 { SAM_H_A7_CAN_ID }
    /// Sets Close the tailgate button

    pub fn set_TST_HFS_SCHL(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Close the tailgate button
    pub fn get_TST_HFS_SCHL(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Open tailgate button

    pub fn set_TST_HFS_OEFF(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Open tailgate button
    pub fn get_TST_HFS_OEFF(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
}
pub struct SD_RS_SAM_H(u64);

impl SD_RS_SAM_H {

	/// Gets CAN ID of SD_RS_SAM_H
	pub fn get_canid() -> u16 { SD_RS_SAM_H_CAN_ID }
    /// Sets Identification for > 8 bytes

    pub fn set_SAM_H_KENN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Identification for > 8 bytes
    pub fn get_SAM_H_KENN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets state variable 07h

    pub fn set_SAM_H_PGV07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets state variable 07h
    pub fn get_SAM_H_PGV07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets state variable 06h

    pub fn set_SAM_H_PGV06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets state variable 06h
    pub fn get_SAM_H_PGV06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets state variable 05h

    pub fn set_SAM_H_PGV05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets state variable 05h
    pub fn get_SAM_H_PGV05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets state variable 04h

    pub fn set_SAM_H_PGV04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets state variable 04h
    pub fn get_SAM_H_PGV04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets state variable 03h

    pub fn set_SAM_H_PGV03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets state variable 03h
    pub fn get_SAM_H_PGV03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets state variable 02h

    pub fn set_SAM_H_PGV02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets state variable 02h
    pub fn get_SAM_H_PGV02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets state variable 01h

    pub fn set_SAM_H_PGV01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets state variable 01h
    pub fn get_SAM_H_PGV01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Error message 01h. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_SAM_H_FM01(&mut self, value: u16){ self.0 = (self.0 & 0xff0000ffffffffff) | ((value as u64) & 0xffff) << 40; }

    /// Gets Error message 01h. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_SAM_H_FM01(&self) -> u16 { (self.0 >> 40 & 0xffff) as u16 }
        
    /// Sets Error message 02h. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_SAM_H_FM02(&mut self, value: u16){ self.0 = (self.0 & 0xffffff0000ffffff) | ((value as u64) & 0xffff) << 24; }

    /// Gets Error message 02h. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_SAM_H_FM02(&self) -> u16 { (self.0 >> 24 & 0xffff) as u16 }
        
    /// Sets Error message 03h. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_SAM_H_FM03(&mut self, value: u16){ self.0 = (self.0 & 0xffffffffff0000ff) | ((value as u64) & 0xffff) << 8; }

    /// Gets Error message 03h. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_SAM_H_FM03(&self) -> u16 { (self.0 >> 8 & 0xffff) as u16 }
        
    /// Sets state variable 0Fh

    pub fn set_SAM_H_PGV0F(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffff7f) | ((value as u64) & 0x1) << 7; }

    /// Gets state variable 0Fh
    pub fn get_SAM_H_PGV0F(&self) -> bool { (self.0 >> 7 & 0x1) != 0 }
        
    /// Sets state variable 0Eh

    pub fn set_SAM_H_PGV0E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffbf) | ((value as u64) & 0x1) << 6; }

    /// Gets state variable 0Eh
    pub fn get_SAM_H_PGV0E(&self) -> bool { (self.0 >> 6 & 0x1) != 0 }
        
    /// Sets State variable 0Dh

    pub fn set_SAM_H_PGV0D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffdf) | ((value as u64) & 0x1) << 5; }

    /// Gets State variable 0Dh
    pub fn get_SAM_H_PGV0D(&self) -> bool { (self.0 >> 5 & 0x1) != 0 }
        
    /// Sets state variable 0Ch

    pub fn set_SAM_H_PGV0C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffef) | ((value as u64) & 0x1) << 4; }

    /// Gets state variable 0Ch
    pub fn get_SAM_H_PGV0C(&self) -> bool { (self.0 >> 4 & 0x1) != 0 }
        
    /// Sets state variable 0Bh

    pub fn set_SAM_H_PGV0B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffff7) | ((value as u64) & 0x1) << 3; }

    /// Gets state variable 0Bh
    pub fn get_SAM_H_PGV0B(&self) -> bool { (self.0 >> 3 & 0x1) != 0 }
        
    /// Sets State variable 0Ah

    pub fn set_SAM_H_PGV0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffb) | ((value as u64) & 0x1) << 2; }

    /// Gets State variable 0Ah
    pub fn get_SAM_H_PGV0A(&self) -> bool { (self.0 >> 2 & 0x1) != 0 }
        
    /// Sets state variable 09h

    pub fn set_SAM_H_PGV09(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffd) | ((value as u64) & 0x1) << 1; }

    /// Gets state variable 09h
    pub fn get_SAM_H_PGV09(&self) -> bool { (self.0 >> 1 & 0x1) != 0 }
        
    /// Sets state variable 08h

    pub fn set_SAM_H_PGV08(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffe) | ((value as u64) & 0x1) << 0; }

    /// Gets state variable 08h
    pub fn get_SAM_H_PGV08(&self) -> bool { (self.0 >> 0 & 0x1) != 0 }
        
}