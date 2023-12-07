
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'EZS'
*/
    
pub const EZS_240_CAN_ID: u16 = 0x0240;

/// Left Hand Drive/Right Hand Drive
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_240h_LL_RLC {
	NICHT_DEFINIERT = 0, // Unknown
	LL = 1, // Left hand drive
	RL = 2, // Right hand drive
	SNV = 3, // Code not available
}

impl TryFrom<u8> for EZS_240h_LL_RLC {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::NICHT_DEFINIERT),
			1 => Ok(Self::LL),
			2 => Ok(Self::RL),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// ESP on/off actuated
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_240h_ESP_BET {
	NBET = 0, // Not operated (rocker and push push)
	AUS_BET = 1, // ESP off actuated (rocker), actuated (push push)
	EIN_NDEF = 2, // ESP on actuated (rocker), not defined (push push)
	SNV = 3, // No signal (rocker and push push)
}

impl TryFrom<u8> for EZS_240h_ESP_BET {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::NBET),
			1 => Ok(Self::AUS_BET),
			2 => Ok(Self::EIN_NDEF),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// LF/ABC 2-position switch actuated
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_240h_ST2_BET {
	NBET = 0, // Not operated (rocker and push push)
	UNBET_NDEF = 1, // Bottom Actuated (Rocker), Undefined (Push Push)
	OBBET_BET = 2, // Top Actuated (Rocker), Actuated (Push Push)
	NICHT_DEFINIERT = 3, // Unknown
}

impl TryFrom<u8> for EZS_240h_ST2_BET {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::NBET),
			1 => Ok(Self::UNBET_NDEF),
			2 => Ok(Self::OBBET_BET),
			3 => Ok(Self::NICHT_DEFINIERT),
			_ => Err(())
		}
	}
}
/// LF/ABC 3-position switch operated
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_240h_ST3_BET {
	NBET = 0, // Not operated (rocker and push push)
	UNBET_NDEF = 1, // Bottom Actuated (Rocker), Undefined (Push Push)
	OBBET_BET = 2, // Top Actuated (Rocker), Actuated (Push Push)
	NICHT_DEFINIERT = 3, // Unknown
}

impl TryFrom<u8> for EZS_240h_ST3_BET {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::NBET),
			1 => Ok(Self::UNBET_NDEF),
			2 => Ok(Self::OBBET_BET),
			3 => Ok(Self::NICHT_DEFINIERT),
			_ => Err(())
		}
	}
}
/// ART distance warning on/off actuated
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_240h_ART_ABW_BET {
	NDEF_NBET = 0, // not defined (rocker), not actuated (push push)
	AUS_NDEF = 1, // distance warning off (rocker), not defined (push push)
	EIN_BET = 2, // Distance warning on (rocker), actuated (push push)
	SNV = 3, // No signal (rocker and push push)
}

impl TryFrom<u8> for EZS_240h_ART_ABW_BET {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::NDEF_NBET),
			1 => Ok(Self::AUS_NDEF),
			2 => Ok(Self::EIN_BET),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// Series-dependent vehicle version (only 220/215/230)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_240h_FZGVERSN {
	START = 0, // Status at market launch of the respective series
	V1 = 1, // BR 220: AJ 99/X, C215: AJ 01/1, R230: AJ 02/1
	V2 = 2, // BR 220: AJ 01/1, C215: AJ 02/X, R230: AJ 03/X
	V3 = 3, // BR 220: ÄJ 02/X, C215: ÄJ 03/X, R230: not defined
	V4 = 4, // BR 220: prohibited, C215/R230: not defined
	V5 = 5, // BR 220: prohibited, C215/R230: not defined
	V6 = 6, // BR 220: ÄJ 03/X, C215,/R230: not defined
	V7 = 7, // BR 220/ C215,/R230: not defined
}

impl TryFrom<u8> for EZS_240h_FZGVERSN {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::START),
			1 => Ok(Self::V1),
			2 => Ok(Self::V2),
			3 => Ok(Self::V3),
			4 => Ok(Self::V4),
			5 => Ok(Self::V5),
			6 => Ok(Self::V6),
			7 => Ok(Self::V7),
			_ => Err(())
		}
	}
}
/// country code
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_240h_LDC {
	RDW = 0, // Rest of the world
	USA_CAN = 1, // USA/Canada
	NICHT_DEFINIERT = 2, // Unknown
	SNV = 3, // Code not available
}

impl TryFrom<u8> for EZS_240h_LDC {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RDW),
			1 => Ok(Self::USA_CAN),
			2 => Ok(Self::NICHT_DEFINIERT),
			3 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EZS_240(pub u64);

impl EZS_240 {

	/// Gets CAN ID of EZS_240
	pub const fn get_canid() -> u16 { EZS_240_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Keyless Go terminal control active

    pub fn set_KG_KL_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets Keyless Go terminal control active
    pub fn get_KG_KL_AKT(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets Keyles Go occasion requirements met

    pub fn set_KG_ALB_OK(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets Keyles Go occasion requirements met
    pub fn get_KG_ALB_OK(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets Left Hand Drive/Right Hand Drive

    pub fn set_LL_RLC(&mut self, value: EZS_240h_LL_RLC){ self.0 = (self.0 & 0xffcfffffffffffff) | ((value as u64) & 0x3) << 52; }

    /// Gets Left Hand Drive/Right Hand Drive
    pub fn get_LL_RLC(&self) -> Option<EZS_240h_LL_RLC> {  EZS_240h_LL_RLC::try_from((self.0 >> 52 & 0x3) as u8).ok() }
        
    /// Sets reverse gear engaged (manual gearbox only)

    pub fn set_RG_SCHALT(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets reverse gear engaged (manual gearbox only)
    pub fn get_RG_SCHALT(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets Brake switch for shift lock

    pub fn set_BS_SL(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets Brake switch for shift lock
    pub fn get_BS_SL(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets Terminal 15

    pub fn set_KL_15(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets Terminal 15
    pub fn get_KL_15(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets Terminal 50

    pub fn set_KL_50(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets Terminal 50
    pub fn get_KL_50(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets SAM/x passive, x = Bb (230), V (211), F (240)

    pub fn set_SAM_PAS(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets SAM/x passive, x = Bb (230), V (211), F (240)
    pub fn get_SAM_PAS(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets SAM/x: brake light switch output EHB-ASG, x = B (230), V (211), F (240)

    pub fn set_BLS_A(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets SAM/x: brake light switch output EHB-ASG, x = B (230), V (211), F (240)
    pub fn get_BLS_A(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets Vehicle electrical system warning: starter battery state of charge

    pub fn set_BN_SOCS(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets Vehicle electrical system warning: starter battery state of charge
    pub fn get_BN_SOCS(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets ASG sport mode on/off actuated (ST2_LED_DL if ABC available)

    pub fn set_ASG_SPORT_BET(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets ASG sport mode on/off actuated (ST2_LED_DL if ABC available)
    pub fn get_ASG_SPORT_BET(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
    /// Sets SAM/x: v-signal from EHB-ASG, x = B (230), V (211), F ( 240)

    pub fn set_VSTAT_A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffff7ffffffff) | ((value as u64) & 0x1) << 35; }

    /// Gets SAM/x: v-signal from EHB-ASG, x = B (230), V (211), F ( 240)
    pub fn get_VSTAT_A(&self) -> bool { (self.0 >> 35 & 0x1) != 0 }
        
    /// Sets SAM/x: EHB-ASG in fallback level, x = B (230), V (211,164,251), F (240)

    pub fn set_INF_RFE_SAM(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffbffffffff) | ((value as u64) & 0x1) << 34; }

    /// Gets SAM/x: EHB-ASG in fallback level, x = B (230), V (211,164,251), F (240)
    pub fn get_INF_RFE_SAM(&self) -> bool { (self.0 >> 34 & 0x1) != 0 }
        
    /// Sets CRASH confirm bit

    pub fn set_CRASH_CNF(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffdffffffff) | ((value as u64) & 0x1) << 33; }

    /// Gets CRASH confirm bit
    pub fn get_CRASH_CNF(&self) -> bool { (self.0 >> 33 & 0x1) != 0 }
        
    /// Sets Crash signal from airbag SG

    pub fn set_CRASH(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffeffffffff) | ((value as u64) & 0x1) << 32; }

    /// Gets Crash signal from airbag SG
    pub fn get_CRASH(&self) -> bool { (self.0 >> 32 & 0x1) != 0 }
        
    /// Sets Vehicle power supply emergency mode: Prio1 and Prio2 consumers off, second battery supports

    pub fn set_BN_NTLF(&mut self, value: bool){ self.0 = (self.0 & 0xffffffff7fffffff) | ((value as u64) & 0x1) << 31; }

    /// Gets Vehicle power supply emergency mode: Prio1 and Prio2 consumers off, second battery supports
    pub fn get_BN_NTLF(&self) -> bool { (self.0 >> 31 & 0x1) != 0 }
        
    /// Sets ESP on/off actuated

    pub fn set_ESP_BET(&mut self, value: EZS_240h_ESP_BET){ self.0 = (self.0 & 0xffffffff9fffffff) | ((value as u64) & 0x3) << 29; }

    /// Gets ESP on/off actuated
    pub fn get_ESP_BET(&self) -> Option<EZS_240h_ESP_BET> {  EZS_240h_ESP_BET::try_from((self.0 >> 29 & 0x3) as u8).ok() }
        
    /// Sets Handbrake applied (indicator lamp)

    pub fn set_HAS_KL(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffefffffff) | ((value as u64) & 0x1) << 28; }

    /// Gets Handbrake applied (indicator lamp)
    pub fn get_HAS_KL(&self) -> bool { (self.0 >> 28 & 0x1) != 0 }
        
    /// Sets Wiper out of park position

    pub fn set_KL_31B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffff7ffffff) | ((value as u64) & 0x1) << 27; }

    /// Gets Wiper out of park position
    pub fn get_KL_31B(&self) -> bool { (self.0 >> 27 & 0x1) != 0 }
        
    /// Sets Turn signal right

    pub fn set_BLI_RE(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffdffffff) | ((value as u64) & 0x1) << 25; }

    /// Gets Turn signal right
    pub fn get_BLI_RE(&self) -> bool { (self.0 >> 25 & 0x1) != 0 }
        
    /// Sets Turn signal left

    pub fn set_BLI_LI(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffeffffff) | ((value as u64) & 0x1) << 24; }

    /// Gets Turn signal left
    pub fn get_BLI_LI(&self) -> bool { (self.0 >> 24 & 0x1) != 0 }
        
    /// Sets LF/ABC 2-position switch actuated

    pub fn set_ST2_BET(&mut self, value: EZS_240h_ST2_BET){ self.0 = (self.0 & 0xffffffffff3fffff) | ((value as u64) & 0x3) << 22; }

    /// Gets LF/ABC 2-position switch actuated
    pub fn get_ST2_BET(&self) -> Option<EZS_240h_ST2_BET> {  EZS_240h_ST2_BET::try_from((self.0 >> 22 & 0x3) as u8).ok() }
        
    /// Sets LF/ABC 3-position switch operated

    pub fn set_ST3_BET(&mut self, value: EZS_240h_ST3_BET){ self.0 = (self.0 & 0xffffffffffcfffff) | ((value as u64) & 0x3) << 20; }

    /// Gets LF/ABC 3-position switch operated
    pub fn get_ST3_BET(&self) -> Option<EZS_240h_ST3_BET> {  EZS_240h_ST3_BET::try_from((self.0 >> 20 & 0x3) as u8).ok() }
        
    /// Sets ART distance warning on/off actuated

    pub fn set_ART_ABW_BET(&mut self, value: EZS_240h_ART_ABW_BET){ self.0 = (self.0 & 0xfffffffffff3ffff) | ((value as u64) & 0x3) << 18; }

    /// Gets ART distance warning on/off actuated
    pub fn get_ART_ABW_BET(&self) -> Option<EZS_240h_ART_ABW_BET> {  EZS_240h_ART_ABW_BET::try_from((self.0 >> 18 & 0x3) as u8).ok() }
        
    /// Sets Turn on low beam

    pub fn set_ABL_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets Turn on low beam
    pub fn get_ABL_EIN(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets Terminal 54 hardware active

    pub fn set_KL54_RM(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets Terminal 54 hardware active
    pub fn get_KL54_RM(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
    /// Sets distance factor. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_ART_ABSTAND(&mut self, value: u8){ self.0 = (self.0 & 0xffffffffffff00ff) | ((value as u64) & 0xff) << 8; }

    /// Gets distance factor. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_ART_ABSTAND(&self) -> u8 { (self.0 >> 8 & 0xff) as u8 }
        
    /// Sets ART available

    pub fn set_ART_VH(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffff7f) | ((value as u64) & 0x1) << 7; }

    /// Gets ART available
    pub fn get_ART_VH(&self) -> bool { (self.0 >> 7 & 0x1) != 0 }
        
    /// Sets E-suction fan: Basic ventilation off

    pub fn set_GBL_AUS(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffbf) | ((value as u64) & 0x1) << 6; }

    /// Gets E-suction fan: Basic ventilation off
    pub fn get_GBL_AUS(&self) -> bool { (self.0 >> 6 & 0x1) != 0 }
        
    /// Sets Series-dependent vehicle version (only 220/215/230)

    pub fn set_FZGVERSN(&mut self, value: EZS_240h_FZGVERSN){ self.0 = (self.0 & 0xffffffffffffffe3) | ((value as u64) & 0x7) << 2; }

    /// Gets Series-dependent vehicle version (only 220/215/230)
    pub fn get_FZGVERSN(&self) -> Option<EZS_240h_FZGVERSN> {  EZS_240h_FZGVERSN::try_from((self.0 >> 2 & 0x7) as u8).ok() }
        
    /// Sets country code

    pub fn set_LDC(&mut self, value: EZS_240h_LDC){ self.0 = (self.0 & 0xfffffffffffffffc) | ((value as u64) & 0x3); }

    /// Gets country code
    pub fn get_LDC(&self) -> Option<EZS_240h_LDC> {  EZS_240h_LDC::try_from((self.0 & 0x3) as u8).ok() }
        
}