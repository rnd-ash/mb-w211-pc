
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'ARMADA'
*/
    
pub const ARMADA_A1_CAN_ID: u16 = 0x0012;
pub const ARMADA_A2_CAN_ID: u16 = 0x0040;
pub const SD_RS_ARMADA_CAN_ID: u16 = 0x07DC;

/// Child seat status
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_KISI_ST {
	KEIN_KISI = 0, // No child seat detected
	KISI_V_ERK = 1, // Child seat detected (installed forward)
	KISI_R_ERK = 2, // Child seat detected (rear mounted)
	KISI_FPOS = 4, // Child seat wrong position
	KISI_FEHLER = 6, // Child seat detection error
	SNV = 7, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_KISI_ST {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::KEIN_KISI),
			1 => Ok(Self::KISI_V_ERK),
			2 => Ok(Self::KISI_R_ERK),
			4 => Ok(Self::KISI_FPOS),
			6 => Ok(Self::KISI_FEHLER),
			7 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Passenger seat belt buckle
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_GS_BF {
	GS_OK = 0, // Seat belt buckle inserted
	GS_NOK = 1, // Seat belt buckle not inserted
	GS_FEHLER = 2, // Seat belt buckle error
	SNV = 3, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_GS_BF {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::GS_OK),
			1 => Ok(Self::GS_NOK),
			2 => Ok(Self::GS_FEHLER),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Passenger detection fast
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_PSG_DETEC_FAST {
	SNA = 0, // Signal not available
	PSG_PRESENT = 1, // Passenger present
	FAULT = 2, // Fault
	PSG_ABSENT = 3, // Passenger absent
}

impl TryFrom<u8> for ARMADA_A1_PSG_DETEC_FAST {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SNA),
			1 => Ok(Self::PSG_PRESENT),
			2 => Ok(Self::FAULT),
			3 => Ok(Self::PSG_ABSENT),
			_ => Err(())
		}
	}
}
/// Occupant classification passenger
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_OC_BF {
	OC_0 = 0, // Occupant Classification 0-10kg
	OC_1 = 1, // Occupant classification 11-30kg
	OC_2 = 2, // Occupant Classification 31-60kg
	OC_3 = 3, // Occupant classification 61-90kg
	OC_4 = 4, // Occupant classification >90kg
	OC_FEHLER = 6, // Error OC
	SNV = 7, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_OC_BF {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::OC_0),
			1 => Ok(Self::OC_1),
			2 => Ok(Self::OC_2),
			3 => Ok(Self::OC_3),
			4 => Ok(Self::OC_4),
			6 => Ok(Self::OC_FEHLER),
			7 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Seat belt buckle driver
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_GS_F {
	GS_OK = 0, // Seat belt buckle inserted
	GS_NOK = 1, // Seat belt buckle not inserted
	GS_FEHLER = 2, // Seat belt buckle error
	SNV = 3, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_GS_F {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::GS_OK),
			1 => Ok(Self::GS_NOK),
			2 => Ok(Self::GS_FEHLER),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Occupant classification driver
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_OC_F {
	OC_0 = 0, // Occupant Classification 0-10kg
	OC_1 = 1, // Occupant classification 11-30kg
	OC_2 = 2, // Occupant Classification 31-60kg
	OC_3 = 3, // Occupant classification 61-90kg
	OC_4 = 4, // Occupant classification >90kg
	OC_FEHLER = 6, // Error OC
	SNV = 7, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_OC_F {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::OC_0),
			1 => Ok(Self::OC_1),
			2 => Ok(Self::OC_2),
			3 => Ok(Self::OC_3),
			4 => Ok(Self::OC_4),
			6 => Ok(Self::OC_FEHLER),
			7 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Belt buckle SHM (reserved)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_GS_SHM {
	GS_OK = 0, // Seat belt buckle inserted
	GS_NOK = 1, // Seat belt buckle not inserted
	GS_FEHLER = 2, // Seat belt buckle error
	SNV = 3, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_GS_SHM {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::GS_OK),
			1 => Ok(Self::GS_NOK),
			2 => Ok(Self::GS_FEHLER),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Passenger Isofix switch (only Roadster)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_ISOFIX_BF {
	KEIN_KISI = 0, // No KISI
	KISI_ERK = 1, // KISI detected
	KISI_FEHLER = 2, // KISI error
	SNV = 3, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_ISOFIX_BF {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::KEIN_KISI),
			1 => Ok(Self::KISI_ERK),
			2 => Ok(Self::KISI_FEHLER),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Seat belt buckle SHR (reserved)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_GS_SHR {
	GS_OK = 0, // Seat belt buckle inserted
	GS_NOK = 1, // Seat belt buckle not inserted
	GS_FEHLER = 2, // Seat belt buckle error
	SNV = 3, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_GS_SHR {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::GS_OK),
			1 => Ok(Self::GS_NOK),
			2 => Ok(Self::GS_FEHLER),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Seat belt buckle SHL (reserved)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum ARMADA_A1_GS_SHL {
	GS_OK = 0, // Seat belt buckle inserted
	GS_NOK = 1, // Seat belt buckle not inserted
	GS_FEHLER = 2, // Seat belt buckle error
	SNV = 3, // Signal not available
}

impl TryFrom<u8> for ARMADA_A1_GS_SHL {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::GS_OK),
			1 => Ok(Self::GS_NOK),
			2 => Ok(Self::GS_FEHLER),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}

pub struct ARMADA_A1(u64);

impl ARMADA_A1 {

	/// Gets CAN ID of ARMADA_A1
	pub fn get_canid() -> u16 { ARMADA_A1_CAN_ID }
    /// Sets Switch on AKSE lamp

    pub fn set_AKSE_EIN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Switch on AKSE lamp
    pub fn get_AKSE_EIN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets AKSE light flashing

    pub fn set_AKSE_BLINK(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets AKSE light flashing
    pub fn get_AKSE_BLINK(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Check passenger seat contact

    pub fn set_KONTAKT_BF(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Check passenger seat contact
    pub fn get_KONTAKT_BF(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Turn on SRS indicator lamp, on [1], off [0]

    pub fn set_SRS_KL(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Turn on SRS indicator lamp, on [1], off [0]
    pub fn get_SRS_KL(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets SRS indicator lamp flashing on [1], off [0]

    pub fn set_SRS_BLINK(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets SRS indicator lamp flashing on [1], off [0]
    pub fn get_SRS_BLINK(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets SRS message 3 "workshop", on [1], off [0]

    pub fn set_SRS_WERK_FT3(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets SRS message 3 "workshop", on [1], off [0]
    pub fn get_SRS_WERK_FT3(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets SRS message "workshop", on [1], off [0]

    pub fn set_SRS_WERK_FT1(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets SRS message "workshop", on [1], off [0]
    pub fn get_SRS_WERK_FT1(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Check seat latches front passenger

    pub fn set_RAST_SITZ_BF(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets Check seat latches front passenger
    pub fn get_RAST_SITZ_BF(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets Child seat status

    pub fn set_KISI_ST(&mut self, value: ARMADA_A1_KISI_ST){ self.0 = (self.0 & 0xfff8ffffffffffff) | ((value as u64) & 0x7) << 48; }

    /// Gets Child seat status
    pub fn get_KISI_ST(&self) -> std::result::Result<ARMADA_A1_KISI_ST, ()> { return ARMADA_A1_KISI_ST::try_from((self.0 >> 48 & 0x7) as u8) }
        
    /// Sets Passenger seat belt buckle

    pub fn set_GS_BF(&mut self, value: ARMADA_A1_GS_BF){ self.0 = (self.0 & 0xffff3fffffffffff) | ((value as u64) & 0x3) << 46; }

    /// Gets Passenger seat belt buckle
    pub fn get_GS_BF(&self) -> std::result::Result<ARMADA_A1_GS_BF, ()> { return ARMADA_A1_GS_BF::try_from((self.0 >> 46 & 0x3) as u8) }
        
    /// Sets Passenger detection fast

    pub fn set_PSG_DETEC_FAST(&mut self, value: ARMADA_A1_PSG_DETEC_FAST){ self.0 = (self.0 & 0xffffe7ffffffffff) | ((value as u64) & 0x3) << 43; }

    /// Gets Passenger detection fast
    pub fn get_PSG_DETEC_FAST(&self) -> std::result::Result<ARMADA_A1_PSG_DETEC_FAST, ()> { return ARMADA_A1_PSG_DETEC_FAST::try_from((self.0 >> 43 & 0x3) as u8) }
        
    /// Sets Occupant classification passenger

    pub fn set_OC_BF(&mut self, value: ARMADA_A1_OC_BF){ self.0 = (self.0 & 0xfffff8ffffffffff) | ((value as u64) & 0x7) << 40; }

    /// Gets Occupant classification passenger
    pub fn get_OC_BF(&self) -> std::result::Result<ARMADA_A1_OC_BF, ()> { return ARMADA_A1_OC_BF::try_from((self.0 >> 40 & 0x7) as u8) }
        
    /// Sets Seat belt buckle driver

    pub fn set_GS_F(&mut self, value: ARMADA_A1_GS_F){ self.0 = (self.0 & 0xffffff3fffffffff) | ((value as u64) & 0x3) << 38; }

    /// Gets Seat belt buckle driver
    pub fn get_GS_F(&self) -> std::result::Result<ARMADA_A1_GS_F, ()> { return ARMADA_A1_GS_F::try_from((self.0 >> 38 & 0x3) as u8) }
        
    /// Sets Occupant classification driver

    pub fn set_OC_F(&mut self, value: ARMADA_A1_OC_F){ self.0 = (self.0 & 0xfffffff8ffffffff) | ((value as u64) & 0x7) << 32; }

    /// Gets Occupant classification driver
    pub fn get_OC_F(&self) -> std::result::Result<ARMADA_A1_OC_F, ()> { return ARMADA_A1_OC_F::try_from((self.0 >> 32 & 0x7) as u8) }
        
    /// Sets Belt buckle SHM (reserved)

    pub fn set_GS_SHM(&mut self, value: ARMADA_A1_GS_SHM){ self.0 = (self.0 & 0xffffffff3fffffff) | ((value as u64) & 0x3) << 30; }

    /// Gets Belt buckle SHM (reserved)
    pub fn get_GS_SHM(&self) -> std::result::Result<ARMADA_A1_GS_SHM, ()> { return ARMADA_A1_GS_SHM::try_from((self.0 >> 30 & 0x3) as u8) }
        
    /// Sets Passenger Isofix switch (only Roadster)

    pub fn set_ISOFIX_BF(&mut self, value: ARMADA_A1_ISOFIX_BF){ self.0 = (self.0 & 0xffffffffcfffffff) | ((value as u64) & 0x3) << 28; }

    /// Gets Passenger Isofix switch (only Roadster)
    pub fn get_ISOFIX_BF(&self) -> std::result::Result<ARMADA_A1_ISOFIX_BF, ()> { return ARMADA_A1_ISOFIX_BF::try_from((self.0 >> 28 & 0x3) as u8) }
        
    /// Sets Seat belt buckle SHR (reserved)

    pub fn set_GS_SHR(&mut self, value: ARMADA_A1_GS_SHR){ self.0 = (self.0 & 0xfffffffff3ffffff) | ((value as u64) & 0x3) << 26; }

    /// Gets Seat belt buckle SHR (reserved)
    pub fn get_GS_SHR(&self) -> std::result::Result<ARMADA_A1_GS_SHR, ()> { return ARMADA_A1_GS_SHR::try_from((self.0 >> 26 & 0x3) as u8) }
        
    /// Sets Seat belt buckle SHL (reserved)

    pub fn set_GS_SHL(&mut self, value: ARMADA_A1_GS_SHL){ self.0 = (self.0 & 0xfffffffffcffffff) | ((value as u64) & 0x3) << 24; }

    /// Gets Seat belt buckle SHL (reserved)
    pub fn get_GS_SHL(&self) -> std::result::Result<ARMADA_A1_GS_SHL, ()> { return ARMADA_A1_GS_SHL::try_from((self.0 >> 24 & 0x3) as u8) }
        
    /// Sets Check rear seat catch

    pub fn set_RAST_SHR(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffff7ffff) | ((value as u64) & 0x1) << 19; }

    /// Gets Check rear seat catch
    pub fn get_RAST_SHR(&self) -> bool { (self.0 >> 19 & 0x1) != 0 }
        
    /// Sets Check seat catch HL

    pub fn set_RAST_SHL(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffbffff) | ((value as u64) & 0x1) << 18; }

    /// Gets Check seat catch HL
    pub fn get_RAST_SHL(&self) -> bool { (self.0 >> 18 & 0x1) != 0 }
        
    /// Sets Check seat HI RE contact

    pub fn set_KONT_HI_RE(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets Check seat HI RE contact
    pub fn get_KONT_HI_RE(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets Check seat HI LI contact

    pub fn set_KONT_HI_LI(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets Check seat HI LI contact
    pub fn get_KONT_HI_LI(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
}
pub struct ARMADA_A2(u64);

impl ARMADA_A2 {

	/// Gets CAN ID of ARMADA_A2
	pub fn get_canid() -> u16 { ARMADA_A2_CAN_ID }
    /// Sets Confirm bit for all crash events, toggles

    pub fn set_CONF_CRASH(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Confirm bit for all crash events, toggles
    pub fn get_CONF_CRASH(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets rollover event 1

    pub fn set_CRASH_G(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets rollover event 1
    pub fn get_CRASH_G(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Frontal event 2

    pub fn set_CRASH_F(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Frontal event 2
    pub fn get_CRASH_F(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets tail event 2

    pub fn set_CRASH_E(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets tail event 2
    pub fn get_CRASH_E(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets side event 1

    pub fn set_CRASH_D(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets side event 1
    pub fn get_CRASH_D(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Frontal event 5

    pub fn set_CRASH_C(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Frontal event 5
    pub fn get_CRASH_C(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets tail event 1

    pub fn set_CRASH_B(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets tail event 1
    pub fn get_CRASH_B(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Frontal event 1

    pub fn set_CRASH_A(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Frontal event 1
    pub fn get_CRASH_A(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Any crash event present

    pub fn set_X_CRASH(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets Any crash event present
    pub fn get_X_CRASH(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets event tbd

    pub fn set_CRASH_O(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets event tbd
    pub fn get_CRASH_O(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets event tbd

    pub fn set_CRASH_N(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets event tbd
    pub fn get_CRASH_N(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets event tbd

    pub fn set_CRASH_M(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets event tbd
    pub fn get_CRASH_M(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets side event 2

    pub fn set_CRASH_L(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets side event 2
    pub fn get_CRASH_L(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets tail event 3

    pub fn set_CRASH_K(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets tail event 3
    pub fn get_CRASH_K(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets rollover event 3

    pub fn set_CRASH_I(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets rollover event 3
    pub fn get_CRASH_I(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets rollover event 2

    pub fn set_CRASH_H(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets rollover event 2
    pub fn get_CRASH_H(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
}
pub struct SD_RS_ARMADA(u64);

impl SD_RS_ARMADA {

	/// Gets CAN ID of SD_RS_ARMADA
	pub fn get_canid() -> u16 { SD_RS_ARMADA_CAN_ID }
    /// Sets Identification for > 8 bytes

    pub fn set_ARMADA_KENN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Identification for > 8 bytes
    pub fn get_ARMADA_KENN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets state variable 07h

    pub fn set_ARMADA_PGV07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets state variable 07h
    pub fn get_ARMADA_PGV07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets state variable 06h

    pub fn set_ARMADA_PGV06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets state variable 06h
    pub fn get_ARMADA_PGV06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets state variable 05h

    pub fn set_ARMADA_PGV05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets state variable 05h
    pub fn get_ARMADA_PGV05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets state variable 04h

    pub fn set_ARMADA_PGV04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets state variable 04h
    pub fn get_ARMADA_PGV04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets state variable 03h

    pub fn set_ARMADA_PGV03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets state variable 03h
    pub fn get_ARMADA_PGV03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets state variable 02h

    pub fn set_ARMADA_PGV02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets state variable 02h
    pub fn get_ARMADA_PGV02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets state variable 01h

    pub fn set_ARMADA_PGV01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets state variable 01h
    pub fn get_ARMADA_PGV01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Error message 01h. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_ARMADA_FM01(&mut self, value: u16){ self.0 = (self.0 & 0xff0000ffffffffff) | ((value as u64) & 0xffff) << 40; }

    /// Gets Error message 01h. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_ARMADA_FM01(&self) -> u16 { (self.0 >> 40 & 0xffff) as u16 }
        
    /// Sets Error message 02h. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_ARMADA_FM02(&mut self, value: u16){ self.0 = (self.0 & 0xffffff0000ffffff) | ((value as u64) & 0xffff) << 24; }

    /// Gets Error message 02h. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_ARMADA_FM02(&self) -> u16 { (self.0 >> 24 & 0xffff) as u16 }
        
    /// Sets Error message 03h. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_ARMADA_FM03(&mut self, value: u16){ self.0 = (self.0 & 0xffffffffff0000ff) | ((value as u64) & 0xffff) << 8; }

    /// Gets Error message 03h. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_ARMADA_FM03(&self) -> u16 { (self.0 >> 8 & 0xffff) as u16 }
        
    /// Sets state variable 0Fh

    pub fn set_ARMADA_PGV0F(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffff7f) | ((value as u64) & 0x1) << 7; }

    /// Gets state variable 0Fh
    pub fn get_ARMADA_PGV0F(&self) -> bool { (self.0 >> 7 & 0x1) != 0 }
        
    /// Sets state variable 0Eh

    pub fn set_ARMADA_PGV0E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffbf) | ((value as u64) & 0x1) << 6; }

    /// Gets state variable 0Eh
    pub fn get_ARMADA_PGV0E(&self) -> bool { (self.0 >> 6 & 0x1) != 0 }
        
    /// Sets State variable 0Dh

    pub fn set_ARMADA_PGV0D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffdf) | ((value as u64) & 0x1) << 5; }

    /// Gets State variable 0Dh
    pub fn get_ARMADA_PGV0D(&self) -> bool { (self.0 >> 5 & 0x1) != 0 }
        
    /// Sets state variable 0Ch

    pub fn set_ARMADA_PGV0C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffef) | ((value as u64) & 0x1) << 4; }

    /// Gets state variable 0Ch
    pub fn get_ARMADA_PGV0C(&self) -> bool { (self.0 >> 4 & 0x1) != 0 }
        
    /// Sets state variable 0Bh

    pub fn set_ARMADA_PGV0B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffff7) | ((value as u64) & 0x1) << 3; }

    /// Gets state variable 0Bh
    pub fn get_ARMADA_PGV0B(&self) -> bool { (self.0 >> 3 & 0x1) != 0 }
        
    /// Sets State variable 0Ah

    pub fn set_ARMADA_PGV0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffb) | ((value as u64) & 0x1) << 2; }

    /// Gets State variable 0Ah
    pub fn get_ARMADA_PGV0A(&self) -> bool { (self.0 >> 2 & 0x1) != 0 }
        
    /// Sets state variable 09h

    pub fn set_ARMADA_PGV09(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffd) | ((value as u64) & 0x1) << 1; }

    /// Gets state variable 09h
    pub fn get_ARMADA_PGV09(&self) -> bool { (self.0 >> 1 & 0x1) != 0 }
        
    /// Sets state variable 08h

    pub fn set_ARMADA_PGV08(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffe) | ((value as u64) & 0x1) << 0; }

    /// Gets state variable 08h
    pub fn get_ARMADA_PGV08(&self) -> bool { (self.0 >> 0 & 0x1) != 0 }
        
}