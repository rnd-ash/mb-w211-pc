
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'MSS'
*/
    
pub const MSS_A1_CAN_ID: u16 = 0x0015;
pub const MSS_A2_CAN_ID: u16 = 0x01AE;
pub const MSS_A3_CAN_ID: u16 = 0x01CE;
pub const MSS_A4_CAN_ID: u16 = 0x0248;
pub const MSSK_A1_CAN_ID: u16 = 0x0046;
pub const MSSK_A2_CAN_ID: u16 = 0x0208;
pub const SD_RS_MSS_CAN_ID: u16 = 0x07C6;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MSS_A1(pub u64);

impl MSS_A1 {

	/// Gets CAN ID of MSS_A1
	pub const fn get_canid() -> u16 { MSS_A1_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Turn on high beam

    pub fn set_FL_EIN_MSS(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Turn on high beam
    pub fn get_FL_EIN_MSS(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Turn on fog lights

    pub fn set_NSW_EIN_MSS(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Turn on fog lights
    pub fn get_NSW_EIN_MSS(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Turn on the horn

    pub fn set_SGH_EIN_MSS(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Turn on the horn
    pub fn get_SGH_EIN_MSS(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets MSS blink on

    pub fn set_BLI_EIN_MSS(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets MSS blink on
    pub fn get_BLI_EIN_MSS(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Drop off a taxi call for help

    pub fn set_T_HIRU_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Drop off a taxi call for help
    pub fn get_T_HIRU_EIN(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Close recirculation flaps

    pub fn set_UMLUFT_MSS(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Close recirculation flaps
    pub fn get_UMLUFT_MSS(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Turn on interior light

    pub fn set_IL_EIN_MSS(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Turn on interior light
    pub fn get_IL_EIN_MSS(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Close central locking

    pub fn set_ZV_ZU_MSS(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Close central locking
    pub fn get_ZV_ZU_MSS(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Duration of blinker light phase. Conversion formula (To raw from real): y=(x-0.0)/10.00 (Unit: ms)

    pub fn set_BLI_HELL_MSS(&mut self, value: u8){ self.0 = (self.0 & 0xff00ffffffffffff) | ((value as u64) & 0xff) << 48; }

    /// Gets Duration of blinker light phase. Conversion formula (To real from raw): y=(10.00x)+0.0 (Unit: ms)
    pub fn get_BLI_HELL_MSS(&self) -> u8 { (self.0 >> 48 & 0xff) as u8 }
        
    /// Sets duration of the horn. Conversion formula (To raw from real): y=(x-0.0)/10.00 (Unit: ms)

    pub fn set_SGH_AN_MSS(&mut self, value: u8){ self.0 = (self.0 & 0xffff00ffffffffff) | ((value as u64) & 0xff) << 40; }

    /// Gets duration of the horn. Conversion formula (To real from raw): y=(10.00x)+0.0 (Unit: ms)
    pub fn get_SGH_AN_MSS(&self) -> u8 { (self.0 >> 40 & 0xff) as u8 }
        
    /// Sets Duration fog light bright phase. Conversion formula (To raw from real): y=(x-0.0)/10.00 (Unit: ms)

    pub fn set_NSW_HELL_MSS(&mut self, value: u8){ self.0 = (self.0 & 0xffffff00ffffffff) | ((value as u64) & 0xff) << 32; }

    /// Gets Duration fog light bright phase. Conversion formula (To real from raw): y=(10.00x)+0.0 (Unit: ms)
    pub fn get_NSW_HELL_MSS(&self) -> u8 { (self.0 >> 32 & 0xff) as u8 }
        
    /// Sets Duration high beam light phase. Conversion formula (To raw from real): y=(x-0.0)/10.00 (Unit: ms)

    pub fn set_FL_HELL_MSS(&mut self, value: u8){ self.0 = (self.0 & 0xffffffff00ffffff) | ((value as u64) & 0xff) << 24; }

    /// Gets Duration high beam light phase. Conversion formula (To real from raw): y=(10.00x)+0.0 (Unit: ms)
    pub fn get_FL_HELL_MSS(&self) -> u8 { (self.0 >> 24 & 0xff) as u8 }
        
    /// Sets Radio override active

    pub fn set_FU_FRSP_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffff7fffff) | ((value as u64) & 0x1) << 23; }

    /// Gets Radio override active
    pub fn get_FU_FRSP_AKT(&self) -> bool { (self.0 >> 23 & 0x1) != 0 }
        
    /// Sets Switch on the authority radio LED

    pub fn set_BHF_LED_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffdfffff) | ((value as u64) & 0x1) << 21; }

    /// Gets Switch on the authority radio LED
    pub fn get_BHF_LED_AKT(&self) -> bool { (self.0 >> 21 & 0x1) != 0 }
        
    /// Sets Request numeric keypad HU

    pub fn set_ANF_ZT(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffefffff) | ((value as u64) & 0x1) << 20; }

    /// Gets Request numeric keypad HU
    pub fn get_ANF_ZT(&self) -> bool { (self.0 >> 20 & 0x1) != 0 }
        
    /// Sets 220V external charging is connected

    pub fn set_LADEN_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffff7ffff) | ((value as u64) & 0x1) << 19; }

    /// Gets 220V external charging is connected
    pub fn get_LADEN_AKT(&self) -> bool { (self.0 >> 19 & 0x1) != 0 }
        
    /// Sets Muting of the audio source with radio reception

    pub fn set_AUDIO_MUTE2(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffbffff) | ((value as u64) & 0x1) << 18; }

    /// Gets Muting of the audio source with radio reception
    pub fn get_AUDIO_MUTE2(&self) -> bool { (self.0 >> 18 & 0x1) != 0 }
        
    /// Sets Audio Mute

    pub fn set_AUDIO_MUTE1(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets Audio Mute
    pub fn get_AUDIO_MUTE1(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets Turn on the siren

    pub fn set_SIR_EIN_MSS(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets Turn on the siren
    pub fn get_SIR_EIN_MSS(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MSS_A2(pub u64);

impl MSS_A2 {

	/// Gets CAN ID of MSS_A2
	pub const fn get_canid() -> u16 { MSS_A2_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Roof sign indicator light on

    pub fn set_DZ_KL(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Roof sign indicator light on
    pub fn get_DZ_KL(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Lamp defect roof sign

    pub fn set_DZ_LA_DEF(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Lamp defect roof sign
    pub fn get_DZ_LA_DEF(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Check roof sign (break)

    pub fn set_DZ_PRF(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Check roof sign (break)
    pub fn get_DZ_PRF(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Roof sign defective (short circuit)

    pub fn set_DZ_DEF(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Roof sign defective (short circuit)
    pub fn get_DZ_DEF(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets MSS detects undervoltage

    pub fn set_MSS_USPG(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets MSS detects undervoltage
    pub fn get_MSS_USPG(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Silent alarm triggered

    pub fn set_MSS_ALM(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Silent alarm triggered
    pub fn get_MSS_ALM(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Emergency alarm system defective

    pub fn set_NOTALM_DEF(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Emergency alarm system defective
    pub fn get_NOTALM_DEF(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Electronics on the MSS defective

    pub fn set_MSS_EE_DEF(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Electronics on the MSS defective
    pub fn get_MSS_EE_DEF(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Radio Status on [1] off [0]

    pub fn set_FNK_STAT(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets Radio Status on [1] off [0]
    pub fn get_FNK_STAT(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets Activate the buzzer in the station wagon

    pub fn set_MSS_SUMMER(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets Activate the buzzer in the station wagon
    pub fn get_MSS_SUMMER(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets MSS detects undervoltage when motor is running

    pub fn set_MSS_USPG_MO(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets MSS detects undervoltage when motor is running
    pub fn get_MSS_USPG_MO(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets Windshield Hzg. malfunction

    pub fn set_FSB_FEHLER(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets Windshield Hzg. malfunction
    pub fn get_FSB_FEHLER(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets Windshield Hzg. from wg. timer

    pub fn set_FSB_HZG_AUS(&mut self, value: bool){ self.0 = (self.0 & 0xfffffeffffffffff) | ((value as u64) & 0x1) << 40; }

    /// Gets Windshield Hzg. from wg. timer
    pub fn get_FSB_HZG_AUS(&self) -> bool { (self.0 >> 40 & 0x1) != 0 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MSS_A3(pub u64);

impl MSS_A3 {

	/// Gets CAN ID of MSS_A3
	pub const fn get_canid() -> u16 { MSS_A3_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets GPS Latitude, South = [-]; North = [+]. Conversion formula (To raw from real): y=(x+648000000.0)/0.30 (Unit: ms)

    pub fn set_DEST_LAT(&mut self, value: u32){ self.0 = (self.0 & 0x00000000ffffffff) | ((value as u64) & 0xffffffff) << 32; }

    /// Gets GPS Latitude, South = [-]; North = [+]. Conversion formula (To real from raw): y=(0.30x)-648000000.0 (Unit: ms)
    pub fn get_DEST_LAT(&self) -> u32 { (self.0 >> 32 & 0xffffffff) as u32 }
        
    /// Sets GPS Longitude, West = [-]; East = [+]. Conversion formula (To raw from real): y=(x+648000000.0)/0.30 (Unit: ms)

    pub fn set_DEST_LONG(&mut self, value: u32){ self.0 = (self.0 & 0xffffffff00000000) | ((value as u64) & 0xffffffff); }

    /// Gets GPS Longitude, West = [-]; East = [+]. Conversion formula (To real from raw): y=(0.30x)-648000000.0 (Unit: ms)
    pub fn get_DEST_LONG(&self) -> u32 { (self.0 & 0xffffffff) as u32 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MSS_A4(pub u64);

impl MSS_A4 {

	/// Gets CAN ID of MSS_A4
	pub const fn get_canid() -> u16 { MSS_A4_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Close the rear right window

    pub fn set_FHR_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Close the rear right window
    pub fn get_FHR_ALARM(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Close rear left window

    pub fn set_FHL_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Close rear left window
    pub fn get_FHL_ALARM(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Close the front right window

    pub fn set_FVR_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Close the front right window
    pub fn get_FVR_ALARM(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Close the front left window

    pub fn set_FVL_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Close the front left window
    pub fn get_FVL_ALARM(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Close SHD

    pub fn set_SHD_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Close SHD
    pub fn get_SHD_ALARM(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Direction of alarm actuation: Open [0], Close [1]

    pub fn set_RI_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Direction of alarm actuation: Open [0], Close [1]
    pub fn get_RI_ALARM(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Automatic run [1], Manual [0]

    pub fn set_MOD_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Automatic run [1], Manual [0]
    pub fn get_MOD_ALARM(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MSSK_A1(pub u64);

impl MSSK_A1 {

	/// Gets CAN ID of MSSK_A1
	pub const fn get_canid() -> u16 { MSSK_A1_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Turn on the horn

    pub fn set_SGH_EIN_K(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Turn on the horn
    pub fn get_SGH_EIN_K(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Switch on headlight flasher

    pub fn set_LHP_EIN_K(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Switch on headlight flasher
    pub fn get_LHP_EIN_K(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Turn on high beam

    pub fn set_FL_EIN_K(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Turn on high beam
    pub fn get_FL_EIN_K(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Turn signal right

    pub fn set_BLI_RE_K(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Turn signal right
    pub fn get_BLI_RE_K(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Turn signal left

    pub fn set_BLI_LI_K(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Turn signal left
    pub fn get_BLI_LI_K(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Switch on windscreen wipers stage 2

    pub fn set_SCH_WI_2_K(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets Switch on windscreen wipers stage 2
    pub fn get_SCH_WI_2_K(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets Switch on wiper speed 1

    pub fn set_SCH_WI_1_K(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets Switch on wiper speed 1
    pub fn get_SCH_WI_1_K(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets Switch on wiper interval

    pub fn set_SCH_WI_INT_K(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets Switch on wiper interval
    pub fn get_SCH_WI_INT_K(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets Turn on washing

    pub fn set_WASCHEN_K(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets Turn on washing
    pub fn get_WASCHEN_K(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets Turn on swiping

    pub fn set_TIPP_WISCH_K(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets Turn on swiping
    pub fn get_TIPP_WISCH_K(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets Intermittent rear window wipe

    pub fn set_HECK_INT_K(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets Intermittent rear window wipe
    pub fn get_HECK_INT_K(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets wipe/wash rear window

    pub fn set_HECK_WISCH_K(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets wipe/wash rear window
    pub fn get_HECK_WISCH_K(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets Hazard warning lights on

    pub fn set_WBL_EIN_K(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets Hazard warning lights on
    pub fn get_WBL_EIN_K(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets Turn on parking lights

    pub fn set_STL_EIN_K(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets Turn on parking lights
    pub fn get_STL_EIN_K(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets Turn on low beam

    pub fn set_ABL_EIN_K(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets Turn on low beam
    pub fn get_ABL_EIN_K(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets Turn on fog lights

    pub fn set_NSW_EIN_K(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets Turn on fog lights
    pub fn get_NSW_EIN_K(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets Turn on rear fog light

    pub fn set_NSL_EIN_K(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets Turn on rear fog light
    pub fn get_NSL_EIN_K(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets sunroof stop

    pub fn set_SHD_STOP(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets sunroof stop
    pub fn get_SHD_STOP(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets Open lifting roof

    pub fn set_HD_AUF_K(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets Open lifting roof
    pub fn get_HD_AUF_K(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets Close sunroof

    pub fn set_SHD_ZU_K(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets Close sunroof
    pub fn get_SHD_ZU_K(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets Open sunroof

    pub fn set_SHD_AUF_K(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets Open sunroof
    pub fn get_SHD_AUF_K(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MSSK_A2(pub u64);

impl MSSK_A2 {

	/// Gets CAN ID of MSSK_A2
	pub const fn get_canid() -> u16 { MSSK_A2_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Passenger seat - toggle bit

    pub fn set_SBF_K_TGL(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Passenger seat - toggle bit
    pub fn get_SBF_K_TGL(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Passenger seat - lengthways back

    pub fn set_SBF_ZUR_K(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Passenger seat - lengthways back
    pub fn get_SBF_ZUR_K(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Passenger seat - longitudinally forward

    pub fn set_SBF_VOR_K(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Passenger seat - longitudinally forward
    pub fn get_SBF_VOR_K(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SD_RS_MSS(pub u64);

impl SD_RS_MSS {

	/// Gets CAN ID of SD_RS_MSS
	pub const fn get_canid() -> u16 { SD_RS_MSS_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Identification for > 8 bytes

    pub fn set_MSS_KENN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Identification for > 8 bytes
    pub fn get_MSS_KENN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets error vector 07h

    pub fn set_MSS_FV07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets error vector 07h
    pub fn get_MSS_FV07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets error vector 06h

    pub fn set_MSS_FV06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets error vector 06h
    pub fn get_MSS_FV06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets error vector 05h

    pub fn set_MSS_FV05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets error vector 05h
    pub fn get_MSS_FV05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets error vector 04h

    pub fn set_MSS_FV04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets error vector 04h
    pub fn get_MSS_FV04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets error vector 03h

    pub fn set_MSS_FV03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets error vector 03h
    pub fn get_MSS_FV03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets error vector 02h

    pub fn set_MSS_FV02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets error vector 02h
    pub fn get_MSS_FV02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets error vector 01h

    pub fn set_MSS_FV01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets error vector 01h
    pub fn get_MSS_FV01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets error vector 0Fh

    pub fn set_MSS_FV0F(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets error vector 0Fh
    pub fn get_MSS_FV0F(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets error vector 0Eh

    pub fn set_MSS_FV0E(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets error vector 0Eh
    pub fn get_MSS_FV0E(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets error vector 0Dh

    pub fn set_MSS_FV0D(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets error vector 0Dh
    pub fn get_MSS_FV0D(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets error vector 0Ch

    pub fn set_MSS_FV0C(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets error vector 0Ch
    pub fn get_MSS_FV0C(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets error vector 0Bh

    pub fn set_MSS_FV0B(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets error vector 0Bh
    pub fn get_MSS_FV0B(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets error vector 0Ah

    pub fn set_MSS_FV0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets error vector 0Ah
    pub fn get_MSS_FV0A(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets error vector 09h

    pub fn set_MSS_FV09(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets error vector 09h
    pub fn get_MSS_FV09(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets error vector 08h

    pub fn set_MSS_FV08(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets error vector 08h
    pub fn get_MSS_FV08(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets error vector 17h

    pub fn set_MSS_FV17(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets error vector 17h
    pub fn get_MSS_FV17(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets error vector 16h

    pub fn set_MSS_FV16(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets error vector 16h
    pub fn get_MSS_FV16(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets error vector 15h

    pub fn set_MSS_FV15(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets error vector 15h
    pub fn get_MSS_FV15(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets error vector 14h

    pub fn set_MSS_FV14(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets error vector 14h
    pub fn get_MSS_FV14(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets error vector 13h

    pub fn set_MSS_FV13(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets error vector 13h
    pub fn get_MSS_FV13(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets error vector 12h

    pub fn set_MSS_FV12(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets error vector 12h
    pub fn get_MSS_FV12(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets error vector 11h

    pub fn set_MSS_FV11(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets error vector 11h
    pub fn get_MSS_FV11(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets error vector 10h

    pub fn set_MSS_FV10(&mut self, value: bool){ self.0 = (self.0 & 0xfffffeffffffffff) | ((value as u64) & 0x1) << 40; }

    /// Gets error vector 10h
    pub fn get_MSS_FV10(&self) -> bool { (self.0 >> 40 & 0x1) != 0 }
        
    /// Sets error vector 1Fh

    pub fn set_MSS_FV1F(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets error vector 1Fh
    pub fn get_MSS_FV1F(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets error vector 1Eh

    pub fn set_MSS_FV1E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets error vector 1Eh
    pub fn get_MSS_FV1E(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets error vector 1Dh

    pub fn set_MSS_FV1D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets error vector 1Dh
    pub fn get_MSS_FV1D(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets Error vector 1Ch

    pub fn set_MSS_FV1C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets Error vector 1Ch
    pub fn get_MSS_FV1C(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
    /// Sets error vector 1Bh

    pub fn set_MSS_FV1B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffff7ffffffff) | ((value as u64) & 0x1) << 35; }

    /// Gets error vector 1Bh
    pub fn get_MSS_FV1B(&self) -> bool { (self.0 >> 35 & 0x1) != 0 }
        
    /// Sets Error vector 1Ah

    pub fn set_MSS_FV1A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffbffffffff) | ((value as u64) & 0x1) << 34; }

    /// Gets Error vector 1Ah
    pub fn get_MSS_FV1A(&self) -> bool { (self.0 >> 34 & 0x1) != 0 }
        
    /// Sets error vector 19h

    pub fn set_MSS_FV19(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffdffffffff) | ((value as u64) & 0x1) << 33; }

    /// Gets error vector 19h
    pub fn get_MSS_FV19(&self) -> bool { (self.0 >> 33 & 0x1) != 0 }
        
    /// Sets error vector 18h

    pub fn set_MSS_FV18(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffeffffffff) | ((value as u64) & 0x1) << 32; }

    /// Gets error vector 18h
    pub fn get_MSS_FV18(&self) -> bool { (self.0 >> 32 & 0x1) != 0 }
        
    /// Sets error vector 27h

    pub fn set_MSS_FV27(&mut self, value: bool){ self.0 = (self.0 & 0xffffffff7fffffff) | ((value as u64) & 0x1) << 31; }

    /// Gets error vector 27h
    pub fn get_MSS_FV27(&self) -> bool { (self.0 >> 31 & 0x1) != 0 }
        
    /// Sets error vector 26h

    pub fn set_MSS_FV26(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffbfffffff) | ((value as u64) & 0x1) << 30; }

    /// Gets error vector 26h
    pub fn get_MSS_FV26(&self) -> bool { (self.0 >> 30 & 0x1) != 0 }
        
    /// Sets error vector 25h

    pub fn set_MSS_FV25(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffdfffffff) | ((value as u64) & 0x1) << 29; }

    /// Gets error vector 25h
    pub fn get_MSS_FV25(&self) -> bool { (self.0 >> 29 & 0x1) != 0 }
        
    /// Sets error vector 24h

    pub fn set_MSS_FV24(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffefffffff) | ((value as u64) & 0x1) << 28; }

    /// Gets error vector 24h
    pub fn get_MSS_FV24(&self) -> bool { (self.0 >> 28 & 0x1) != 0 }
        
    /// Sets error vector 23h

    pub fn set_MSS_FV23(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffff7ffffff) | ((value as u64) & 0x1) << 27; }

    /// Gets error vector 23h
    pub fn get_MSS_FV23(&self) -> bool { (self.0 >> 27 & 0x1) != 0 }
        
    /// Sets error vector 22h

    pub fn set_MSS_FV22(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffbffffff) | ((value as u64) & 0x1) << 26; }

    /// Gets error vector 22h
    pub fn get_MSS_FV22(&self) -> bool { (self.0 >> 26 & 0x1) != 0 }
        
    /// Sets error vector 21h

    pub fn set_MSS_FV21(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffdffffff) | ((value as u64) & 0x1) << 25; }

    /// Gets error vector 21h
    pub fn get_MSS_FV21(&self) -> bool { (self.0 >> 25 & 0x1) != 0 }
        
    /// Sets error vector 20h

    pub fn set_MSS_FV20(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffeffffff) | ((value as u64) & 0x1) << 24; }

    /// Gets error vector 20h
    pub fn get_MSS_FV20(&self) -> bool { (self.0 >> 24 & 0x1) != 0 }
        
    /// Sets error vector 2Fh

    pub fn set_MSS_FV2F(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffff7fffff) | ((value as u64) & 0x1) << 23; }

    /// Gets error vector 2Fh
    pub fn get_MSS_FV2F(&self) -> bool { (self.0 >> 23 & 0x1) != 0 }
        
    /// Sets error vector 2Eh

    pub fn set_MSS_FV2E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffbfffff) | ((value as u64) & 0x1) << 22; }

    /// Gets error vector 2Eh
    pub fn get_MSS_FV2E(&self) -> bool { (self.0 >> 22 & 0x1) != 0 }
        
    /// Sets error vector 2Dh

    pub fn set_MSS_FV2D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffdfffff) | ((value as u64) & 0x1) << 21; }

    /// Gets error vector 2Dh
    pub fn get_MSS_FV2D(&self) -> bool { (self.0 >> 21 & 0x1) != 0 }
        
    /// Sets error vector 2Ch

    pub fn set_MSS_FV2C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffefffff) | ((value as u64) & 0x1) << 20; }

    /// Gets error vector 2Ch
    pub fn get_MSS_FV2C(&self) -> bool { (self.0 >> 20 & 0x1) != 0 }
        
    /// Sets error vector 2Bh

    pub fn set_MSS_FV2B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffff7ffff) | ((value as u64) & 0x1) << 19; }

    /// Gets error vector 2Bh
    pub fn get_MSS_FV2B(&self) -> bool { (self.0 >> 19 & 0x1) != 0 }
        
    /// Sets Error vector 2Ah

    pub fn set_MSS_FV2A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffbffff) | ((value as u64) & 0x1) << 18; }

    /// Gets Error vector 2Ah
    pub fn get_MSS_FV2A(&self) -> bool { (self.0 >> 18 & 0x1) != 0 }
        
    /// Sets error vector 29h

    pub fn set_MSS_FV29(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets error vector 29h
    pub fn get_MSS_FV29(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets error vector 28h

    pub fn set_MSS_FV28(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets error vector 28h
    pub fn get_MSS_FV28(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
    /// Sets error vector 37h

    pub fn set_MSS_FV37(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffff7fff) | ((value as u64) & 0x1) << 15; }

    /// Gets error vector 37h
    pub fn get_MSS_FV37(&self) -> bool { (self.0 >> 15 & 0x1) != 0 }
        
    /// Sets error vector 36h

    pub fn set_MSS_FV36(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffbfff) | ((value as u64) & 0x1) << 14; }

    /// Gets error vector 36h
    pub fn get_MSS_FV36(&self) -> bool { (self.0 >> 14 & 0x1) != 0 }
        
    /// Sets error vector 35h

    pub fn set_MSS_FV35(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffdfff) | ((value as u64) & 0x1) << 13; }

    /// Gets error vector 35h
    pub fn get_MSS_FV35(&self) -> bool { (self.0 >> 13 & 0x1) != 0 }
        
    /// Sets error vector 34h

    pub fn set_MSS_FV34(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffefff) | ((value as u64) & 0x1) << 12; }

    /// Gets error vector 34h
    pub fn get_MSS_FV34(&self) -> bool { (self.0 >> 12 & 0x1) != 0 }
        
    /// Sets error vector 33h

    pub fn set_MSS_FV33(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffff7ff) | ((value as u64) & 0x1) << 11; }

    /// Gets error vector 33h
    pub fn get_MSS_FV33(&self) -> bool { (self.0 >> 11 & 0x1) != 0 }
        
    /// Sets error vector 32h

    pub fn set_MSS_FV32(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffbff) | ((value as u64) & 0x1) << 10; }

    /// Gets error vector 32h
    pub fn get_MSS_FV32(&self) -> bool { (self.0 >> 10 & 0x1) != 0 }
        
    /// Sets error vector 31h

    pub fn set_MSS_FV31(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffdff) | ((value as u64) & 0x1) << 9; }

    /// Gets error vector 31h
    pub fn get_MSS_FV31(&self) -> bool { (self.0 >> 9 & 0x1) != 0 }
        
    /// Sets error vector 30h

    pub fn set_MSS_FV30(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffeff) | ((value as u64) & 0x1) << 8; }

    /// Gets error vector 30h
    pub fn get_MSS_FV30(&self) -> bool { (self.0 >> 8 & 0x1) != 0 }
        
    /// Sets state variable 04h

    pub fn set_MSS_PGV04(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffff7f) | ((value as u64) & 0x1) << 7; }

    /// Gets state variable 04h
    pub fn get_MSS_PGV04(&self) -> bool { (self.0 >> 7 & 0x1) != 0 }
        
    /// Sets state variable 03h

    pub fn set_MSS_PGV03(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffbf) | ((value as u64) & 0x1) << 6; }

    /// Gets state variable 03h
    pub fn get_MSS_PGV03(&self) -> bool { (self.0 >> 6 & 0x1) != 0 }
        
    /// Sets state variable 02h

    pub fn set_MSS_PGV02(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffdf) | ((value as u64) & 0x1) << 5; }

    /// Gets state variable 02h
    pub fn get_MSS_PGV02(&self) -> bool { (self.0 >> 5 & 0x1) != 0 }
        
    /// Sets state variable 01h

    pub fn set_MSS_PGV01(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffef) | ((value as u64) & 0x1) << 4; }

    /// Gets state variable 01h
    pub fn get_MSS_PGV01(&self) -> bool { (self.0 >> 4 & 0x1) != 0 }
        
    /// Sets error vector 3Bh

    pub fn set_MSS_FV3B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffff7) | ((value as u64) & 0x1) << 3; }

    /// Gets error vector 3Bh
    pub fn get_MSS_FV3B(&self) -> bool { (self.0 >> 3 & 0x1) != 0 }
        
    /// Sets Error vector 3Ah

    pub fn set_MSS_FV3A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffb) | ((value as u64) & 0x1) << 2; }

    /// Gets Error vector 3Ah
    pub fn get_MSS_FV3A(&self) -> bool { (self.0 >> 2 & 0x1) != 0 }
        
    /// Sets error vector 39h

    pub fn set_MSS_FV39(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffd) | ((value as u64) & 0x1) << 1; }

    /// Gets error vector 39h
    pub fn get_MSS_FV39(&self) -> bool { (self.0 >> 1 & 0x1) != 0 }
        
    /// Sets error vector 38h

    pub fn set_MSS_FV38(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffe) | ((value as u64) & 0x1); }

    /// Gets error vector 38h
    pub fn get_MSS_FV38(&self) -> bool { (self.0 & 0x1) != 0 }
        
}