
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'RGS_L'
*/
    
pub const RGS_L_450_CAN_ID: u16 = 0x0450;

/// Presafe adjustment combined message
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum RGS_L_450h_PRESF_L_KI {
	PRESF_WM_AUS = 0, // Presafe workshop message off
	PRESF_WM_EIN = 1, // Presafe workshop message on
	NDEF2 = 2, // undefined
	NDEF3 = 3, // undefined
}

impl TryFrom<u8> for RGS_L_450h_PRESF_L_KI {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::PRESF_WM_AUS),
			1 => Ok(Self::PRESF_WM_EIN),
			2 => Ok(Self::NDEF2),
			3 => Ok(Self::NDEF3),
			_ => Err(())
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGS_L_450(pub u64);

impl RGS_L_450 {

	/// Gets CAN ID of RGS_L_450
	pub const fn get_canid() -> u16 { RGS_L_450_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Presafe toggle bit 40ms (1/message)

    pub fn set_PRESF_L_TGL(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Presafe toggle bit 40ms (1/message)
    pub fn get_PRESF_L_TGL(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Rev. belt tensioner left Tightening cycle active

    pub fn set_PRESF_L_RGS_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Rev. belt tensioner left Tightening cycle active
    pub fn get_PRESF_L_RGS_AKT(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Presafe adjustment combined message

    pub fn set_PRESF_L_KI(&mut self, value: RGS_L_450h_PRESF_L_KI){ self.0 = (self.0 & 0xfffcffffffffffff) | ((value as u64) & 0x3) << 48; }

    /// Gets Presafe adjustment combined message
    pub fn get_PRESF_L_KI(&self) -> Option<RGS_L_450h_PRESF_L_KI> {  RGS_L_450h_PRESF_L_KI::try_from((self.0 >> 48 & 0x3) as u8).ok() }
        
}