
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'HFS'
*/
    
pub const HFS_A1_CAN_ID: u16 = 0x0078;
pub const SD_RS_HFS_CAN_ID: u16 = 0x07D7;

/// Trunk lid status
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum HFS_A1_HD_ST {
	N_DEF = 0, // undefined
	HD_RI_ZU = 1, // Trunk lid closes
	HD_RI_AUF = 2, // Trunk lid opens
	HD_ST_AUF = 3, // Trunk lid is fully open
	HD_ST_ZW = 4, // Trunk lid in intermediate position
	SNV = 7, // Signal not available
}

impl TryFrom<u8> for HFS_A1_HD_ST {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::N_DEF),
			1 => Ok(Self::HD_RI_ZU),
			2 => Ok(Self::HD_RI_AUF),
			3 => Ok(Self::HD_ST_AUF),
			4 => Ok(Self::HD_ST_ZW),
			7 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HFS_A1(pub u64);

impl HFS_A1 {

	/// Gets CAN ID of HFS_A1
	pub const fn get_canid() -> u16 { HFS_A1_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Trunk lid status

    pub fn set_HD_ST(&mut self, value: HFS_A1_HD_ST){ self.0 = (self.0 & 0x1fffffffffffffff) | ((value as u64) & 0x7) << 61; }

    /// Gets Trunk lid status
    pub fn get_HD_ST(&self) -> std::result::Result<HFS_A1_HD_ST, ()> { return HFS_A1_HD_ST::try_from((self.0 >> 61 & 0x7) as u8) }
        
    /// Sets Close and secure boot lid actuated

    pub fn set_HDI_SKG_HFS(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Close and secure boot lid actuated
    pub fn get_HDI_SKG_HFS(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Close boot lid actuated

    pub fn set_HDI_S_HFS(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Close boot lid actuated
    pub fn get_HDI_S_HFS(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Load floor warning

    pub fn set_ALB_WARN(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Load floor warning
    pub fn get_ALB_WARN(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SD_RS_HFS(pub u64);

impl SD_RS_HFS {

	/// Gets CAN ID of SD_RS_HFS
	pub const fn get_canid() -> u16 { SD_RS_HFS_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Identification for > 8 bytes

    pub fn set_HFS_KENN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Identification for > 8 bytes
    pub fn get_HFS_KENN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets error vector 07h

    pub fn set_HFS_FV07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets error vector 07h
    pub fn get_HFS_FV07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets error vector 06h

    pub fn set_HFS_FV06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets error vector 06h
    pub fn get_HFS_FV06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets error vector 05h

    pub fn set_HFS_FV05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets error vector 05h
    pub fn get_HFS_FV05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets error vector 04h

    pub fn set_HFS_FV04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets error vector 04h
    pub fn get_HFS_FV04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets error vector 03h

    pub fn set_HFS_FV03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets error vector 03h
    pub fn get_HFS_FV03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets error vector 02h

    pub fn set_HFS_FV02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets error vector 02h
    pub fn get_HFS_FV02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets error vector 01h

    pub fn set_HFS_FV01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets error vector 01h
    pub fn get_HFS_FV01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets error vector 0Fh

    pub fn set_HFS_FV0F(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets error vector 0Fh
    pub fn get_HFS_FV0F(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets error vector 0Eh

    pub fn set_HFS_FV0E(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets error vector 0Eh
    pub fn get_HFS_FV0E(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets error vector 0Dh

    pub fn set_HFS_FV0D(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets error vector 0Dh
    pub fn get_HFS_FV0D(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets error vector 0Ch

    pub fn set_HFS_FV0C(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets error vector 0Ch
    pub fn get_HFS_FV0C(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets error vector 0Bh

    pub fn set_HFS_FV0B(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets error vector 0Bh
    pub fn get_HFS_FV0B(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets error vector 0Ah

    pub fn set_HFS_FV0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets error vector 0Ah
    pub fn get_HFS_FV0A(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets error vector 09h

    pub fn set_HFS_FV09(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets error vector 09h
    pub fn get_HFS_FV09(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets error vector 08h

    pub fn set_HFS_FV08(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets error vector 08h
    pub fn get_HFS_FV08(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets state variable 08h

    pub fn set_HFS_PGV08(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets state variable 08h
    pub fn get_HFS_PGV08(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets state variable 07h

    pub fn set_HFS_PGV07(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets state variable 07h
    pub fn get_HFS_PGV07(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets state variable 06h

    pub fn set_HFS_PGV06(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets state variable 06h
    pub fn get_HFS_PGV06(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets state variable 05h

    pub fn set_HFS_PGV05(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets state variable 05h
    pub fn get_HFS_PGV05(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets state variable 04h

    pub fn set_HFS_PGV04(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets state variable 04h
    pub fn get_HFS_PGV04(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets state variable 03h

    pub fn set_HFS_PGV03(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets state variable 03h
    pub fn get_HFS_PGV03(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets state variable 02h

    pub fn set_HFS_PGV02(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets state variable 02h
    pub fn get_HFS_PGV02(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets state variable 01h

    pub fn set_HFS_PGV01(&mut self, value: bool){ self.0 = (self.0 & 0xfffffeffffffffff) | ((value as u64) & 0x1) << 40; }

    /// Gets state variable 01h
    pub fn get_HFS_PGV01(&self) -> bool { (self.0 >> 40 & 0x1) != 0 }
        
}