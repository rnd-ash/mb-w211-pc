
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'LRK'
*/
    
pub const LRK_A1_CAN_ID: u16 = 0x0288;
pub const SD_RS_LRK_CAN_ID: u16 = 0x07CF;


pub struct LRK_A1(u64);

impl LRK_A1 {

	/// Gets CAN ID of LRK_A1
	pub fn get_canid() -> u16 { LRK_A1_CAN_ID }
    /// Sets Switch on LED steering wheel ventilation

    pub fn set_LLFT_LED_EIN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Switch on LED steering wheel ventilation
    pub fn get_LLFT_LED_EIN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets Switch on LED steering wheel heating

    pub fn set_LHZG_LED_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets Switch on LED steering wheel heating
    pub fn get_LHZG_LED_EIN(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets LEDs LRK flash due to Disturbance

    pub fn set_LRK_STOERG(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets LEDs LRK flash due to Disturbance
    pub fn get_LRK_STOERG(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets Switch on the steering wheel ventilation

    pub fn set_LLFT_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets Switch on the steering wheel ventilation
    pub fn get_LLFT_EIN(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets Switch on steering wheel heating

    pub fn set_LHZG_EIN(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets Switch on steering wheel heating
    pub fn get_LHZG_EIN(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
}
pub struct SD_RS_LRK(u64);

impl SD_RS_LRK {

	/// Gets CAN ID of SD_RS_LRK
	pub fn get_canid() -> u16 { SD_RS_LRK_CAN_ID }
    /// Sets Identification for > 8 bytes

    pub fn set_LRK_KENN(&mut self, value: bool){ self.0 = (self.0 & 0x7fffffffffffffff) | ((value as u64) & 0x1) << 63; }

    /// Gets Identification for > 8 bytes
    pub fn get_LRK_KENN(&self) -> bool { (self.0 >> 63 & 0x1) != 0 }
        
    /// Sets error vector 07h

    pub fn set_LRK_FV07(&mut self, value: bool){ self.0 = (self.0 & 0xbfffffffffffffff) | ((value as u64) & 0x1) << 62; }

    /// Gets error vector 07h
    pub fn get_LRK_FV07(&self) -> bool { (self.0 >> 62 & 0x1) != 0 }
        
    /// Sets error vector 06h

    pub fn set_LRK_FV06(&mut self, value: bool){ self.0 = (self.0 & 0xdfffffffffffffff) | ((value as u64) & 0x1) << 61; }

    /// Gets error vector 06h
    pub fn get_LRK_FV06(&self) -> bool { (self.0 >> 61 & 0x1) != 0 }
        
    /// Sets error vector 05h

    pub fn set_LRK_FV05(&mut self, value: bool){ self.0 = (self.0 & 0xefffffffffffffff) | ((value as u64) & 0x1) << 60; }

    /// Gets error vector 05h
    pub fn get_LRK_FV05(&self) -> bool { (self.0 >> 60 & 0x1) != 0 }
        
    /// Sets error vector 04h

    pub fn set_LRK_FV04(&mut self, value: bool){ self.0 = (self.0 & 0xf7ffffffffffffff) | ((value as u64) & 0x1) << 59; }

    /// Gets error vector 04h
    pub fn get_LRK_FV04(&self) -> bool { (self.0 >> 59 & 0x1) != 0 }
        
    /// Sets error vector 03h

    pub fn set_LRK_FV03(&mut self, value: bool){ self.0 = (self.0 & 0xfbffffffffffffff) | ((value as u64) & 0x1) << 58; }

    /// Gets error vector 03h
    pub fn get_LRK_FV03(&self) -> bool { (self.0 >> 58 & 0x1) != 0 }
        
    /// Sets error vector 02h

    pub fn set_LRK_FV02(&mut self, value: bool){ self.0 = (self.0 & 0xfdffffffffffffff) | ((value as u64) & 0x1) << 57; }

    /// Gets error vector 02h
    pub fn get_LRK_FV02(&self) -> bool { (self.0 >> 57 & 0x1) != 0 }
        
    /// Sets error vector 01h

    pub fn set_LRK_FV01(&mut self, value: bool){ self.0 = (self.0 & 0xfeffffffffffffff) | ((value as u64) & 0x1) << 56; }

    /// Gets error vector 01h
    pub fn get_LRK_FV01(&self) -> bool { (self.0 >> 56 & 0x1) != 0 }
        
    /// Sets error vector 0Fh

    pub fn set_LRK_FV0F(&mut self, value: bool){ self.0 = (self.0 & 0xff7fffffffffffff) | ((value as u64) & 0x1) << 55; }

    /// Gets error vector 0Fh
    pub fn get_LRK_FV0F(&self) -> bool { (self.0 >> 55 & 0x1) != 0 }
        
    /// Sets error vector 0Eh

    pub fn set_LRK_FV0E(&mut self, value: bool){ self.0 = (self.0 & 0xffbfffffffffffff) | ((value as u64) & 0x1) << 54; }

    /// Gets error vector 0Eh
    pub fn get_LRK_FV0E(&self) -> bool { (self.0 >> 54 & 0x1) != 0 }
        
    /// Sets error vector 0Dh

    pub fn set_LRK_FV0D(&mut self, value: bool){ self.0 = (self.0 & 0xffdfffffffffffff) | ((value as u64) & 0x1) << 53; }

    /// Gets error vector 0Dh
    pub fn get_LRK_FV0D(&self) -> bool { (self.0 >> 53 & 0x1) != 0 }
        
    /// Sets error vector 0Ch

    pub fn set_LRK_FV0C(&mut self, value: bool){ self.0 = (self.0 & 0xffefffffffffffff) | ((value as u64) & 0x1) << 52; }

    /// Gets error vector 0Ch
    pub fn get_LRK_FV0C(&self) -> bool { (self.0 >> 52 & 0x1) != 0 }
        
    /// Sets error vector 0Bh

    pub fn set_LRK_FV0B(&mut self, value: bool){ self.0 = (self.0 & 0xfff7ffffffffffff) | ((value as u64) & 0x1) << 51; }

    /// Gets error vector 0Bh
    pub fn get_LRK_FV0B(&self) -> bool { (self.0 >> 51 & 0x1) != 0 }
        
    /// Sets error vector 0Ah

    pub fn set_LRK_FV0A(&mut self, value: bool){ self.0 = (self.0 & 0xfffbffffffffffff) | ((value as u64) & 0x1) << 50; }

    /// Gets error vector 0Ah
    pub fn get_LRK_FV0A(&self) -> bool { (self.0 >> 50 & 0x1) != 0 }
        
    /// Sets error vector 09h

    pub fn set_LRK_FV09(&mut self, value: bool){ self.0 = (self.0 & 0xfffdffffffffffff) | ((value as u64) & 0x1) << 49; }

    /// Gets error vector 09h
    pub fn get_LRK_FV09(&self) -> bool { (self.0 >> 49 & 0x1) != 0 }
        
    /// Sets error vector 08h

    pub fn set_LRK_FV08(&mut self, value: bool){ self.0 = (self.0 & 0xfffeffffffffffff) | ((value as u64) & 0x1) << 48; }

    /// Gets error vector 08h
    pub fn get_LRK_FV08(&self) -> bool { (self.0 >> 48 & 0x1) != 0 }
        
    /// Sets error vector 17h

    pub fn set_LRK_FV17(&mut self, value: bool){ self.0 = (self.0 & 0xffff7fffffffffff) | ((value as u64) & 0x1) << 47; }

    /// Gets error vector 17h
    pub fn get_LRK_FV17(&self) -> bool { (self.0 >> 47 & 0x1) != 0 }
        
    /// Sets error vector 16h

    pub fn set_LRK_FV16(&mut self, value: bool){ self.0 = (self.0 & 0xffffbfffffffffff) | ((value as u64) & 0x1) << 46; }

    /// Gets error vector 16h
    pub fn get_LRK_FV16(&self) -> bool { (self.0 >> 46 & 0x1) != 0 }
        
    /// Sets error vector 15h

    pub fn set_LRK_FV15(&mut self, value: bool){ self.0 = (self.0 & 0xffffdfffffffffff) | ((value as u64) & 0x1) << 45; }

    /// Gets error vector 15h
    pub fn get_LRK_FV15(&self) -> bool { (self.0 >> 45 & 0x1) != 0 }
        
    /// Sets error vector 14h

    pub fn set_LRK_FV14(&mut self, value: bool){ self.0 = (self.0 & 0xffffefffffffffff) | ((value as u64) & 0x1) << 44; }

    /// Gets error vector 14h
    pub fn get_LRK_FV14(&self) -> bool { (self.0 >> 44 & 0x1) != 0 }
        
    /// Sets error vector 13h

    pub fn set_LRK_FV13(&mut self, value: bool){ self.0 = (self.0 & 0xfffff7ffffffffff) | ((value as u64) & 0x1) << 43; }

    /// Gets error vector 13h
    pub fn get_LRK_FV13(&self) -> bool { (self.0 >> 43 & 0x1) != 0 }
        
    /// Sets error vector 12h

    pub fn set_LRK_FV12(&mut self, value: bool){ self.0 = (self.0 & 0xfffffbffffffffff) | ((value as u64) & 0x1) << 42; }

    /// Gets error vector 12h
    pub fn get_LRK_FV12(&self) -> bool { (self.0 >> 42 & 0x1) != 0 }
        
    /// Sets error vector 11h

    pub fn set_LRK_FV11(&mut self, value: bool){ self.0 = (self.0 & 0xfffffdffffffffff) | ((value as u64) & 0x1) << 41; }

    /// Gets error vector 11h
    pub fn get_LRK_FV11(&self) -> bool { (self.0 >> 41 & 0x1) != 0 }
        
    /// Sets error vector 10h

    pub fn set_LRK_FV10(&mut self, value: bool){ self.0 = (self.0 & 0xfffffeffffffffff) | ((value as u64) & 0x1) << 40; }

    /// Gets error vector 10h
    pub fn get_LRK_FV10(&self) -> bool { (self.0 >> 40 & 0x1) != 0 }
        
    /// Sets error vector 1Fh

    pub fn set_LRK_FV1F(&mut self, value: bool){ self.0 = (self.0 & 0xffffff7fffffffff) | ((value as u64) & 0x1) << 39; }

    /// Gets error vector 1Fh
    pub fn get_LRK_FV1F(&self) -> bool { (self.0 >> 39 & 0x1) != 0 }
        
    /// Sets error vector 1Eh

    pub fn set_LRK_FV1E(&mut self, value: bool){ self.0 = (self.0 & 0xffffffbfffffffff) | ((value as u64) & 0x1) << 38; }

    /// Gets error vector 1Eh
    pub fn get_LRK_FV1E(&self) -> bool { (self.0 >> 38 & 0x1) != 0 }
        
    /// Sets error vector 1Dh

    pub fn set_LRK_FV1D(&mut self, value: bool){ self.0 = (self.0 & 0xffffffdfffffffff) | ((value as u64) & 0x1) << 37; }

    /// Gets error vector 1Dh
    pub fn get_LRK_FV1D(&self) -> bool { (self.0 >> 37 & 0x1) != 0 }
        
    /// Sets Error vector 1Ch

    pub fn set_LRK_FV1C(&mut self, value: bool){ self.0 = (self.0 & 0xffffffefffffffff) | ((value as u64) & 0x1) << 36; }

    /// Gets Error vector 1Ch
    pub fn get_LRK_FV1C(&self) -> bool { (self.0 >> 36 & 0x1) != 0 }
        
    /// Sets error vector 1Bh

    pub fn set_LRK_FV1B(&mut self, value: bool){ self.0 = (self.0 & 0xfffffff7ffffffff) | ((value as u64) & 0x1) << 35; }

    /// Gets error vector 1Bh
    pub fn get_LRK_FV1B(&self) -> bool { (self.0 >> 35 & 0x1) != 0 }
        
    /// Sets Error vector 1Ah

    pub fn set_LRK_FV1A(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffbffffffff) | ((value as u64) & 0x1) << 34; }

    /// Gets Error vector 1Ah
    pub fn get_LRK_FV1A(&self) -> bool { (self.0 >> 34 & 0x1) != 0 }
        
    /// Sets error vector 19h

    pub fn set_LRK_FV19(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffdffffffff) | ((value as u64) & 0x1) << 33; }

    /// Gets error vector 19h
    pub fn get_LRK_FV19(&self) -> bool { (self.0 >> 33 & 0x1) != 0 }
        
    /// Sets error vector 18h

    pub fn set_LRK_FV18(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffeffffffff) | ((value as u64) & 0x1) << 32; }

    /// Gets error vector 18h
    pub fn get_LRK_FV18(&self) -> bool { (self.0 >> 32 & 0x1) != 0 }
        
    /// Sets state variable 08h

    pub fn set_LRK_PGV08(&mut self, value: bool){ self.0 = (self.0 & 0xffffffff7fffffff) | ((value as u64) & 0x1) << 31; }

    /// Gets state variable 08h
    pub fn get_LRK_PGV08(&self) -> bool { (self.0 >> 31 & 0x1) != 0 }
        
    /// Sets state variable 07h

    pub fn set_LRK_PGV07(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffbfffffff) | ((value as u64) & 0x1) << 30; }

    /// Gets state variable 07h
    pub fn get_LRK_PGV07(&self) -> bool { (self.0 >> 30 & 0x1) != 0 }
        
    /// Sets state variable 06h

    pub fn set_LRK_PGV06(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffdfffffff) | ((value as u64) & 0x1) << 29; }

    /// Gets state variable 06h
    pub fn get_LRK_PGV06(&self) -> bool { (self.0 >> 29 & 0x1) != 0 }
        
    /// Sets state variable 05h

    pub fn set_LRK_PGV05(&mut self, value: bool){ self.0 = (self.0 & 0xffffffffefffffff) | ((value as u64) & 0x1) << 28; }

    /// Gets state variable 05h
    pub fn get_LRK_PGV05(&self) -> bool { (self.0 >> 28 & 0x1) != 0 }
        
    /// Sets state variable 04h

    pub fn set_LRK_PGV04(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffff7ffffff) | ((value as u64) & 0x1) << 27; }

    /// Gets state variable 04h
    pub fn get_LRK_PGV04(&self) -> bool { (self.0 >> 27 & 0x1) != 0 }
        
    /// Sets state variable 03h

    pub fn set_LRK_PGV03(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffbffffff) | ((value as u64) & 0x1) << 26; }

    /// Gets state variable 03h
    pub fn get_LRK_PGV03(&self) -> bool { (self.0 >> 26 & 0x1) != 0 }
        
    /// Sets state variable 02h

    pub fn set_LRK_PGV02(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffdffffff) | ((value as u64) & 0x1) << 25; }

    /// Gets state variable 02h
    pub fn get_LRK_PGV02(&self) -> bool { (self.0 >> 25 & 0x1) != 0 }
        
    /// Sets state variable 01h

    pub fn set_LRK_PGV01(&mut self, value: bool){ self.0 = (self.0 & 0xfffffffffeffffff) | ((value as u64) & 0x1) << 24; }

    /// Gets state variable 01h
    pub fn get_LRK_PGV01(&self) -> bool { (self.0 >> 24 & 0x1) != 0 }
        
}