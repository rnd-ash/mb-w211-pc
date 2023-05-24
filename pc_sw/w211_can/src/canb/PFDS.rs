
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'PFDS'
*/
    
pub const PFDS_A1_CAN_ID: u16 = 0x02A8;
pub const SD_RS_PFDS_CAN_ID: u16 = 0x07CE;

/// FDS pump motor status
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum PFDS_A1_PFDSMOT_ST {
	AUS = 0, // Pump motor is off
	EIN = 1, // Pump motor is on
	KWL = 2, // Pump motor makes condensate flow
	FEHLER = 3, // Pump motor is inactive due to internal disruption
}

impl TryFrom<u8> for PFDS_A1_PFDSMOT_ST {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::AUS),
			1 => Ok(Self::EIN),
			2 => Ok(Self::KWL),
			3 => Ok(Self::FEHLER),
			_ => Err(())
		}
	}
}

pub struct PFDS_A1(u64);

impl PFDS_A1 {

	/// Gets CAN ID of PFDS_A1
	pub fn get_canid() -> u16 { PFDS_A1_CAN_ID }
    /// Sets FDS pump deactivated

    pub fn set_PFDS_DEAKT(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets FDS pump deactivated
    pub fn get_PFDS_DEAKT(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Bleed the pressure vessel

    pub fn set_PFDS_ENT(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Bleed the pressure vessel
    pub fn get_PFDS_ENT(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Pre-warning overload

    pub fn set_PFDS_OVLD(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Pre-warning overload
    pub fn get_PFDS_OVLD(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets FDS pump motor status

    pub fn set_PFDSMOT_ST(&mut self, value: PFDS_A1_PFDSMOT_ST){ self.0 = (self.0 & 0xfcffffffffffffff) | ((value as u64) & 0x3) << 56; }

    /// Gets FDS pump motor status
    pub fn get_PFDSMOT_ST(&self) -> std::result::Result<PFDS_A1_PFDSMOT_ST, ()> { return PFDS_A1_PFDSMOT_ST::try_from((self.0 >> 56 & 0x3) as u8) }
        
    /// Sets FDS pump supply pressure. Conversion formula (To raw from real): y=(x-1250.0)/50.00 (Unit: hPa)

    pub fn set_P_PFDS(&mut self, value: u8){ self.0 = (self.0 & 0xffe0ffffffffffff) | ((value as u64) & 0x1f) << 48; }

    /// Gets FDS pump supply pressure. Conversion formula (To real from raw): y=(50.00x)+1250.0 (Unit: hPa)
    pub fn get_P_PFDS(&self) -> u8 { (self.0 >> 48 & 0x1f) as u8 }
        
}
pub struct SD_RS_PFDS(u64);

impl SD_RS_PFDS {

	/// Gets CAN ID of SD_RS_PFDS
	pub fn get_canid() -> u16 { SD_RS_PFDS_CAN_ID }
    /// Sets Identification for > 8 bytes

    pub fn set_PFDS_KENN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Identification for > 8 bytes
    pub fn get_PFDS_KENN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets error vector 07h

    pub fn set_PFDS_FV07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets error vector 07h
    pub fn get_PFDS_FV07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets error vector 06h

    pub fn set_PFDS_FV06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets error vector 06h
    pub fn get_PFDS_FV06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets error vector 05h

    pub fn set_PFDS_FV05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets error vector 05h
    pub fn get_PFDS_FV05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets error vector 04h

    pub fn set_PFDS_FV04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets error vector 04h
    pub fn get_PFDS_FV04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets error vector 03h

    pub fn set_PFDS_FV03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets error vector 03h
    pub fn get_PFDS_FV03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets error vector 02h

    pub fn set_PFDS_FV02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets error vector 02h
    pub fn get_PFDS_FV02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets error vector 01h

    pub fn set_PFDS_FV01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets error vector 01h
    pub fn get_PFDS_FV01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets error vector 0Fh

    pub fn set_PFDS_FV0F(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets error vector 0Fh
    pub fn get_PFDS_FV0F(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets error vector 0Eh

    pub fn set_PFDS_FV0E(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets error vector 0Eh
    pub fn get_PFDS_FV0E(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets error vector 0Dh

    pub fn set_PFDS_FV0D(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets error vector 0Dh
    pub fn get_PFDS_FV0D(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets error vector 0Ch

    pub fn set_PFDS_FV0C(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets error vector 0Ch
    pub fn get_PFDS_FV0C(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets error vector 0Bh

    pub fn set_PFDS_FV0B(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets error vector 0Bh
    pub fn get_PFDS_FV0B(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets error vector 0Ah

    pub fn set_PFDS_FV0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets error vector 0Ah
    pub fn get_PFDS_FV0A(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets error vector 09h

    pub fn set_PFDS_FV09(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets error vector 09h
    pub fn get_PFDS_FV09(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets error vector 08h

    pub fn set_PFDS_FV08(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets error vector 08h
    pub fn get_PFDS_FV08(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets state variable 08h

    pub fn set_PFDS_PGV08(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets state variable 08h
    pub fn get_PFDS_PGV08(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets state variable 07h

    pub fn set_PFDS_PGV07(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets state variable 07h
    pub fn get_PFDS_PGV07(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets state variable 06h

    pub fn set_PFDS_PGV06(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets state variable 06h
    pub fn get_PFDS_PGV06(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets state variable 05h

    pub fn set_PFDS_PGV05(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets state variable 05h
    pub fn get_PFDS_PGV05(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets state variable 04h

    pub fn set_PFDS_PGV04(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets state variable 04h
    pub fn get_PFDS_PGV04(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets state variable 03h

    pub fn set_PFDS_PGV03(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets state variable 03h
    pub fn get_PFDS_PGV03(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets state variable 02h

    pub fn set_PFDS_PGV02(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets state variable 02h
    pub fn get_PFDS_PGV02(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets state variable 01h

    pub fn set_PFDS_PGV01(&mut self, value: bool){ self.0 = (self.0 & 0xfffffeffffffffff) | ((value as u64) & 0x1) << 40; }

    /// Gets state variable 01h
    pub fn get_PFDS_PGV01(&self) -> bool { (self.0 >> 40 & 0x1) != 0 }
        
}