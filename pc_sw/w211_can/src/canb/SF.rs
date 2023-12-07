
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'SF'
*/
    
pub const SF_A1_CAN_ID: u16 = 0x01AC;
pub const SF_A2_CAN_ID: u16 = 0x02D0;
pub const SD_RS_SF_CAN_ID: u16 = 0x07CC;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SF_A1(pub u64);

impl SF_A1 {

	/// Gets CAN ID of SF_A1
	pub const fn get_canid() -> u16 { SF_A1_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Driver's seat backrest unlocked

    pub fn set_LE_F_ENT(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Driver's seat backrest unlocked
    pub fn get_LE_F_ENT(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Driver seat position longitudinal adjustment. Conversion formula (To raw from real): y=(x-0.0)/1.00 (Unit: Abschnitte)

    pub fn set_SF_POS_LV(&mut self, value: u8){ self.0 = (self.0 & 0xff00ffffffffffff) | ((value as u64) & 0xff) << 48; }

    /// Gets Driver seat position longitudinal adjustment. Conversion formula (To real from raw): y=(1.00x)+0.0 (Unit: Abschnitte)
    pub fn get_SF_POS_LV(&self) -> u8 { (self.0 >> 48 & 0xff) as u8 }
        
    /// Sets Driver seat position height adjustment. Conversion formula (To raw from real): y=(x-0.0)/1.00 (Unit: Abschnitte)

    pub fn set_SF_POS_HV(&mut self, value: u8){ self.0 = (self.0 & 0xffff00ffffffffff) | ((value as u64) & 0xff) << 40; }

    /// Gets Driver seat position height adjustment. Conversion formula (To real from raw): y=(1.00x)+0.0 (Unit: Abschnitte)
    pub fn get_SF_POS_HV(&self) -> u8 { (self.0 >> 40 & 0xff) as u8 }
        
    /// Sets Driver's seat position, backrest adjustment. Conversion formula (To raw from real): y=(x-0.0)/1.00 (Unit: Abschnitte)

    pub fn set_SF_POS_L(&mut self, value: u8){ self.0 = (self.0 & 0xffffff00ffffffff) | ((value as u64) & 0xff) << 32; }

    /// Gets Driver's seat position, backrest adjustment. Conversion formula (To real from raw): y=(1.00x)+0.0 (Unit: Abschnitte)
    pub fn get_SF_POS_L(&self) -> u8 { (self.0 >> 32 & 0xff) as u8 }
        
    /// Sets Driver seat position tilt adjustment. Conversion formula (To raw from real): y=(x-0.0)/1.00 (Unit: Abschnitte)

    pub fn set_SF_POS_NV(&mut self, value: u8){ self.0 = (self.0 & 0xffffffff00ffffff) | ((value as u64) & 0xff) << 24; }

    /// Gets Driver seat position tilt adjustment. Conversion formula (To real from raw): y=(1.00x)+0.0 (Unit: Abschnitte)
    pub fn get_SF_POS_NV(&self) -> u8 { (self.0 >> 24 & 0xff) as u8 }
        
    /// Sets Driver's seat position, headrest adjustment. Conversion formula (To raw from real): y=(x-0.0)/1.00 (Unit: Abschnitte)

    pub fn set_SF_POS_K(&mut self, value: u8){ self.0 = (self.0 & 0xffffffffff00ffff) | ((value as u64) & 0xff) << 16; }

    /// Gets Driver's seat position, headrest adjustment. Conversion formula (To real from raw): y=(1.00x)+0.0 (Unit: Abschnitte)
    pub fn get_SF_POS_K(&self) -> u8 { (self.0 >> 16 & 0xff) as u8 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SF_A2(pub u64);

impl SF_A2 {

	/// Gets CAN ID of SF_A2
	pub const fn get_canid() -> u16 { SF_A2_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Move to entry/exit position active

    pub fn set_ESH_AKT(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Move to entry/exit position active
    pub fn get_ESH_AKT(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Approach travel position active

    pub fn set_AUTO_AKT(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Approach travel position active
    pub fn get_AUTO_AKT(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets Save manually adjusted position

    pub fn set_MF_MAN_SP(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets Save manually adjusted position
    pub fn get_MF_MAN_SP(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Store manually set interior mirror position

    pub fn set_MF_MAN_ISP_SP(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Store manually set interior mirror position
    pub fn get_MF_MAN_ISP_SP(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Carry out positioning of interior mirror

    pub fn set_ESH_AUTO_ISP(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Carry out positioning of interior mirror
    pub fn get_ESH_AUTO_ISP(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets Perform mirror positioning

    pub fn set_ESH_AUTO_REST(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets Perform mirror positioning
    pub fn get_ESH_AUTO_REST(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets Driver memory button pressed

    pub fn set_MF_BET(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets Driver memory button pressed
    pub fn get_MF_BET(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets Memory driver - save position 3

    pub fn set_MF_P3_SP(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets Memory driver - save position 3
    pub fn get_MF_P3_SP(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets Memory driver - save position 2

    pub fn set_MF_P2_SP(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets Memory driver - save position 2
    pub fn get_MF_P2_SP(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets Memory driver - save position 1

    pub fn set_MF_P1_SP(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets Memory driver - save position 1
    pub fn get_MF_P1_SP(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets Memory driver - take position 3

    pub fn set_MF_P3_EN(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets Memory driver - take position 3
    pub fn get_MF_P3_EN(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets Memory driver - take position 2

    pub fn set_MF_P2_EN(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets Memory driver - take position 2
    pub fn get_MF_P2_EN(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets Memory driver - take position 1

    pub fn set_MF_P1_EN(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets Memory driver - take position 1
    pub fn get_MF_P1_EN(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SD_RS_SF(pub u64);

impl SD_RS_SF {

	/// Gets CAN ID of SD_RS_SF
	pub const fn get_canid() -> u16 { SD_RS_SF_CAN_ID }
	pub fn new(data: u64) -> Self { Self(data) }
    /// Sets Reserved for vector format designation BR211

    pub fn set_SF_RES(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Reserved for vector format designation BR211
    pub fn get_SF_RES(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets process variable 07h

    pub fn set_SF_PG07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets process variable 07h
    pub fn get_SF_PG07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets process variable 06h

    pub fn set_SF_PG06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets process variable 06h
    pub fn get_SF_PG06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets process variable 05h

    pub fn set_SF_PG05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets process variable 05h
    pub fn get_SF_PG05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets process variable 04h

    pub fn set_SF_PG04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets process variable 04h
    pub fn get_SF_PG04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets process variable 03h

    pub fn set_SF_PG03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets process variable 03h
    pub fn get_SF_PG03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets process variable 02h

    pub fn set_SF_PG02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets process variable 02h
    pub fn get_SF_PG02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets process variable 01h

    pub fn set_SF_PG01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets process variable 01h
    pub fn get_SF_PG01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets Process variable 0Fh

    pub fn set_SF_PG0F(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets Process variable 0Fh
    pub fn get_SF_PG0F(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets process variable 0Eh

    pub fn set_SF_PG0E(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets process variable 0Eh
    pub fn get_SF_PG0E(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets process variable 0Dh

    pub fn set_SF_PG0D(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets process variable 0Dh
    pub fn get_SF_PG0D(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets process variable 0Ch

    pub fn set_SF_PG0C(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets process variable 0Ch
    pub fn get_SF_PG0C(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets process variable 0Bh

    pub fn set_SF_PG0B(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets process variable 0Bh
    pub fn get_SF_PG0B(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets process variable 0Ah

    pub fn set_SF_PG0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets process variable 0Ah
    pub fn get_SF_PG0A(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets process variable 09h

    pub fn set_SF_PG09(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets process variable 09h
    pub fn get_SF_PG09(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets process variable 08h

    pub fn set_SF_PG08(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets process variable 08h
    pub fn get_SF_PG08(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets process variable 17h

    pub fn set_SF_PG17(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets process variable 17h
    pub fn get_SF_PG17(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets process variable 16h

    pub fn set_SF_PG16(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets process variable 16h
    pub fn get_SF_PG16(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets process variable 15h

    pub fn set_SF_PG15(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets process variable 15h
    pub fn get_SF_PG15(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets process variable 14h

    pub fn set_SF_PG14(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets process variable 14h
    pub fn get_SF_PG14(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets process variable 13h

    pub fn set_SF_PG13(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets process variable 13h
    pub fn get_SF_PG13(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets process variable 12h

    pub fn set_SF_PG12(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets process variable 12h
    pub fn get_SF_PG12(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets process variable 11h

    pub fn set_SF_PG11(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets process variable 11h
    pub fn get_SF_PG11(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets process variable 10h

    pub fn set_SF_PG10(&mut self, value: bool){ self.0 = (self.0 & 0xfffffeffffffffff) | ((value as u64) & 0x1) << 40; }

    /// Gets process variable 10h
    pub fn get_SF_PG10(&self) -> bool { (self.0 >> 40 & 0x1) != 0 }
        
    /// Sets process variable 1Fh

    pub fn set_SF_PG1F(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets process variable 1Fh
    pub fn get_SF_PG1F(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets process variable 1Eh

    pub fn set_SF_PG1E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets process variable 1Eh
    pub fn get_SF_PG1E(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets process variable 1Dh

    pub fn set_SF_PG1D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets process variable 1Dh
    pub fn get_SF_PG1D(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets process size 1Ch

    pub fn set_SF_PG1C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets process size 1Ch
    pub fn get_SF_PG1C(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
    /// Sets process variable 1Bh

    pub fn set_SF_PG1B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffff7ffffffff) | ((value as u64) & 0x1) << 35; }

    /// Gets process variable 1Bh
    pub fn get_SF_PG1B(&self) -> bool { (self.0 >> 35 & 0x1) != 0 }
        
    /// Sets process variable 1Ah

    pub fn set_SF_PG1A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffbffffffff) | ((value as u64) & 0x1) << 34; }

    /// Gets process variable 1Ah
    pub fn get_SF_PG1A(&self) -> bool { (self.0 >> 34 & 0x1) != 0 }
        
    /// Sets process variable 19h

    pub fn set_SF_PG19(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffdffffffff) | ((value as u64) & 0x1) << 33; }

    /// Gets process variable 19h
    pub fn get_SF_PG19(&self) -> bool { (self.0 >> 33 & 0x1) != 0 }
        
    /// Sets process size 18h

    pub fn set_SF_PG18(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffeffffffff) | ((value as u64) & 0x1) << 32; }

    /// Gets process size 18h
    pub fn get_SF_PG18(&self) -> bool { (self.0 >> 32 & 0x1) != 0 }
        
    /// Sets process variable 27h

    pub fn set_SF_PG27(&mut self, value: bool){ self.0 = (self.0 & 0xffffffff7fffffff) | ((value as u64) & 0x1) << 31; }

    /// Gets process variable 27h
    pub fn get_SF_PG27(&self) -> bool { (self.0 >> 31 & 0x1) != 0 }
        
    /// Sets process variable 26h

    pub fn set_SF_PG26(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffbfffffff) | ((value as u64) & 0x1) << 30; }

    /// Gets process variable 26h
    pub fn get_SF_PG26(&self) -> bool { (self.0 >> 30 & 0x1) != 0 }
        
    /// Sets process variable 25h

    pub fn set_SF_PG25(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffdfffffff) | ((value as u64) & 0x1) << 29; }

    /// Gets process variable 25h
    pub fn get_SF_PG25(&self) -> bool { (self.0 >> 29 & 0x1) != 0 }
        
    /// Sets process variable 24h

    pub fn set_SF_PG24(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffefffffff) | ((value as u64) & 0x1) << 28; }

    /// Gets process variable 24h
    pub fn get_SF_PG24(&self) -> bool { (self.0 >> 28 & 0x1) != 0 }
        
    /// Sets process variable 23h

    pub fn set_SF_PG23(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffff7ffffff) | ((value as u64) & 0x1) << 27; }

    /// Gets process variable 23h
    pub fn get_SF_PG23(&self) -> bool { (self.0 >> 27 & 0x1) != 0 }
        
    /// Sets process variable 22h

    pub fn set_SF_PG22(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffbffffff) | ((value as u64) & 0x1) << 26; }

    /// Gets process variable 22h
    pub fn get_SF_PG22(&self) -> bool { (self.0 >> 26 & 0x1) != 0 }
        
    /// Sets process variable 21h

    pub fn set_SF_PG21(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffdffffff) | ((value as u64) & 0x1) << 25; }

    /// Gets process variable 21h
    pub fn get_SF_PG21(&self) -> bool { (self.0 >> 25 & 0x1) != 0 }
        
    /// Sets process variable 20h

    pub fn set_SF_PG20(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffeffffff) | ((value as u64) & 0x1) << 24; }

    /// Gets process variable 20h
    pub fn get_SF_PG20(&self) -> bool { (self.0 >> 24 & 0x1) != 0 }
        
    /// Sets process variable 2Fh

    pub fn set_SF_PG2F(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffff7fffff) | ((value as u64) & 0x1) << 23; }

    /// Gets process variable 2Fh
    pub fn get_SF_PG2F(&self) -> bool { (self.0 >> 23 & 0x1) != 0 }
        
    /// Sets process variable 2Eh

    pub fn set_SF_PG2E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffbfffff) | ((value as u64) & 0x1) << 22; }

    /// Gets process variable 2Eh
    pub fn get_SF_PG2E(&self) -> bool { (self.0 >> 22 & 0x1) != 0 }
        
    /// Sets process variable 2Dh

    pub fn set_SF_PG2D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffdfffff) | ((value as u64) & 0x1) << 21; }

    /// Gets process variable 2Dh
    pub fn get_SF_PG2D(&self) -> bool { (self.0 >> 21 & 0x1) != 0 }
        
    /// Sets process size 2Ch

    pub fn set_SF_PG2C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffefffff) | ((value as u64) & 0x1) << 20; }

    /// Gets process size 2Ch
    pub fn get_SF_PG2C(&self) -> bool { (self.0 >> 20 & 0x1) != 0 }
        
    /// Sets Process variable 2Bh

    pub fn set_SF_PG2B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffff7ffff) | ((value as u64) & 0x1) << 19; }

    /// Gets Process variable 2Bh
    pub fn get_SF_PG2B(&self) -> bool { (self.0 >> 19 & 0x1) != 0 }
        
    /// Sets process variable 2Ah

    pub fn set_SF_PG2A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffbffff) | ((value as u64) & 0x1) << 18; }

    /// Gets process variable 2Ah
    pub fn get_SF_PG2A(&self) -> bool { (self.0 >> 18 & 0x1) != 0 }
        
    /// Sets process variable 29h

    pub fn set_SF_PG29(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffdffff) | ((value as u64) & 0x1) << 17; }

    /// Gets process variable 29h
    pub fn get_SF_PG29(&self) -> bool { (self.0 >> 17 & 0x1) != 0 }
        
    /// Sets Process variable 28h

    pub fn set_SF_PG28(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffeffff) | ((value as u64) & 0x1) << 16; }

    /// Gets Process variable 28h
    pub fn get_SF_PG28(&self) -> bool { (self.0 >> 16 & 0x1) != 0 }
        
    /// Sets process variable 37h

    pub fn set_SF_PG37(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffff7fff) | ((value as u64) & 0x1) << 15; }

    /// Gets process variable 37h
    pub fn get_SF_PG37(&self) -> bool { (self.0 >> 15 & 0x1) != 0 }
        
    /// Sets process variable 36h

    pub fn set_SF_PG36(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffbfff) | ((value as u64) & 0x1) << 14; }

    /// Gets process variable 36h
    pub fn get_SF_PG36(&self) -> bool { (self.0 >> 14 & 0x1) != 0 }
        
    /// Sets process variable 35h

    pub fn set_SF_PG35(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffdfff) | ((value as u64) & 0x1) << 13; }

    /// Gets process variable 35h
    pub fn get_SF_PG35(&self) -> bool { (self.0 >> 13 & 0x1) != 0 }
        
    /// Sets process variable 34h

    pub fn set_SF_PG34(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffefff) | ((value as u64) & 0x1) << 12; }

    /// Gets process variable 34h
    pub fn get_SF_PG34(&self) -> bool { (self.0 >> 12 & 0x1) != 0 }
        
    /// Sets process variable 33h

    pub fn set_SF_PG33(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffff7ff) | ((value as u64) & 0x1) << 11; }

    /// Gets process variable 33h
    pub fn get_SF_PG33(&self) -> bool { (self.0 >> 11 & 0x1) != 0 }
        
    /// Sets process variable 32h

    pub fn set_SF_PG32(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffbff) | ((value as u64) & 0x1) << 10; }

    /// Gets process variable 32h
    pub fn get_SF_PG32(&self) -> bool { (self.0 >> 10 & 0x1) != 0 }
        
    /// Sets process variable 31h

    pub fn set_SF_PG31(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffdff) | ((value as u64) & 0x1) << 9; }

    /// Gets process variable 31h
    pub fn get_SF_PG31(&self) -> bool { (self.0 >> 9 & 0x1) != 0 }
        
    /// Sets process variable 30h

    pub fn set_SF_PG30(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffeff) | ((value as u64) & 0x1) << 8; }

    /// Gets process variable 30h
    pub fn get_SF_PG30(&self) -> bool { (self.0 >> 8 & 0x1) != 0 }
        
    /// Sets process variable 3Fh

    pub fn set_SF_PG3F(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffff7f) | ((value as u64) & 0x1) << 7; }

    /// Gets process variable 3Fh
    pub fn get_SF_PG3F(&self) -> bool { (self.0 >> 7 & 0x1) != 0 }
        
    /// Sets process variable 3Eh

    pub fn set_SF_PG3E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffbf) | ((value as u64) & 0x1) << 6; }

    /// Gets process variable 3Eh
    pub fn get_SF_PG3E(&self) -> bool { (self.0 >> 6 & 0x1) != 0 }
        
    /// Sets process variable 3Dh

    pub fn set_SF_PG3D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffdf) | ((value as u64) & 0x1) << 5; }

    /// Gets process variable 3Dh
    pub fn get_SF_PG3D(&self) -> bool { (self.0 >> 5 & 0x1) != 0 }
        
    /// Sets process size 3Ch

    pub fn set_SF_PG3C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffffffffef) | ((value as u64) & 0x1) << 4; }

    /// Gets process size 3Ch
    pub fn get_SF_PG3C(&self) -> bool { (self.0 >> 4 & 0x1) != 0 }
        
    /// Sets process variable 3Bh

    pub fn set_SF_PG3B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffff7) | ((value as u64) & 0x1) << 3; }

    /// Gets process variable 3Bh
    pub fn get_SF_PG3B(&self) -> bool { (self.0 >> 3 & 0x1) != 0 }
        
    /// Sets process variable 3Ah

    pub fn set_SF_PG3A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffb) | ((value as u64) & 0x1) << 2; }

    /// Gets process variable 3Ah
    pub fn get_SF_PG3A(&self) -> bool { (self.0 >> 2 & 0x1) != 0 }
        
    /// Sets process variable 39h

    pub fn set_SF_PG39(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffd) | ((value as u64) & 0x1) << 1; }

    /// Gets process variable 39h
    pub fn get_SF_PG39(&self) -> bool { (self.0 >> 1 & 0x1) != 0 }
        
    /// Sets Process variable 38h

    pub fn set_SF_PG38(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffffffffe) | ((value as u64) & 0x1); }

    /// Gets Process variable 38h
    pub fn get_SF_PG38(&self) -> bool { (self.0 & 0x1) != 0 }
        
}