
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'STH'
*/
    
pub const STH_A1_CAN_ID: u16 = 0x0094;
pub const SD_RS_STH_CAN_ID: u16 = 0x07D9;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct STH_A1(pub u64);

impl STH_A1 {

	/// Gets CAN ID of STH_A1
	pub const fn get_canid() -> u16 { STH_A1_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Switch on auxiliary heating/ventilation

    pub fn set_STHL_EIN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Switch on auxiliary heating/ventilation
    pub fn get_STHL_EIN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Switch off auxiliary heating/ventilation

    pub fn set_STHL_AUS(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Switch off auxiliary heating/ventilation
    pub fn get_STHL_AUS(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Turn on vehicle fan

    pub fn set_GEBLAESE_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Turn on vehicle fan
    pub fn get_GEBLAESE_EIN(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Open preset time menu

    pub fn set_VWZ_MENUE(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Open preset time menu
    pub fn get_VWZ_MENUE(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Transmitter learning mode on

    pub fn set_SENDLM_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets Transmitter learning mode on
    pub fn get_SENDLM_EIN(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SD_RS_STH(pub u64);

impl SD_RS_STH {

	/// Gets CAN ID of SD_RS_STH
	pub const fn get_canid() -> u16 { SD_RS_STH_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Identification for > 8 bytes

    pub fn set_STH_KENN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Identification for > 8 bytes
    pub fn get_STH_KENN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets error vector 07h

    pub fn set_STH_FV07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets error vector 07h
    pub fn get_STH_FV07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets error vector 06h

    pub fn set_STH_FV06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets error vector 06h
    pub fn get_STH_FV06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets error vector 05h

    pub fn set_STH_FV05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets error vector 05h
    pub fn get_STH_FV05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets error vector 04h

    pub fn set_STH_FV04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets error vector 04h
    pub fn get_STH_FV04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets error vector 03h

    pub fn set_STH_FV03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets error vector 03h
    pub fn get_STH_FV03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets error vector 02h

    pub fn set_STH_FV02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets error vector 02h
    pub fn get_STH_FV02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets error vector 01h

    pub fn set_STH_FV01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets error vector 01h
    pub fn get_STH_FV01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets error vector 0Fh

    pub fn set_STH_FV0F(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets error vector 0Fh
    pub fn get_STH_FV0F(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets error vector 0Eh

    pub fn set_STH_FV0E(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets error vector 0Eh
    pub fn get_STH_FV0E(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets error vector 0Dh

    pub fn set_STH_FV0D(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets error vector 0Dh
    pub fn get_STH_FV0D(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets error vector 0Ch

    pub fn set_STH_FV0C(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets error vector 0Ch
    pub fn get_STH_FV0C(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets error vector 0Bh

    pub fn set_STH_FV0B(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets error vector 0Bh
    pub fn get_STH_FV0B(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets error vector 0Ah

    pub fn set_STH_FV0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets error vector 0Ah
    pub fn get_STH_FV0A(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets error vector 09h

    pub fn set_STH_FV09(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets error vector 09h
    pub fn get_STH_FV09(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets error vector 08h

    pub fn set_STH_FV08(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets error vector 08h
    pub fn get_STH_FV08(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets error vector 17h

    pub fn set_STH_FV17(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets error vector 17h
    pub fn get_STH_FV17(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets error vector 16h

    pub fn set_STH_FV16(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets error vector 16h
    pub fn get_STH_FV16(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets error vector 15h

    pub fn set_STH_FV15(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets error vector 15h
    pub fn get_STH_FV15(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets error vector 14h

    pub fn set_STH_FV14(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets error vector 14h
    pub fn get_STH_FV14(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets error vector 13h

    pub fn set_STH_FV13(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets error vector 13h
    pub fn get_STH_FV13(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets error vector 12h

    pub fn set_STH_FV12(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets error vector 12h
    pub fn get_STH_FV12(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets error vector 11h

    pub fn set_STH_FV11(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets error vector 11h
    pub fn get_STH_FV11(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets error vector 10h

    pub fn set_STH_FV10(&mut self, value: bool){ self.0 = (self.0 & 0xfffffeffffffffff) | ((value as u64) & 0x1) << 40; }

    /// Gets error vector 10h
    pub fn get_STH_FV10(&self) -> bool { (self.0 >> 40 & 0x1) != 0 }
        
    /// Sets error vector 1Fh

    pub fn set_STH_FV1F(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets error vector 1Fh
    pub fn get_STH_FV1F(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets error vector 1Eh

    pub fn set_STH_FV1E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets error vector 1Eh
    pub fn get_STH_FV1E(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets error vector 1Dh

    pub fn set_STH_FV1D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets error vector 1Dh
    pub fn get_STH_FV1D(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets Error vector 1Ch

    pub fn set_STH_FV1C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets Error vector 1Ch
    pub fn get_STH_FV1C(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
    /// Sets error vector 1Bh

    pub fn set_STH_FV1B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffff7ffffffff) | ((value as u64) & 0x1) << 35; }

    /// Gets error vector 1Bh
    pub fn get_STH_FV1B(&self) -> bool { (self.0 >> 35 & 0x1) != 0 }
        
    /// Sets Error vector 1Ah

    pub fn set_STH_FV1A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffbffffffff) | ((value as u64) & 0x1) << 34; }

    /// Gets Error vector 1Ah
    pub fn get_STH_FV1A(&self) -> bool { (self.0 >> 34 & 0x1) != 0 }
        
    /// Sets error vector 19h

    pub fn set_STH_FV19(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffdffffffff) | ((value as u64) & 0x1) << 33; }

    /// Gets error vector 19h
    pub fn get_STH_FV19(&self) -> bool { (self.0 >> 33 & 0x1) != 0 }
        
    /// Sets error vector 18h

    pub fn set_STH_FV18(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffeffffffff) | ((value as u64) & 0x1) << 32; }

    /// Gets error vector 18h
    pub fn get_STH_FV18(&self) -> bool { (self.0 >> 32 & 0x1) != 0 }
        
    /// Sets error vector 27h

    pub fn set_STH_FV27(&mut self, value: bool){ self.0 = (self.0 & 0xffffffff7fffffff) | ((value as u64) & 0x1) << 31; }

    /// Gets error vector 27h
    pub fn get_STH_FV27(&self) -> bool { (self.0 >> 31 & 0x1) != 0 }
        
    /// Sets error vector 26h

    pub fn set_STH_FV26(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffbfffffff) | ((value as u64) & 0x1) << 30; }

    /// Gets error vector 26h
    pub fn get_STH_FV26(&self) -> bool { (self.0 >> 30 & 0x1) != 0 }
        
    /// Sets error vector 25h

    pub fn set_STH_FV25(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffdfffffff) | ((value as u64) & 0x1) << 29; }

    /// Gets error vector 25h
    pub fn get_STH_FV25(&self) -> bool { (self.0 >> 29 & 0x1) != 0 }
        
    /// Sets error vector 24h

    pub fn set_STH_FV24(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffefffffff) | ((value as u64) & 0x1) << 28; }

    /// Gets error vector 24h
    pub fn get_STH_FV24(&self) -> bool { (self.0 >> 28 & 0x1) != 0 }
        
    /// Sets error vector 23h

    pub fn set_STH_FV23(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffff7ffffff) | ((value as u64) & 0x1) << 27; }

    /// Gets error vector 23h
    pub fn get_STH_FV23(&self) -> bool { (self.0 >> 27 & 0x1) != 0 }
        
    /// Sets error vector 22h

    pub fn set_STH_FV22(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffbffffff) | ((value as u64) & 0x1) << 26; }

    /// Gets error vector 22h
    pub fn get_STH_FV22(&self) -> bool { (self.0 >> 26 & 0x1) != 0 }
        
    /// Sets error vector 21h

    pub fn set_STH_FV21(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffdffffff) | ((value as u64) & 0x1) << 25; }

    /// Gets error vector 21h
    pub fn get_STH_FV21(&self) -> bool { (self.0 >> 25 & 0x1) != 0 }
        
    /// Sets error vector 20h

    pub fn set_STH_FV20(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffeffffff) | ((value as u64) & 0x1) << 24; }

    /// Gets error vector 20h
    pub fn get_STH_FV20(&self) -> bool { (self.0 >> 24 & 0x1) != 0 }
        
    /// Sets error vector 2Fh

    pub fn set_STH_FV2F(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffff7fffff) | ((value as u64) & 0x1) << 23; }

    /// Gets error vector 2Fh
    pub fn get_STH_FV2F(&self) -> bool { (self.0 >> 23 & 0x1) != 0 }
        
    /// Sets error vector 2Eh

    pub fn set_STH_FV2E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffbfffff) | ((value as u64) & 0x1) << 22; }

    /// Gets error vector 2Eh
    pub fn get_STH_FV2E(&self) -> bool { (self.0 >> 22 & 0x1) != 0 }
        
    /// Sets error vector 2Dh

    pub fn set_STH_FV2D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffdfffff) | ((value as u64) & 0x1) << 21; }

    /// Gets error vector 2Dh
    pub fn get_STH_FV2D(&self) -> bool { (self.0 >> 21 & 0x1) != 0 }
        
    /// Sets error vector 2Ch

    pub fn set_STH_FV2C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffefffff) | ((value as u64) & 0x1) << 20; }

    /// Gets error vector 2Ch
    pub fn get_STH_FV2C(&self) -> bool { (self.0 >> 20 & 0x1) != 0 }
        
    /// Sets error vector 2Bh

    pub fn set_STH_FV2B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffff7ffff) | ((value as u64) & 0x1) << 19; }

    /// Gets error vector 2Bh
    pub fn get_STH_FV2B(&self) -> bool { (self.0 >> 19 & 0x1) != 0 }
        
    /// Sets Error vector 2Ah

    pub fn set_STH_FV2A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffbffff) | ((value as u64) & 0x1) << 18; }

    /// Gets Error vector 2Ah
    pub fn get_STH_FV2A(&self) -> bool { (self.0 >> 18 & 0x1) != 0 }
        
    /// Sets error vector 29h

    pub fn set_STH_FV29(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets error vector 29h
    pub fn get_STH_FV29(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets error vector 28h

    pub fn set_STH_FV28(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets error vector 28h
    pub fn get_STH_FV28(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
    /// Sets error vector 37h

    pub fn set_STH_FV37(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffff7fff) | ((value as u64) & 0x1) << 15; }

    /// Gets error vector 37h
    pub fn get_STH_FV37(&self) -> bool { (self.0 >> 15 & 0x1) != 0 }
        
    /// Sets error vector 36h

    pub fn set_STH_FV36(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffbfff) | ((value as u64) & 0x1) << 14; }

    /// Gets error vector 36h
    pub fn get_STH_FV36(&self) -> bool { (self.0 >> 14 & 0x1) != 0 }
        
    /// Sets error vector 35h

    pub fn set_STH_FV35(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffdfff) | ((value as u64) & 0x1) << 13; }

    /// Gets error vector 35h
    pub fn get_STH_FV35(&self) -> bool { (self.0 >> 13 & 0x1) != 0 }
        
    /// Sets error vector 34h

    pub fn set_STH_FV34(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffefff) | ((value as u64) & 0x1) << 12; }

    /// Gets error vector 34h
    pub fn get_STH_FV34(&self) -> bool { (self.0 >> 12 & 0x1) != 0 }
        
    /// Sets error vector 33h

    pub fn set_STH_FV33(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffff7ff) | ((value as u64) & 0x1) << 11; }

    /// Gets error vector 33h
    pub fn get_STH_FV33(&self) -> bool { (self.0 >> 11 & 0x1) != 0 }
        
    /// Sets error vector 32h

    pub fn set_STH_FV32(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffbff) | ((value as u64) & 0x1) << 10; }

    /// Gets error vector 32h
    pub fn get_STH_FV32(&self) -> bool { (self.0 >> 10 & 0x1) != 0 }
        
    /// Sets error vector 31h

    pub fn set_STH_FV31(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffdff) | ((value as u64) & 0x1) << 9; }

    /// Gets error vector 31h
    pub fn get_STH_FV31(&self) -> bool { (self.0 >> 9 & 0x1) != 0 }
        
    /// Sets error vector 30h

    pub fn set_STH_FV30(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffeff) | ((value as u64) & 0x1) << 8; }

    /// Gets error vector 30h
    pub fn get_STH_FV30(&self) -> bool { (self.0 >> 8 & 0x1) != 0 }
        
    /// Sets state variable 08h

    pub fn set_STH_PGV08(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffff7f) | ((value as u64) & 0x1) << 7; }

    /// Gets state variable 08h
    pub fn get_STH_PGV08(&self) -> bool { (self.0 >> 7 & 0x1) != 0 }
        
    /// Sets state variable 07h

    pub fn set_STH_PGV07(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffbf) | ((value as u64) & 0x1) << 6; }

    /// Gets state variable 07h
    pub fn get_STH_PGV07(&self) -> bool { (self.0 >> 6 & 0x1) != 0 }
        
    /// Sets state variable 06h

    pub fn set_STH_PGV06(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffdf) | ((value as u64) & 0x1) << 5; }

    /// Gets state variable 06h
    pub fn get_STH_PGV06(&self) -> bool { (self.0 >> 5 & 0x1) != 0 }
        
    /// Sets state variable 05h

    pub fn set_STH_PGV05(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffef) | ((value as u64) & 0x1) << 4; }

    /// Gets state variable 05h
    pub fn get_STH_PGV05(&self) -> bool { (self.0 >> 4 & 0x1) != 0 }
        
    /// Sets state variable 04h

    pub fn set_STH_PGV04(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffff7) | ((value as u64) & 0x1) << 3; }

    /// Gets state variable 04h
    pub fn get_STH_PGV04(&self) -> bool { (self.0 >> 3 & 0x1) != 0 }
        
    /// Sets state variable 03h

    pub fn set_STH_PGV03(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffb) | ((value as u64) & 0x1) << 2; }

    /// Gets state variable 03h
    pub fn get_STH_PGV03(&self) -> bool { (self.0 >> 2 & 0x1) != 0 }
        
    /// Sets state variable 02h

    pub fn set_STH_PGV02(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffd) | ((value as u64) & 0x1) << 1; }

    /// Gets state variable 02h
    pub fn get_STH_PGV02(&self) -> bool { (self.0 >> 1 & 0x1) != 0 }
        
    /// Sets state variable 01h

    pub fn set_STH_PGV01(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffe) | ((value as u64) & 0x1) << 0; }

    /// Gets state variable 01h
    pub fn get_STH_PGV01(&self) -> bool { (self.0 >> 0 & 0x1) != 0 }
        
}