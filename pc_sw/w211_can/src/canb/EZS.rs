
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'EZS'
*/
    
pub const EZS_A1_CAN_ID: u16 = 0x0000;
pub const EZS_A12_CAN_ID: u16 = 0x0180;
pub const EZS_A4_CAN_ID: u16 = 0x0058;
pub const EZS_A9_CAN_ID: u16 = 0x00B2;
pub const KG_A1_CAN_ID: u16 = 0x01B2;
pub const KG_A2_CAN_ID: u16 = 0x0050;
pub const EZS_ANZ_CAN_ID: u16 = 0x0332;
pub const SD_RS_EZS_CAN_ID: u16 = 0x07C0;

/// Current memory block number
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_A1_SPEI_NR {
	SPEICHER1 = 0, // memory block 1
	SPEICHER2 = 1, // memory block 2
	SPEICHER3 = 2, // memory block 3
	NICHT_DEFINIERT_3 = 3, // Unknown
	NICHT_DEFINIERT_4 = 4, // Unknown
	NICHT_DEFINIERT_5 = 5, // Unknown
	NICHT_DEFINIERT_6 = 6, // Unknown
	SIGNAL_NICHT_VERFUGBAR = 7, // Unknown
}

impl TryFrom<u8> for EZS_A1_SPEI_NR {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SPEICHER1),
			1 => Ok(Self::SPEICHER2),
			2 => Ok(Self::SPEICHER3),
			3 => Ok(Self::NICHT_DEFINIERT_3),
			4 => Ok(Self::NICHT_DEFINIERT_4),
			5 => Ok(Self::NICHT_DEFINIERT_5),
			6 => Ok(Self::NICHT_DEFINIERT_6),
			7 => Ok(Self::SIGNAL_NICHT_VERFUGBAR),
			_ => Err(())
		}
	}
}
/// Mobility account status
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_A12_MOB_STAT {
	BEDGET_VAL = 0, // budget valid
	BEDGET_REQ = 1, // budget request
	FAHRER_INFO = 2, // status "Inform driver"
	FAHRER_WARN = 3, // status "warn driver"
	BUDGET_CON = 4, // budget consumed
	STAT_TEST = 5, // status test
	N_DEF = 6, // undefined
	SNV = 7, // SNV
}

impl TryFrom<u8> for EZS_A12_MOB_STAT {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::BEDGET_VAL),
			1 => Ok(Self::BEDGET_REQ),
			2 => Ok(Self::FAHRER_INFO),
			3 => Ok(Self::FAHRER_WARN),
			4 => Ok(Self::BUDGET_CON),
			5 => Ok(Self::STAT_TEST),
			6 => Ok(Self::N_DEF),
			7 => Ok(Self::SNV),
			_ => Err(())
		}
	}
}
/// VIN signal part
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub enum EZS_A9_VIN_MSG {
	N_DEF = 0, // undefined
	LO = 1, // VIN characters 1 - 7
	MID = 2, // VIN characters 8 - 14
	HI = 3, // VIN characters 15 - 17
}

impl TryFrom<u8> for EZS_A9_VIN_MSG {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::N_DEF),
			1 => Ok(Self::LO),
			2 => Ok(Self::MID),
			3 => Ok(Self::HI),
			_ => Err(())
		}
	}
}

pub struct EZS_A1(u64);

impl EZS_A1 {

	/// Gets CAN ID of EZS_A1
	pub fn get_canid() -> u16 { EZS_A1_CAN_ID }
    /// Sets Keyless Go terminal control active

    pub fn set_KG_KL_AKT(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Keyless Go terminal control active
    pub fn get_KG_KL_AKT(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Terminal 50 is switched on

    pub fn set_KL_50_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Terminal 50 is switched on
    pub fn get_KL_50_EIN(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Terminal 15X is on

    pub fn set_KL_15X_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Terminal 15X is on
    pub fn get_KL_15X_EIN(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Terminal 15 is switched on

    pub fn set_KL_15_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Terminal 15 is switched on
    pub fn get_KL_15_EIN(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Terminal 15R is on

    pub fn set_KL_15R_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Terminal 15R is on
    pub fn get_KL_15R_EIN(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Terminal 15C is on

    pub fn set_KL_15C_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Terminal 15C is on
    pub fn get_KL_15C_EIN(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Message: "Vehicle calculates, please wait"

    pub fn set_FZG_RECH(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets Message: "Vehicle calculates, please wait"
    pub fn get_FZG_RECH(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets Diagnostic toggle bit

    pub fn set_DIAG_TGL(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets Diagnostic toggle bit
    pub fn get_DIAG_TGL(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets Do not send application IDs, only NM IDs

    pub fn set_APPL_AUS(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets Do not send application IDs, only NM IDs
    pub fn get_APPL_AUS(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets Panic alarm by key off

    pub fn set_PNK_ALM_AUS(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets Panic alarm by key off
    pub fn get_PNK_ALM_AUS(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets Panic alarm by key on

    pub fn set_PNK_ALM_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets Panic alarm by key on
    pub fn get_PNK_ALM_EIN(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets Remote trigger MSS alarm

    pub fn set_FERN_ALARM(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets Remote trigger MSS alarm
    pub fn get_FERN_ALARM(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets Message: Renew key

    pub fn set_SCHLUE_NEU(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets Message: Renew key
    pub fn get_SCHLUE_NEU(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets Passive closure

    pub fn set_ZV_PASSIV(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets Passive closure
    pub fn get_ZV_PASSIV(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets Stop boot lid

    pub fn set_HD_STOPP(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets Stop boot lid
    pub fn get_HD_STOPP(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets Do not lock ZV while SBC added value is active

    pub fn set_ZV_N_VER_MW_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets Do not lock ZV while SBC added value is active
    pub fn get_ZV_N_VER_MW_AKT(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets Current memory block number

    pub fn set_SPEI_NR(&mut self, value: EZS_A1_SPEI_NR){ self.0 = (self.0 & 0xfffff8ffffffffff) | ((value as u64) & 0x7) << 40; }

    /// Gets Current memory block number
    pub fn get_SPEI_NR(&self) -> std::result::Result<EZS_A1_SPEI_NR, ()> { return EZS_A1_SPEI_NR::try_from((self.0 >> 40 & 0x7) as u8) }
        
    /// Sets external security

    pub fn set_AUSS_SICH(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets external security
    pub fn get_AUSS_SICH(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets external arming

    pub fn set_AUSS_ENTSI(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets external arming
    pub fn get_AUSS_ENTSI(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets Secure ZV blinker feedback

    pub fn set_BLI_SICH(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets Secure ZV blinker feedback
    pub fn get_BLI_SICH(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets Unlock ZV indicator feedback

    pub fn set_BLI_ENTSI(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets Unlock ZV indicator feedback
    pub fn get_BLI_ENTSI(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
    /// Sets Trunk lid remote release

    pub fn set_HFE_EZS(&mut self, value: bool){ self.0 = (self.0 & 0xffffffff7fffffff) | ((value as u64) & 0x1) << 31; }

    /// Gets Trunk lid remote release
    pub fn get_HFE_EZS(&self) -> bool { (self.0 >> 31 & 0x1) != 0 }
        
    /// Sets Secure boot lid

    pub fn set_HD_SICH(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffbfffffff) | ((value as u64) & 0x1) << 30; }

    /// Gets Secure boot lid
    pub fn get_HD_SICH(&self) -> bool { (self.0 >> 30 & 0x1) != 0 }
        
    /// Sets Unlock trunk lid

    pub fn set_HD_ENTSI(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffdfffffff) | ((value as u64) & 0x1) << 29; }

    /// Gets Unlock trunk lid
    pub fn get_HD_ENTSI(&self) -> bool { (self.0 >> 29 & 0x1) != 0 }
        
    /// Sets Lock fuel cap

    pub fn set_TD_VERRI(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffefffffff) | ((value as u64) & 0x1) << 28; }

    /// Gets Lock fuel cap
    pub fn get_TD_VERRI(&self) -> bool { (self.0 >> 28 & 0x1) != 0 }
        
    /// Sets Unlock the tank cap

    pub fn set_TD_ENTRI(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffff7ffffff) | ((value as u64) & 0x1) << 27; }

    /// Gets Unlock the tank cap
    pub fn get_TD_ENTRI(&self) -> bool { (self.0 >> 27 & 0x1) != 0 }
        
    /// Sets Open tailgate remotely

    pub fn set_RWTFE_EZS(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffbffffff) | ((value as u64) & 0x1) << 26; }

    /// Gets Open tailgate remotely
    pub fn get_RWTFE_EZS(&self) -> bool { (self.0 >> 26 & 0x1) != 0 }
        
    /// Sets ZV post-locking

    pub fn set_ZV_NV(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffdffffff) | ((value as u64) & 0x1) << 25; }

    /// Gets ZV post-locking
    pub fn get_ZV_NV(&self) -> bool { (self.0 >> 25 & 0x1) != 0 }
        
    /// Sets Mechanical / FB key active

    pub fn set_SCHL_BEF(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffeffffff) | ((value as u64) & 0x1) << 24; }

    /// Gets Mechanical / FB key active
    pub fn get_SCHL_BEF(&self) -> bool { (self.0 >> 24 & 0x1) != 0 }
        
    /// Sets Lock rear right door

    pub fn set_THR_VERRI(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffff7fffff) | ((value as u64) & 0x1) << 23; }

    /// Gets Lock rear right door
    pub fn get_THR_VERRI(&self) -> bool { (self.0 >> 23 & 0x1) != 0 }
        
    /// Sets Unlock rear right door

    pub fn set_THR_ENTRI(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffbfffff) | ((value as u64) & 0x1) << 22; }

    /// Gets Unlock rear right door
    pub fn get_THR_ENTRI(&self) -> bool { (self.0 >> 22 & 0x1) != 0 }
        
    /// Sets Lock rear left door

    pub fn set_THL_VERRI(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffdfffff) | ((value as u64) & 0x1) << 21; }

    /// Gets Lock rear left door
    pub fn get_THL_VERRI(&self) -> bool { (self.0 >> 21 & 0x1) != 0 }
        
    /// Sets Unlock rear left door

    pub fn set_THL_ENTRI(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffefffff) | ((value as u64) & 0x1) << 20; }

    /// Gets Unlock rear left door
    pub fn get_THL_ENTRI(&self) -> bool { (self.0 >> 20 & 0x1) != 0 }
        
    /// Sets Lock front right door

    pub fn set_TVR_VERRI(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffff7ffff) | ((value as u64) & 0x1) << 19; }

    /// Gets Lock front right door
    pub fn get_TVR_VERRI(&self) -> bool { (self.0 >> 19 & 0x1) != 0 }
        
    /// Sets Unlock front right door

    pub fn set_TVR_ENTRI(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffbffff) | ((value as u64) & 0x1) << 18; }

    /// Gets Unlock front right door
    pub fn get_TVR_ENTRI(&self) -> bool { (self.0 >> 18 & 0x1) != 0 }
        
    /// Sets Lock front left door

    pub fn set_TVL_VERRI(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets Lock front left door
    pub fn get_TVL_VERRI(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets Unlock front left door

    pub fn set_TVL_ENTRI(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets Unlock front left door
    pub fn get_TVL_ENTRI(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
}
pub struct EZS_A12(u64);

impl EZS_A12 {

	/// Gets CAN ID of EZS_A12
	pub fn get_canid() -> u16 { EZS_A12_CAN_ID }
    /// Sets Mobility account status

    pub fn set_MOB_STAT(&mut self, value: EZS_A12_MOB_STAT){ self.0 = (self.0 & 0x1fffffffffffffff) | ((value as u64) & 0x7) << 61; }

    /// Gets Mobility account status
    pub fn get_MOB_STAT(&self) -> std::result::Result<EZS_A12_MOB_STAT, ()> { return EZS_A12_MOB_STAT::try_from((self.0 >> 61 & 0x7) as u8) }
        
    /// Sets Mobility account activated

    pub fn set_MOB_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets Mobility account activated
    pub fn get_MOB_AKT(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
}
pub struct EZS_A4(u64);

impl EZS_A4 {

	/// Gets CAN ID of EZS_A4
	pub fn get_canid() -> u16 { EZS_A4_CAN_ID }
    /// Sets KeyID from EZS. Conversion formula (To raw from real): y=(x-0.0)/1.00

    pub fn set_SCHLUE_ID(&mut self, value: u32){ self.0 = (self.0 & 0x00000000ffffffff) | ((value as u64) & 0xffffffff) << 32; }

    /// Gets KeyID from EZS. Conversion formula (To real from raw): y=(1.00x)+0.0
    pub fn get_SCHLUE_ID(&self) -> u32 { (self.0 >> 32 & 0xffffffff) as u32 }
        
    /// Sets kilometer reading sent by the EZS. Conversion formula (To raw from real): y=(x-0.0)/0.10 (Unit: km)

    pub fn set_KM_EZS(&mut self, value: u32){ self.0 = (self.0 & 0xffffffff000000ff) | ((value as u64) & 0xffffff) << 8; }

    /// Gets kilometer reading sent by the EZS. Conversion formula (To real from raw): y=(0.10x)+0.0 (Unit: km)
    pub fn get_KM_EZS(&self) -> u32 { (self.0 >> 8 & 0xffffff) as u32 }
        
}
pub struct EZS_A9(u64);

impl EZS_A9 {

	/// Gets CAN ID of EZS_A9
	pub fn get_canid() -> u16 { EZS_A9_CAN_ID }
    /// Sets VIN signal part

    pub fn set_VIN_MSG(&mut self, value: EZS_A9_VIN_MSG){ self.0 = (self.0 & 0xfcffffffffffffff) | ((value as u64) & 0x3) << 56; }

    /// Gets VIN signal part
    pub fn get_VIN_MSG(&self) -> std::result::Result<EZS_A9_VIN_MSG, ()> { return EZS_A9_VIN_MSG::try_from((self.0 >> 56 & 0x3) as u8) }
        
}
pub struct KG_A1(u64);

impl KG_A1 {

	/// Gets CAN ID of KG_A1
	pub fn get_canid() -> u16 { KG_A1_CAN_ID }
    /// Sets Message 5: "Please selector lever in P or N position"

    pub fn set_M5(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Message 5: "Please selector lever in P or N position"
    pub fn get_M5(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Message 4: "Check chip card/key battery" (white)

    pub fn set_M4(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Message 4: "Check chip card/key battery" (white)
    pub fn get_M4(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Message 3: "Selector lever to P" (red, continuous tone)

    pub fn set_M3(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Message 3: "Selector lever to P" (red, continuous tone)
    pub fn get_M3(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Message 2: "Chip card/key detected in vehicle" (white)

    pub fn set_M2(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Message 2: "Chip card/key detected in vehicle" (white)
    pub fn get_M2(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Message 1: "Chip card/key not recognized" (white)

    pub fn set_M1(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Message 1: "Chip card/key not recognized" (white)
    pub fn get_M1(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Message 0: "Chip card/key not recognized" (red)

    pub fn set_M0(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Message 0: "Chip card/key not recognized" (red)
    pub fn get_M0(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Turn on warning sound

    pub fn set_WARNTON_KG(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Turn on warning sound
    pub fn get_WARNTON_KG(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Message 12: "Take your chip card/key with you!"

    pub fn set_M12(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets Message 12: "Take your chip card/key with you!"
    pub fn get_M12(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets Message 11: "Reserved"

    pub fn set_M11(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets Message 11: "Reserved"
    pub fn get_M11(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets Message 10: "Reserved"

    pub fn set_M10(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets Message 10: "Reserved"
    pub fn get_M10(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets Message 9: "Keyless Go into Diagnosis"

    pub fn set_M9(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets Message 9: "Keyless Go into Diagnosis"
    pub fn get_M9(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets Message 8: "door open"

    pub fn set_M8(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets Message 8: "door open"
    pub fn get_M8(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets Message 7: Reserved

    pub fn set_M7(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets Message 7: Reserved
    pub fn get_M7(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets Message 6: "Reserved"

    pub fn set_M6(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets Message 6: "Reserved"
    pub fn get_M6(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets Keyless Go route indication. Conversion formula (To raw from real): y=(x-0.0)/1.00 (Unit: km)

    pub fn set_KM_REST_KG(&mut self, value: u8){ self.0 = (self.0 & 0xffff00ffffffffff) | ((value as u64) & 0xff) << 40; }

    /// Gets Keyless Go route indication. Conversion formula (To real from raw): y=(1.00x)+0.0 (Unit: km)
    pub fn get_KM_REST_KG(&self) -> u8 { (self.0 >> 40 & 0xff) as u8 }
        
}
pub struct KG_A2(u64);

impl KG_A2 {

	/// Gets CAN ID of KG_A2
	pub fn get_canid() -> u16 { KG_A2_CAN_ID }
    /// Sets Open/close rear right window

    pub fn set_FHR_KG(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Open/close rear right window
    pub fn get_FHR_KG(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Open/close rear left window

    pub fn set_FHL_KG(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Open/close rear left window
    pub fn get_FHL_KG(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Open/close front right window

    pub fn set_FVR_KG(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Open/close front right window
    pub fn get_FVR_KG(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Open/close front left window

    pub fn set_FVL_KG(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Open/close front left window
    pub fn get_FVL_KG(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Open/close SHD

    pub fn set_SHD_KG(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Open/close SHD
    pub fn get_SHD_KG(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets direction comfort operation: open [0], close [1]

    pub fn set_KB_RI_KG(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets direction comfort operation: open [0], close [1]
    pub fn get_KB_RI_KG(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Comfort control Automatic operation [1] Manual [0]

    pub fn set_KB_MOD_KG(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Comfort control Automatic operation [1] Manual [0]
    pub fn get_KB_MOD_KG(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
}
pub struct EZS_ANZ(u64);

impl EZS_ANZ {

	/// Gets CAN ID of EZS_ANZ
	pub fn get_canid() -> u16 { EZS_ANZ_CAN_ID }
    /// Sets Message 7: "MK "

    pub fn set_EZS_M7(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Message 7: "MK "
    pub fn get_EZS_M7(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Message 6: "MK manual reloading failed"

    pub fn set_EZS_M6(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Message 6: "MK manual reloading failed"
    pub fn get_EZS_M6(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Message 5: "MK manual reloading successful"

    pub fn set_EZS_M5(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Message 5: "MK manual reloading successful"
    pub fn get_EZS_M5(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Message 4: "MK manual reloading carried out"

    pub fn set_EZS_M4(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Message 4: "MK manual reloading carried out"
    pub fn get_EZS_M4(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Message 3: "MK authorization error"

    pub fn set_EZS_M3(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Message 3: "MK authorization error"
    pub fn get_EZS_M3(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Message 2: "MK expired, vehicle blocked"

    pub fn set_EZS_M2(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Message 2: "MK expired, vehicle blocked"
    pub fn get_EZS_M2(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Message 1: "Reload MK immediately"

    pub fn set_EZS_M1(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Message 1: "Reload MK immediately"
    pub fn get_EZS_M1(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets Message 0: "MK status reduced"

    pub fn set_EZS_M0(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets Message 0: "MK status reduced"
    pub fn get_EZS_M0(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
}
pub struct SD_RS_EZS(u64);

impl SD_RS_EZS {

	/// Gets CAN ID of SD_RS_EZS
	pub fn get_canid() -> u16 { SD_RS_EZS_CAN_ID }
    /// Sets Identification for > 8 bytes

    pub fn set_EZS_KENN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Identification for > 8 bytes
    pub fn get_EZS_KENN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets error vector 07h

    pub fn set_EZS_FV07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets error vector 07h
    pub fn get_EZS_FV07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets error vector 06h

    pub fn set_EZS_FV06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets error vector 06h
    pub fn get_EZS_FV06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets error vector 05h

    pub fn set_EZS_FV05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets error vector 05h
    pub fn get_EZS_FV05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets error vector 04h

    pub fn set_EZS_FV04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets error vector 04h
    pub fn get_EZS_FV04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets error vector 03h

    pub fn set_EZS_FV03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets error vector 03h
    pub fn get_EZS_FV03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets error vector 02h

    pub fn set_EZS_FV02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets error vector 02h
    pub fn get_EZS_FV02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets error vector 01h

    pub fn set_EZS_FV01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets error vector 01h
    pub fn get_EZS_FV01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets error vector 0Fh

    pub fn set_EZS_FV0F(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets error vector 0Fh
    pub fn get_EZS_FV0F(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets error vector 0Eh

    pub fn set_EZS_FV0E(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets error vector 0Eh
    pub fn get_EZS_FV0E(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets error vector 0Dh

    pub fn set_EZS_FV0D(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets error vector 0Dh
    pub fn get_EZS_FV0D(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets error vector 0Ch

    pub fn set_EZS_FV0C(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets error vector 0Ch
    pub fn get_EZS_FV0C(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets error vector 0Bh

    pub fn set_EZS_FV0B(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets error vector 0Bh
    pub fn get_EZS_FV0B(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets error vector 0Ah

    pub fn set_EZS_FV0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets error vector 0Ah
    pub fn get_EZS_FV0A(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets error vector 09h

    pub fn set_EZS_FV09(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets error vector 09h
    pub fn get_EZS_FV09(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets error vector 08h

    pub fn set_EZS_FV08(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets error vector 08h
    pub fn get_EZS_FV08(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets error vector 17h

    pub fn set_EZS_FV17(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets error vector 17h
    pub fn get_EZS_FV17(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets error vector 16h

    pub fn set_EZS_FV16(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets error vector 16h
    pub fn get_EZS_FV16(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets error vector 15h

    pub fn set_EZS_FV15(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets error vector 15h
    pub fn get_EZS_FV15(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets error vector 14h

    pub fn set_EZS_FV14(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets error vector 14h
    pub fn get_EZS_FV14(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets error vector 13h

    pub fn set_EZS_FV13(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets error vector 13h
    pub fn get_EZS_FV13(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets error vector 12h

    pub fn set_EZS_FV12(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets error vector 12h
    pub fn get_EZS_FV12(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets error vector 11h

    pub fn set_EZS_FV11(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets error vector 11h
    pub fn get_EZS_FV11(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets error vector 10h

    pub fn set_EZS_FV10(&mut self, value: bool){ self.0 = (self.0 & 0xfffffeffffffffff) | ((value as u64) & 0x1) << 40; }

    /// Gets error vector 10h
    pub fn get_EZS_FV10(&self) -> bool { (self.0 >> 40 & 0x1) != 0 }
        
    /// Sets error vector 1Fh

    pub fn set_EZS_FV1F(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets error vector 1Fh
    pub fn get_EZS_FV1F(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets error vector 1Eh

    pub fn set_EZS_FV1E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets error vector 1Eh
    pub fn get_EZS_FV1E(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets error vector 1Dh

    pub fn set_EZS_FV1D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets error vector 1Dh
    pub fn get_EZS_FV1D(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets Error vector 1Ch

    pub fn set_EZS_FV1C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets Error vector 1Ch
    pub fn get_EZS_FV1C(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
    /// Sets error vector 1Bh

    pub fn set_EZS_FV1B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffff7ffffffff) | ((value as u64) & 0x1) << 35; }

    /// Gets error vector 1Bh
    pub fn get_EZS_FV1B(&self) -> bool { (self.0 >> 35 & 0x1) != 0 }
        
    /// Sets Error vector 1Ah

    pub fn set_EZS_FV1A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffbffffffff) | ((value as u64) & 0x1) << 34; }

    /// Gets Error vector 1Ah
    pub fn get_EZS_FV1A(&self) -> bool { (self.0 >> 34 & 0x1) != 0 }
        
    /// Sets error vector 19h

    pub fn set_EZS_FV19(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffdffffffff) | ((value as u64) & 0x1) << 33; }

    /// Gets error vector 19h
    pub fn get_EZS_FV19(&self) -> bool { (self.0 >> 33 & 0x1) != 0 }
        
    /// Sets error vector 18h

    pub fn set_EZS_FV18(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffeffffffff) | ((value as u64) & 0x1) << 32; }

    /// Gets error vector 18h
    pub fn get_EZS_FV18(&self) -> bool { (self.0 >> 32 & 0x1) != 0 }
        
    /// Sets error vector 27h

    pub fn set_EZS_FV27(&mut self, value: bool){ self.0 = (self.0 & 0xffffffff7fffffff) | ((value as u64) & 0x1) << 31; }

    /// Gets error vector 27h
    pub fn get_EZS_FV27(&self) -> bool { (self.0 >> 31 & 0x1) != 0 }
        
    /// Sets error vector 26h

    pub fn set_EZS_FV26(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffbfffffff) | ((value as u64) & 0x1) << 30; }

    /// Gets error vector 26h
    pub fn get_EZS_FV26(&self) -> bool { (self.0 >> 30 & 0x1) != 0 }
        
    /// Sets error vector 25h

    pub fn set_EZS_FV25(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffdfffffff) | ((value as u64) & 0x1) << 29; }

    /// Gets error vector 25h
    pub fn get_EZS_FV25(&self) -> bool { (self.0 >> 29 & 0x1) != 0 }
        
    /// Sets error vector 24h

    pub fn set_EZS_FV24(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffefffffff) | ((value as u64) & 0x1) << 28; }

    /// Gets error vector 24h
    pub fn get_EZS_FV24(&self) -> bool { (self.0 >> 28 & 0x1) != 0 }
        
    /// Sets error vector 23h

    pub fn set_EZS_FV23(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffff7ffffff) | ((value as u64) & 0x1) << 27; }

    /// Gets error vector 23h
    pub fn get_EZS_FV23(&self) -> bool { (self.0 >> 27 & 0x1) != 0 }
        
    /// Sets error vector 22h

    pub fn set_EZS_FV22(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffbffffff) | ((value as u64) & 0x1) << 26; }

    /// Gets error vector 22h
    pub fn get_EZS_FV22(&self) -> bool { (self.0 >> 26 & 0x1) != 0 }
        
    /// Sets error vector 21h

    pub fn set_EZS_FV21(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffdffffff) | ((value as u64) & 0x1) << 25; }

    /// Gets error vector 21h
    pub fn get_EZS_FV21(&self) -> bool { (self.0 >> 25 & 0x1) != 0 }
        
    /// Sets error vector 20h

    pub fn set_EZS_FV20(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffeffffff) | ((value as u64) & 0x1) << 24; }

    /// Gets error vector 20h
    pub fn get_EZS_FV20(&self) -> bool { (self.0 >> 24 & 0x1) != 0 }
        
    /// Sets error vector 2Fh

    pub fn set_EZS_FV2F(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffff7fffff) | ((value as u64) & 0x1) << 23; }

    /// Gets error vector 2Fh
    pub fn get_EZS_FV2F(&self) -> bool { (self.0 >> 23 & 0x1) != 0 }
        
    /// Sets error vector 2Eh

    pub fn set_EZS_FV2E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffbfffff) | ((value as u64) & 0x1) << 22; }

    /// Gets error vector 2Eh
    pub fn get_EZS_FV2E(&self) -> bool { (self.0 >> 22 & 0x1) != 0 }
        
    /// Sets error vector 2Dh

    pub fn set_EZS_FV2D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffdfffff) | ((value as u64) & 0x1) << 21; }

    /// Gets error vector 2Dh
    pub fn get_EZS_FV2D(&self) -> bool { (self.0 >> 21 & 0x1) != 0 }
        
    /// Sets error vector 2Ch

    pub fn set_EZS_FV2C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffefffff) | ((value as u64) & 0x1) << 20; }

    /// Gets error vector 2Ch
    pub fn get_EZS_FV2C(&self) -> bool { (self.0 >> 20 & 0x1) != 0 }
        
    /// Sets error vector 2Bh

    pub fn set_EZS_FV2B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffff7ffff) | ((value as u64) & 0x1) << 19; }

    /// Gets error vector 2Bh
    pub fn get_EZS_FV2B(&self) -> bool { (self.0 >> 19 & 0x1) != 0 }
        
    /// Sets Error vector 2Ah

    pub fn set_EZS_FV2A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffbffff) | ((value as u64) & 0x1) << 18; }

    /// Gets Error vector 2Ah
    pub fn get_EZS_FV2A(&self) -> bool { (self.0 >> 18 & 0x1) != 0 }
        
    /// Sets error vector 29h

    pub fn set_EZS_FV29(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets error vector 29h
    pub fn get_EZS_FV29(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets error vector 28h

    pub fn set_EZS_FV28(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets error vector 28h
    pub fn get_EZS_FV28(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
    /// Sets state variable 04h

    pub fn set_EZS_PGV04(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffff7fff) | ((value as u64) & 0x1) << 15; }

    /// Gets state variable 04h
    pub fn get_EZS_PGV04(&self) -> bool { (self.0 >> 15 & 0x1) != 0 }
        
    /// Sets state variable 03h

    pub fn set_EZS_PGV03(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffbfff) | ((value as u64) & 0x1) << 14; }

    /// Gets state variable 03h
    pub fn get_EZS_PGV03(&self) -> bool { (self.0 >> 14 & 0x1) != 0 }
        
    /// Sets state variable 02h

    pub fn set_EZS_PGV02(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffdfff) | ((value as u64) & 0x1) << 13; }

    /// Gets state variable 02h
    pub fn get_EZS_PGV02(&self) -> bool { (self.0 >> 13 & 0x1) != 0 }
        
    /// Sets state variable 01h

    pub fn set_EZS_PGV01(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffefff) | ((value as u64) & 0x1) << 12; }

    /// Gets state variable 01h
    pub fn get_EZS_PGV01(&self) -> bool { (self.0 >> 12 & 0x1) != 0 }
        
    /// Sets error vector 33h

    pub fn set_EZS_FV33(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffff7ff) | ((value as u64) & 0x1) << 11; }

    /// Gets error vector 33h
    pub fn get_EZS_FV33(&self) -> bool { (self.0 >> 11 & 0x1) != 0 }
        
    /// Sets error vector 32h

    pub fn set_EZS_FV32(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffbff) | ((value as u64) & 0x1) << 10; }

    /// Gets error vector 32h
    pub fn get_EZS_FV32(&self) -> bool { (self.0 >> 10 & 0x1) != 0 }
        
    /// Sets error vector 31h

    pub fn set_EZS_FV31(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffdff) | ((value as u64) & 0x1) << 9; }

    /// Gets error vector 31h
    pub fn get_EZS_FV31(&self) -> bool { (self.0 >> 9 & 0x1) != 0 }
        
    /// Sets error vector 30h

    pub fn set_EZS_FV30(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffeff) | ((value as u64) & 0x1) << 8; }

    /// Gets error vector 30h
    pub fn get_EZS_FV30(&self) -> bool { (self.0 >> 8 & 0x1) != 0 }
        
}