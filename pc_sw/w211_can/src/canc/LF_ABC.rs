
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'LF_ABC'
*/
    
pub const FS_340_CAN_ID: u16 = 0x0340;

/// loading
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum FS_340h_BELAD {
	LEER = 0, // Unloaded
	HALB = 1, // Half loaded
	VOLL = 2, // Fully loaded
	SNV = 3, // Load not recognized
}

impl TryFrom<u8> for FS_340h_BELAD {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::LEER),
			1 => Ok(Self::HALB),
			2 => Ok(Self::VOLL),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Suspension control identification
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum FS_340h_FS_ID {
	LF = 0, // Air suspension/ LF (BR164/251 NR without ADS)
	SLF = 1, // Semi-active air suspension, SLF (BR164/251 NR+ADS)
	EHNR = 2, // Electronic rear axle level control
	ABC = 3, // Active Body Control 1
}

impl TryFrom<u8> for FS_340h_FS_ID {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::LF),
			1 => Ok(Self::SLF),
			2 => Ok(Self::EHNR),
			3 => Ok(Self::ABC),
			_ => Err(())
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FS_340(pub u64);

impl FS_340 {

	/// Gets CAN ID of FS_340
	pub const fn get_canid() -> u16 { FS_340_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Level calibration performed

    pub fn set_NEDG(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Level calibration performed
    pub fn get_NEDG(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Message 2: "Level selection deleted"

    pub fn set_M2(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Message 2: "Level selection deleted"
    pub fn get_M2(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Message 1: "Vehicle lifts", BR164/251: "Highway->Offroad", BR164 Offroad: "Highway->Offroad1"

    pub fn set_M1(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Message 1: "Vehicle lifts", BR164/251: "Highway->Offroad", BR164 Offroad: "Highway->Offroad1"
    pub fn get_M1(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Error 4: "Park vehicle"

    pub fn set_FM4(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Error 4: "Park vehicle"
    pub fn get_FM4(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Error 3: "Visit workshop"

    pub fn set_FM3(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Error 3: "Visit workshop"
    pub fn get_FM3(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Error 2: "wait a moment" (LF)/ "steering oil" (only ABC)

    pub fn set_FM2(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Error 2: "wait a moment" (LF)/ "steering oil" (only ABC)
    pub fn get_FM2(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Error 1: "Stop car too low"

    pub fn set_FM1(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Error 1: "Stop car too low"
    pub fn get_FM1(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets loading

    pub fn set_BELAD(&mut self, value: FS_340h_BELAD){ self.0 = (self.0 & 0xff9fffffffffffff) | ((value as u64) & 0x3) << 53; }

    /// Gets loading
    pub fn get_BELAD(&self) -> Option<FS_340h_BELAD> {  FS_340h_BELAD::try_from((self.0 >> 53 & 0x3) as u8).ok() }
        
    /// Sets LED 2-stage switch steady light

    pub fn set_ST2_LED_DL(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets LED 2-stage switch steady light
    pub fn get_ST2_LED_DL(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets Right LED 3-position switch steady light (164/251 top LED)

    pub fn set_ST3_LEDR_DL(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets Right LED 3-position switch steady light (164/251 top LED)
    pub fn get_ST3_LEDR_DL(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets Left LED 3-position switch steady light (164/251 lower LED)

    pub fn set_ST3_LEDL_DL(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets Left LED 3-position switch steady light (164/251 lower LED)
    pub fn get_ST3_LEDL_DL(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets Vehicle level, front left. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_FZGN_VL(&mut self, value: u8){ self.0 = (self.0 & 0xffff00ffffffffff) | ((value as u64) & 0xff) << 40; }

    /// Gets Vehicle level, front left. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_FZGN_VL(&self) -> u8 { (self.0 >> 40 & 0xff) as u8 }
        
    /// Sets Vehicle level, front right. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_FZGN_VR(&mut self, value: u8){ self.0 = (self.0 & 0xffffff00ffffffff) | ((value as u64) & 0xff) << 32; }

    /// Gets Vehicle level, front right. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_FZGN_VR(&self) -> u8 { (self.0 >> 32 & 0xff) as u8 }
        
    /// Sets Rear left vehicle level. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_FZGN_HL(&mut self, value: u8){ self.0 = (self.0 & 0xffffffff00ffffff) | ((value as u64) & 0xff) << 24; }

    /// Gets Rear left vehicle level. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_FZGN_HL(&self) -> u8 { (self.0 >> 24 & 0xff) as u8 }
        
    /// Sets Vehicle level, rear right. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_FZGN_HR(&mut self, value: u8){ self.0 = (self.0 & 0xffffffffff00ffff) | ((value as u64) & 0xff) << 16; }

    /// Gets Vehicle level, rear right. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_FZGN_HR(&self) -> u8 { (self.0 >> 16 & 0xff) as u8 }
        
    /// Sets Suspension control identification

    pub fn set_FS_ID(&mut self, value: FS_340h_FS_ID){ self.0 = (self.0 & 0xffffffffffffff1f) | ((value as u64) & 0x7) << 5; }

    /// Gets Suspension control identification
    pub fn get_FS_ID(&self) -> Option<FS_340h_FS_ID> {  FS_340h_FS_ID::try_from((self.0 >> 5 & 0x7) as u8).ok() }
        
}