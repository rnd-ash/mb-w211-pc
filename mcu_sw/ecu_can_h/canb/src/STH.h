
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'STH'
*/

#ifndef __ECU_STH_H_
#define __ECU_STH_H_

#include <stdint.h>
    
#define STH_A1_CAN_ID 0x0094
#define SD_RS_STH_CAN_ID 0x07D9



typedef union {
	uint64_t raw;
	uint8_t bytes[8];

	/** Gets CAN ID of STH_A1 */
	uint32_t get_canid(){ return STH_A1_CAN_ID; }
    /** Sets Switch on auxiliary heating/ventilation */
    void set_STHL_EIN(bool value){ raw = (raw & 0x7fffffffffffffff) | ((uint64_t)value & 0x1) << 63; }

    /** Gets Switch on auxiliary heating/ventilation */
    bool get_STHL_EIN() const { return (bool)(raw >> 63 & 0x1); }
        
    /** Sets Switch off auxiliary heating/ventilation */
    void set_STHL_AUS(bool value){ raw = (raw & 0xbfffffffffffffff) | ((uint64_t)value & 0x1) << 62; }

    /** Gets Switch off auxiliary heating/ventilation */
    bool get_STHL_AUS() const { return (bool)(raw >> 62 & 0x1); }
        
    /** Sets Turn on vehicle fan */
    void set_GEBLAESE_EIN(bool value){ raw = (raw & 0xf7ffffffffffffff) | ((uint64_t)value & 0x1) << 59; }

    /** Gets Turn on vehicle fan */
    bool get_GEBLAESE_EIN() const { return (bool)(raw >> 59 & 0x1); }
        
    /** Sets Open preset time menu */
    void set_VWZ_MENUE(bool value){ raw = (raw & 0xfbffffffffffffff) | ((uint64_t)value & 0x1) << 58; }

    /** Gets Open preset time menu */
    bool get_VWZ_MENUE() const { return (bool)(raw >> 58 & 0x1); }
        
    /** Sets Transmitter learning mode on */
    void set_SENDLM_EIN(bool value){ raw = (raw & 0xfdffffffffffffff) | ((uint64_t)value & 0x1) << 57; }

    /** Gets Transmitter learning mode on */
    bool get_SENDLM_EIN() const { return (bool)(raw >> 57 & 0x1); }
        
} STH_A1;



typedef union {
	uint64_t raw;
	uint8_t bytes[8];

	/** Gets CAN ID of SD_RS_STH */
	uint32_t get_canid(){ return SD_RS_STH_CAN_ID; }
    /** Sets Identification for > 8 bytes */
    void set_STH_KENN(bool value){ raw = (raw & 0x7fffffffffffffff) | ((uint64_t)value & 0x1) << 63; }

    /** Gets Identification for > 8 bytes */
    bool get_STH_KENN() const { return (bool)(raw >> 63 & 0x1); }
        
    /** Sets error vector 07h */
    void set_STH_FV07(bool value){ raw = (raw & 0xbfffffffffffffff) | ((uint64_t)value & 0x1) << 62; }

    /** Gets error vector 07h */
    bool get_STH_FV07() const { return (bool)(raw >> 62 & 0x1); }
        
    /** Sets error vector 06h */
    void set_STH_FV06(bool value){ raw = (raw & 0xdfffffffffffffff) | ((uint64_t)value & 0x1) << 61; }

    /** Gets error vector 06h */
    bool get_STH_FV06() const { return (bool)(raw >> 61 & 0x1); }
        
    /** Sets error vector 05h */
    void set_STH_FV05(bool value){ raw = (raw & 0xefffffffffffffff) | ((uint64_t)value & 0x1) << 60; }

    /** Gets error vector 05h */
    bool get_STH_FV05() const { return (bool)(raw >> 60 & 0x1); }
        
    /** Sets error vector 04h */
    void set_STH_FV04(bool value){ raw = (raw & 0xf7ffffffffffffff) | ((uint64_t)value & 0x1) << 59; }

    /** Gets error vector 04h */
    bool get_STH_FV04() const { return (bool)(raw >> 59 & 0x1); }
        
    /** Sets error vector 03h */
    void set_STH_FV03(bool value){ raw = (raw & 0xfbffffffffffffff) | ((uint64_t)value & 0x1) << 58; }

    /** Gets error vector 03h */
    bool get_STH_FV03() const { return (bool)(raw >> 58 & 0x1); }
        
    /** Sets error vector 02h */
    void set_STH_FV02(bool value){ raw = (raw & 0xfdffffffffffffff) | ((uint64_t)value & 0x1) << 57; }

    /** Gets error vector 02h */
    bool get_STH_FV02() const { return (bool)(raw >> 57 & 0x1); }
        
    /** Sets error vector 01h */
    void set_STH_FV01(bool value){ raw = (raw & 0xfeffffffffffffff) | ((uint64_t)value & 0x1) << 56; }

    /** Gets error vector 01h */
    bool get_STH_FV01() const { return (bool)(raw >> 56 & 0x1); }
        
    /** Sets error vector 0Fh */
    void set_STH_FV0F(bool value){ raw = (raw & 0xff7fffffffffffff) | ((uint64_t)value & 0x1) << 55; }

    /** Gets error vector 0Fh */
    bool get_STH_FV0F() const { return (bool)(raw >> 55 & 0x1); }
        
    /** Sets error vector 0Eh */
    void set_STH_FV0E(bool value){ raw = (raw & 0xffbfffffffffffff) | ((uint64_t)value & 0x1) << 54; }

    /** Gets error vector 0Eh */
    bool get_STH_FV0E() const { return (bool)(raw >> 54 & 0x1); }
        
    /** Sets error vector 0Dh */
    void set_STH_FV0D(bool value){ raw = (raw & 0xffdfffffffffffff) | ((uint64_t)value & 0x1) << 53; }

    /** Gets error vector 0Dh */
    bool get_STH_FV0D() const { return (bool)(raw >> 53 & 0x1); }
        
    /** Sets error vector 0Ch */
    void set_STH_FV0C(bool value){ raw = (raw & 0xffefffffffffffff) | ((uint64_t)value & 0x1) << 52; }

    /** Gets error vector 0Ch */
    bool get_STH_FV0C() const { return (bool)(raw >> 52 & 0x1); }
        
    /** Sets error vector 0Bh */
    void set_STH_FV0B(bool value){ raw = (raw & 0xfff7ffffffffffff) | ((uint64_t)value & 0x1) << 51; }

    /** Gets error vector 0Bh */
    bool get_STH_FV0B() const { return (bool)(raw >> 51 & 0x1); }
        
    /** Sets error vector 0Ah */
    void set_STH_FV0A(bool value){ raw = (raw & 0xfffbffffffffffff) | ((uint64_t)value & 0x1) << 50; }

    /** Gets error vector 0Ah */
    bool get_STH_FV0A() const { return (bool)(raw >> 50 & 0x1); }
        
    /** Sets error vector 09h */
    void set_STH_FV09(bool value){ raw = (raw & 0xfffdffffffffffff) | ((uint64_t)value & 0x1) << 49; }

    /** Gets error vector 09h */
    bool get_STH_FV09() const { return (bool)(raw >> 49 & 0x1); }
        
    /** Sets error vector 08h */
    void set_STH_FV08(bool value){ raw = (raw & 0xfffeffffffffffff) | ((uint64_t)value & 0x1) << 48; }

    /** Gets error vector 08h */
    bool get_STH_FV08() const { return (bool)(raw >> 48 & 0x1); }
        
    /** Sets error vector 17h */
    void set_STH_FV17(bool value){ raw = (raw & 0xffff7fffffffffff) | ((uint64_t)value & 0x1) << 47; }

    /** Gets error vector 17h */
    bool get_STH_FV17() const { return (bool)(raw >> 47 & 0x1); }
        
    /** Sets error vector 16h */
    void set_STH_FV16(bool value){ raw = (raw & 0xffffbfffffffffff) | ((uint64_t)value & 0x1) << 46; }

    /** Gets error vector 16h */
    bool get_STH_FV16() const { return (bool)(raw >> 46 & 0x1); }
        
    /** Sets error vector 15h */
    void set_STH_FV15(bool value){ raw = (raw & 0xffffdfffffffffff) | ((uint64_t)value & 0x1) << 45; }

    /** Gets error vector 15h */
    bool get_STH_FV15() const { return (bool)(raw >> 45 & 0x1); }
        
    /** Sets error vector 14h */
    void set_STH_FV14(bool value){ raw = (raw & 0xffffefffffffffff) | ((uint64_t)value & 0x1) << 44; }

    /** Gets error vector 14h */
    bool get_STH_FV14() const { return (bool)(raw >> 44 & 0x1); }
        
    /** Sets error vector 13h */
    void set_STH_FV13(bool value){ raw = (raw & 0xfffff7ffffffffff) | ((uint64_t)value & 0x1) << 43; }

    /** Gets error vector 13h */
    bool get_STH_FV13() const { return (bool)(raw >> 43 & 0x1); }
        
    /** Sets error vector 12h */
    void set_STH_FV12(bool value){ raw = (raw & 0xfffffbffffffffff) | ((uint64_t)value & 0x1) << 42; }

    /** Gets error vector 12h */
    bool get_STH_FV12() const { return (bool)(raw >> 42 & 0x1); }
        
    /** Sets error vector 11h */
    void set_STH_FV11(bool value){ raw = (raw & 0xfffffdffffffffff) | ((uint64_t)value & 0x1) << 41; }

    /** Gets error vector 11h */
    bool get_STH_FV11() const { return (bool)(raw >> 41 & 0x1); }
        
    /** Sets error vector 10h */
    void set_STH_FV10(bool value){ raw = (raw & 0xfffffeffffffffff) | ((uint64_t)value & 0x1) << 40; }

    /** Gets error vector 10h */
    bool get_STH_FV10() const { return (bool)(raw >> 40 & 0x1); }
        
    /** Sets error vector 1Fh */
    void set_STH_FV1F(bool value){ raw = (raw & 0xffffff7fffffffff) | ((uint64_t)value & 0x1) << 39; }

    /** Gets error vector 1Fh */
    bool get_STH_FV1F() const { return (bool)(raw >> 39 & 0x1); }
        
    /** Sets error vector 1Eh */
    void set_STH_FV1E(bool value){ raw = (raw & 0xffffffbfffffffff) | ((uint64_t)value & 0x1) << 38; }

    /** Gets error vector 1Eh */
    bool get_STH_FV1E() const { return (bool)(raw >> 38 & 0x1); }
        
    /** Sets error vector 1Dh */
    void set_STH_FV1D(bool value){ raw = (raw & 0xffffffdfffffffff) | ((uint64_t)value & 0x1) << 37; }

    /** Gets error vector 1Dh */
    bool get_STH_FV1D() const { return (bool)(raw >> 37 & 0x1); }
        
    /** Sets Error vector 1Ch */
    void set_STH_FV1C(bool value){ raw = (raw & 0xffffffefffffffff) | ((uint64_t)value & 0x1) << 36; }

    /** Gets Error vector 1Ch */
    bool get_STH_FV1C() const { return (bool)(raw >> 36 & 0x1); }
        
    /** Sets error vector 1Bh */
    void set_STH_FV1B(bool value){ raw = (raw & 0xfffffff7ffffffff) | ((uint64_t)value & 0x1) << 35; }

    /** Gets error vector 1Bh */
    bool get_STH_FV1B() const { return (bool)(raw >> 35 & 0x1); }
        
    /** Sets Error vector 1Ah */
    void set_STH_FV1A(bool value){ raw = (raw & 0xfffffffbffffffff) | ((uint64_t)value & 0x1) << 34; }

    /** Gets Error vector 1Ah */
    bool get_STH_FV1A() const { return (bool)(raw >> 34 & 0x1); }
        
    /** Sets error vector 19h */
    void set_STH_FV19(bool value){ raw = (raw & 0xfffffffdffffffff) | ((uint64_t)value & 0x1) << 33; }

    /** Gets error vector 19h */
    bool get_STH_FV19() const { return (bool)(raw >> 33 & 0x1); }
        
    /** Sets error vector 18h */
    void set_STH_FV18(bool value){ raw = (raw & 0xfffffffeffffffff) | ((uint64_t)value & 0x1) << 32; }

    /** Gets error vector 18h */
    bool get_STH_FV18() const { return (bool)(raw >> 32 & 0x1); }
        
    /** Sets error vector 27h */
    void set_STH_FV27(bool value){ raw = (raw & 0xffffffff7fffffff) | ((uint64_t)value & 0x1) << 31; }

    /** Gets error vector 27h */
    bool get_STH_FV27() const { return (bool)(raw >> 31 & 0x1); }
        
    /** Sets error vector 26h */
    void set_STH_FV26(bool value){ raw = (raw & 0xffffffffbfffffff) | ((uint64_t)value & 0x1) << 30; }

    /** Gets error vector 26h */
    bool get_STH_FV26() const { return (bool)(raw >> 30 & 0x1); }
        
    /** Sets error vector 25h */
    void set_STH_FV25(bool value){ raw = (raw & 0xffffffffdfffffff) | ((uint64_t)value & 0x1) << 29; }

    /** Gets error vector 25h */
    bool get_STH_FV25() const { return (bool)(raw >> 29 & 0x1); }
        
    /** Sets error vector 24h */
    void set_STH_FV24(bool value){ raw = (raw & 0xffffffffefffffff) | ((uint64_t)value & 0x1) << 28; }

    /** Gets error vector 24h */
    bool get_STH_FV24() const { return (bool)(raw >> 28 & 0x1); }
        
    /** Sets error vector 23h */
    void set_STH_FV23(bool value){ raw = (raw & 0xfffffffff7ffffff) | ((uint64_t)value & 0x1) << 27; }

    /** Gets error vector 23h */
    bool get_STH_FV23() const { return (bool)(raw >> 27 & 0x1); }
        
    /** Sets error vector 22h */
    void set_STH_FV22(bool value){ raw = (raw & 0xfffffffffbffffff) | ((uint64_t)value & 0x1) << 26; }

    /** Gets error vector 22h */
    bool get_STH_FV22() const { return (bool)(raw >> 26 & 0x1); }
        
    /** Sets error vector 21h */
    void set_STH_FV21(bool value){ raw = (raw & 0xfffffffffdffffff) | ((uint64_t)value & 0x1) << 25; }

    /** Gets error vector 21h */
    bool get_STH_FV21() const { return (bool)(raw >> 25 & 0x1); }
        
    /** Sets error vector 20h */
    void set_STH_FV20(bool value){ raw = (raw & 0xfffffffffeffffff) | ((uint64_t)value & 0x1) << 24; }

    /** Gets error vector 20h */
    bool get_STH_FV20() const { return (bool)(raw >> 24 & 0x1); }
        
    /** Sets error vector 2Fh */
    void set_STH_FV2F(bool value){ raw = (raw & 0xffffffffff7fffff) | ((uint64_t)value & 0x1) << 23; }

    /** Gets error vector 2Fh */
    bool get_STH_FV2F() const { return (bool)(raw >> 23 & 0x1); }
        
    /** Sets error vector 2Eh */
    void set_STH_FV2E(bool value){ raw = (raw & 0xffffffffffbfffff) | ((uint64_t)value & 0x1) << 22; }

    /** Gets error vector 2Eh */
    bool get_STH_FV2E() const { return (bool)(raw >> 22 & 0x1); }
        
    /** Sets error vector 2Dh */
    void set_STH_FV2D(bool value){ raw = (raw & 0xffffffffffdfffff) | ((uint64_t)value & 0x1) << 21; }

    /** Gets error vector 2Dh */
    bool get_STH_FV2D() const { return (bool)(raw >> 21 & 0x1); }
        
    /** Sets error vector 2Ch */
    void set_STH_FV2C(bool value){ raw = (raw & 0xffffffffffefffff) | ((uint64_t)value & 0x1) << 20; }

    /** Gets error vector 2Ch */
    bool get_STH_FV2C() const { return (bool)(raw >> 20 & 0x1); }
        
    /** Sets error vector 2Bh */
    void set_STH_FV2B(bool value){ raw = (raw & 0xfffffffffff7ffff) | ((uint64_t)value & 0x1) << 19; }

    /** Gets error vector 2Bh */
    bool get_STH_FV2B() const { return (bool)(raw >> 19 & 0x1); }
        
    /** Sets Error vector 2Ah */
    void set_STH_FV2A(bool value){ raw = (raw & 0xfffffffffffbffff) | ((uint64_t)value & 0x1) << 18; }

    /** Gets Error vector 2Ah */
    bool get_STH_FV2A() const { return (bool)(raw >> 18 & 0x1); }
        
    /** Sets error vector 29h */
    void set_STH_FV29(bool value){ raw = (raw & 0xfffffffffffdffff) | ((uint64_t)value & 0x1) << 17; }

    /** Gets error vector 29h */
    bool get_STH_FV29() const { return (bool)(raw >> 17 & 0x1); }
        
    /** Sets error vector 28h */
    void set_STH_FV28(bool value){ raw = (raw & 0xfffffffffffeffff) | ((uint64_t)value & 0x1) << 16; }

    /** Gets error vector 28h */
    bool get_STH_FV28() const { return (bool)(raw >> 16 & 0x1); }
        
    /** Sets error vector 37h */
    void set_STH_FV37(bool value){ raw = (raw & 0xffffffffffff7fff) | ((uint64_t)value & 0x1) << 15; }

    /** Gets error vector 37h */
    bool get_STH_FV37() const { return (bool)(raw >> 15 & 0x1); }
        
    /** Sets error vector 36h */
    void set_STH_FV36(bool value){ raw = (raw & 0xffffffffffffbfff) | ((uint64_t)value & 0x1) << 14; }

    /** Gets error vector 36h */
    bool get_STH_FV36() const { return (bool)(raw >> 14 & 0x1); }
        
    /** Sets error vector 35h */
    void set_STH_FV35(bool value){ raw = (raw & 0xffffffffffffdfff) | ((uint64_t)value & 0x1) << 13; }

    /** Gets error vector 35h */
    bool get_STH_FV35() const { return (bool)(raw >> 13 & 0x1); }
        
    /** Sets error vector 34h */
    void set_STH_FV34(bool value){ raw = (raw & 0xffffffffffffefff) | ((uint64_t)value & 0x1) << 12; }

    /** Gets error vector 34h */
    bool get_STH_FV34() const { return (bool)(raw >> 12 & 0x1); }
        
    /** Sets error vector 33h */
    void set_STH_FV33(bool value){ raw = (raw & 0xfffffffffffff7ff) | ((uint64_t)value & 0x1) << 11; }

    /** Gets error vector 33h */
    bool get_STH_FV33() const { return (bool)(raw >> 11 & 0x1); }
        
    /** Sets error vector 32h */
    void set_STH_FV32(bool value){ raw = (raw & 0xfffffffffffffbff) | ((uint64_t)value & 0x1) << 10; }

    /** Gets error vector 32h */
    bool get_STH_FV32() const { return (bool)(raw >> 10 & 0x1); }
        
    /** Sets error vector 31h */
    void set_STH_FV31(bool value){ raw = (raw & 0xfffffffffffffdff) | ((uint64_t)value & 0x1) << 9; }

    /** Gets error vector 31h */
    bool get_STH_FV31() const { return (bool)(raw >> 9 & 0x1); }
        
    /** Sets error vector 30h */
    void set_STH_FV30(bool value){ raw = (raw & 0xfffffffffffffeff) | ((uint64_t)value & 0x1) << 8; }

    /** Gets error vector 30h */
    bool get_STH_FV30() const { return (bool)(raw >> 8 & 0x1); }
        
    /** Sets state variable 08h */
    void set_STH_PGV08(bool value){ raw = (raw & 0xffffffffffffff7f) | ((uint64_t)value & 0x1) << 7; }

    /** Gets state variable 08h */
    bool get_STH_PGV08() const { return (bool)(raw >> 7 & 0x1); }
        
    /** Sets state variable 07h */
    void set_STH_PGV07(bool value){ raw = (raw & 0xffffffffffffffbf) | ((uint64_t)value & 0x1) << 6; }

    /** Gets state variable 07h */
    bool get_STH_PGV07() const { return (bool)(raw >> 6 & 0x1); }
        
    /** Sets state variable 06h */
    void set_STH_PGV06(bool value){ raw = (raw & 0xffffffffffffffdf) | ((uint64_t)value & 0x1) << 5; }

    /** Gets state variable 06h */
    bool get_STH_PGV06() const { return (bool)(raw >> 5 & 0x1); }
        
    /** Sets state variable 05h */
    void set_STH_PGV05(bool value){ raw = (raw & 0xffffffffffffffef) | ((uint64_t)value & 0x1) << 4; }

    /** Gets state variable 05h */
    bool get_STH_PGV05() const { return (bool)(raw >> 4 & 0x1); }
        
    /** Sets state variable 04h */
    void set_STH_PGV04(bool value){ raw = (raw & 0xfffffffffffffff7) | ((uint64_t)value & 0x1) << 3; }

    /** Gets state variable 04h */
    bool get_STH_PGV04() const { return (bool)(raw >> 3 & 0x1); }
        
    /** Sets state variable 03h */
    void set_STH_PGV03(bool value){ raw = (raw & 0xfffffffffffffffb) | ((uint64_t)value & 0x1) << 2; }

    /** Gets state variable 03h */
    bool get_STH_PGV03() const { return (bool)(raw >> 2 & 0x1); }
        
    /** Sets state variable 02h */
    void set_STH_PGV02(bool value){ raw = (raw & 0xfffffffffffffffd) | ((uint64_t)value & 0x1) << 1; }

    /** Gets state variable 02h */
    bool get_STH_PGV02() const { return (bool)(raw >> 1 & 0x1); }
        
    /** Sets state variable 01h */
    void set_STH_PGV01(bool value){ raw = (raw & 0xfffffffffffffffe) | ((uint64_t)value & 0x1) << 0; }

    /** Gets state variable 01h */
    bool get_STH_PGV01() const { return (bool)(raw >> 0 & 0x1); }
        
} SD_RS_STH;



class ECU_STH {
	public:
        /**
         * @brief Imports the CAN frame given the CAN ID, CAN Contents, and current timestamp
         *
         * Returns true if the frame was imported successfully, and false if import failed (Due to non-matching CAN ID).
         *
         * NOTE: The endianness of the value cannot be guaranteed. It is up to the caller to correct the byte order!
         */
        bool import_frames(uint64_t value, uint32_t can_id, uint64_t timestamp_now) {
            switch(can_id) {
                case STH_A1_CAN_ID:
                    LAST_FRAME_TIMES[0] = timestamp_now;
                    FRAME_DATA[0] = value;
                    return true;
                case SD_RS_STH_CAN_ID:
                    LAST_FRAME_TIMES[1] = timestamp_now;
                    FRAME_DATA[1] = value;
                    return true;
                default:
                    return false;
            }
        }
        
        /** Sets data in pointer to STH_A1
          * 
          * If this function returns false, then the CAN Frame is invalid or has not been seen
          * on the CANBUS network yet. Meaning it's data cannot be used.
          *
          * If the function returns true, then the pointer to 'dest' has been updated with the new CAN data
          */
        bool get_STH_A1(uint64_t now, uint64_t max_expire_time, STH_A1* dest) const {
            if (LAST_FRAME_TIMES[0] == 0 || dest == nullptr) { // CAN Frame has not been seen on bus yet / NULL pointer
                return false;
            } else if (now > LAST_FRAME_TIMES[0] && now - LAST_FRAME_TIMES[0] > max_expire_time) { // CAN Frame has not refreshed in valid interval
                return false;
            } else { // CAN Frame is valid! return it
                dest->raw = FRAME_DATA[0];
                return true;
            }
        }
            
        /** Sets data in pointer to SD_RS_STH
          * 
          * If this function returns false, then the CAN Frame is invalid or has not been seen
          * on the CANBUS network yet. Meaning it's data cannot be used.
          *
          * If the function returns true, then the pointer to 'dest' has been updated with the new CAN data
          */
        bool get_SD_RS_STH(uint64_t now, uint64_t max_expire_time, SD_RS_STH* dest) const {
            if (LAST_FRAME_TIMES[1] == 0 || dest == nullptr) { // CAN Frame has not been seen on bus yet / NULL pointer
                return false;
            } else if (now > LAST_FRAME_TIMES[1] && now - LAST_FRAME_TIMES[1] > max_expire_time) { // CAN Frame has not refreshed in valid interval
                return false;
            } else { // CAN Frame is valid! return it
                dest->raw = FRAME_DATA[1];
                return true;
            }
        }
            
	private:
		uint64_t FRAME_DATA[2];
		uint64_t LAST_FRAME_TIMES[2];
};
#endif // __ECU_STH_H_