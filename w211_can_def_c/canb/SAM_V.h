
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU 'SAM_V'
*/

#ifndef __ECU_SAM_V_H_
#define __ECU_SAM_V_H_

#include <stdint.h>
    
#define SAM_V_A1_CAN_ID 0x000A
#define SAM_V_A2_CAN_ID 0x0017
#define SAM_V_A3_CAN_ID 0x0070
#define SD_RS_SAM_V_CAN_ID 0x07C2



typedef union {
	uint64_t raw;
	uint8_t bytes[8];

	/** Gets CAN ID of SAM_V_A1 */
	uint32_t get_canid(){ return SAM_V_A1_CAN_ID; }
    /** Sets Terminal 61 */
    void set_KL_61_EIN(bool value){ raw = (raw & 0x7fffffffffffffff) | ((uint64_t)value & 0x1) << 63; }

    /** Gets Terminal 61 */
    bool get_KL_61_EIN() const { return (bool)(raw >> 63 & 0x1); }
        
    /** Sets Headlight activation active */
    void set_SWA_AKT(bool value){ raw = (raw & 0xbfffffffffffffff) | ((uint64_t)value & 0x1) << 62; }

    /** Gets Headlight activation active */
    bool get_SWA_AKT() const { return (bool)(raw >> 62 & 0x1); }
        
    /** Sets Kl15R on */
    void set_KL_15R_KG_V(bool value){ raw = (raw & 0xdfffffffffffffff) | ((uint64_t)value & 0x1) << 61; }

    /** Gets Kl15R on */
    bool get_KL_15R_KG_V() const { return (bool)(raw >> 61 & 0x1); }
        
    /** Sets Kl15/87 FW on */
    void set_KL_15_KG_V(bool value){ raw = (raw & 0xefffffffffffffff) | ((uint64_t)value & 0x1) << 60; }

    /** Gets Kl15/87 FW on */
    bool get_KL_15_KG_V() const { return (bool)(raw >> 60 & 0x1); }
        
    /** Sets Horn is on */
    void set_SGH_ST_EIN(bool value){ raw = (raw & 0xf7ffffffffffffff) | ((uint64_t)value & 0x1) << 59; }

    /** Gets Horn is on */
    bool get_SGH_ST_EIN() const { return (bool)(raw >> 59 & 0x1); }
        
    /** Sets High beam is switched on */
    void set_FL_ST_EIN(bool value){ raw = (raw & 0xfbffffffffffffff) | ((uint64_t)value & 0x1) << 58; }

    /** Gets High beam is switched on */
    bool get_FL_ST_EIN() const { return (bool)(raw >> 58 & 0x1); }
        
    /** Sets Fog lights are on */
    void set_NSW_ST_EIN(bool value){ raw = (raw & 0xfdffffffffffffff) | ((uint64_t)value & 0x1) << 57; }

    /** Gets Fog lights are on */
    bool get_NSW_ST_EIN() const { return (bool)(raw >> 57 & 0x1); }
        
    /** Sets Outside light activated by light sensor */
    void set_AFL_AKT(bool value){ raw = (raw & 0xfeffffffffffffff) | ((uint64_t)value & 0x1) << 56; }

    /** Gets Outside light activated by light sensor */
    bool get_AFL_AKT() const { return (bool)(raw >> 56 & 0x1); }
        
    /** Sets Turn on rear fog light */
    void set_NSL_EIN(bool value){ raw = (raw & 0xff7fffffffffffff) | ((uint64_t)value & 0x1) << 55; }

    /** Gets Turn on rear fog light */
    bool get_NSL_EIN() const { return (bool)(raw >> 55 & 0x1); }
        
    /** Sets Turn on fog lights */
    void set_NSW_EIN(bool value){ raw = (raw & 0xffbfffffffffffff) | ((uint64_t)value & 0x1) << 54; }

    /** Gets Turn on fog lights */
    bool get_NSW_EIN() const { return (bool)(raw >> 54 & 0x1); }
        
    /** Sets Turn on low beam */
    void set_ABL_EIN(bool value){ raw = (raw & 0xffdfffffffffffff) | ((uint64_t)value & 0x1) << 53; }

    /** Gets Turn on low beam */
    bool get_ABL_EIN() const { return (bool)(raw >> 53 & 0x1); }
        
    /** Sets Turn on parking lights */
    void set_STL_EIN(bool value){ raw = (raw & 0xffefffffffffffff) | ((uint64_t)value & 0x1) << 52; }

    /** Gets Turn on parking lights */
    bool get_STL_EIN() const { return (bool)(raw >> 52 & 0x1); }
        
    /** Sets Outside light dimmed with threshold 2 */
    void set_DIMM2(bool value){ raw = (raw & 0xfff7ffffffffffff) | ((uint64_t)value & 0x1) << 51; }

    /** Gets Outside light dimmed with threshold 2 */
    bool get_DIMM2() const { return (bool)(raw >> 51 & 0x1); }
        
    /** Sets Turn on the right parking light */
    void set_PL_RE_EIN(bool value){ raw = (raw & 0xfffbffffffffffff) | ((uint64_t)value & 0x1) << 50; }

    /** Gets Turn on the right parking light */
    bool get_PL_RE_EIN() const { return (bool)(raw >> 50 & 0x1); }
        
    /** Sets Turn on left parking light */
    void set_PL_LI_EIN(bool value){ raw = (raw & 0xfffdffffffffffff) | ((uint64_t)value & 0x1) << 49; }

    /** Gets Turn on left parking light */
    bool get_PL_LI_EIN() const { return (bool)(raw >> 49 & 0x1); }
        
    /** Sets Turn on high beam */
    void set_FL_EIN_SAM_V(bool value){ raw = (raw & 0xfffeffffffffffff) | ((uint64_t)value & 0x1) << 48; }

    /** Gets Turn on high beam */
    bool get_FL_EIN_SAM_V() const { return (bool)(raw >> 48 & 0x1); }
        
    /** Sets daytime running lights on */
    void set_TFL_EIN_ECE(bool value){ raw = (raw & 0xffff7fffffffffff) | ((uint64_t)value & 0x1) << 47; }

    /** Gets daytime running lights on */
    bool get_TFL_EIN_ECE() const { return (bool)(raw >> 47 & 0x1); }
        
    /** Sets Refrigeration compressor is running */
    void set_KOMP_LFT(bool value){ raw = (raw & 0xffffbfffffffffff) | ((uint64_t)value & 0x1) << 46; }

    /** Gets Refrigeration compressor is running */
    bool get_KOMP_LFT() const { return (bool)(raw >> 46 & 0x1); }
        
    /** Sets Handbrake applied (indicator lamp) */
    void set_HAS_KL(bool value){ raw = (raw & 0xffffdfffffffffff) | ((uint64_t)value & 0x1) << 45; }

    /** Gets Handbrake applied (indicator lamp) */
    bool get_HAS_KL() const { return (bool)(raw >> 45 & 0x1); }
        
    /** Sets Air conditioning compressor switched on */
    void set_KOMP_EIN(bool value){ raw = (raw & 0xffffefffffffffff) | ((uint64_t)value & 0x1) << 44; }

    /** Gets Air conditioning compressor switched on */
    bool get_KOMP_EIN() const { return (bool)(raw >> 44 & 0x1); }
        
    /** Sets Refrigeration compressor control current output defective */
    void set_KOMP_DEF(bool value){ raw = (raw & 0xfffff7ffffffffff) | ((uint64_t)value & 0x1) << 43; }

    /** Gets Refrigeration compressor control current output defective */
    bool get_KOMP_DEF() const { return (bool)(raw >> 43 & 0x1); }
        
    /** Sets Terminal 15 activated via diagnostics */
    void set_DIAG_15_EIN(bool value){ raw = (raw & 0xfffffbffffffffff) | ((uint64_t)value & 0x1) << 42; }

    /** Gets Terminal 15 activated via diagnostics */
    bool get_DIAG_15_EIN() const { return (bool)(raw >> 42 & 0x1); }
        
    /** Sets Terminal 15R activated via diagnostics */
    void set_DIAG_15R_EIN(bool value){ raw = (raw & 0xfffffdffffffffff) | ((uint64_t)value & 0x1) << 41; }

    /** Gets Terminal 15R activated via diagnostics */
    bool get_DIAG_15R_EIN() const { return (bool)(raw >> 41 & 0x1); }
        
    /** Sets Brake pad wear warning light */
    void set_BBV_KL(bool value){ raw = (raw & 0xfffffeffffffffff) | ((uint64_t)value & 0x1) << 40; }

    /** Gets Brake pad wear warning light */
    bool get_BBV_KL() const { return (bool)(raw >> 40 & 0x1); }
        
    /** Sets Brake fluid level warning light */
    void set_BFL_KL(bool value){ raw = (raw & 0xffffff7fffffffff) | ((uint64_t)value & 0x1) << 39; }

    /** Gets Brake fluid level warning light */
    bool get_BFL_KL() const { return (bool)(raw >> 39 & 0x1); }
        
    /** Sets washer fluid level too low indicator light */
    void set_WWS_KL(bool value){ raw = (raw & 0xffffffbfffffffff) | ((uint64_t)value & 0x1) << 38; }

    /** Gets washer fluid level too low indicator light */
    bool get_WWS_KL() const { return (bool)(raw >> 38 & 0x1); }
        
    /** Sets Cooling water level too low Control lamp */
    void set_KWS_KL(bool value){ raw = (raw & 0xffffffdfffffffff) | ((uint64_t)value & 0x1) << 37; }

    /** Gets Cooling water level too low Control lamp */
    bool get_KWS_KL() const { return (bool)(raw >> 37 & 0x1); }
        
    /** Sets SBC added value: run-on active */
    void set_MW_AKT_SAM_V(bool value){ raw = (raw & 0xffffffefffffffff) | ((uint64_t)value & 0x1) << 36; }

    /** Gets SBC added value: run-on active */
    bool get_MW_AKT_SAM_V() const { return (bool)(raw >> 36 & 0x1); }
        
    /** Sets SAM/x: v-signal from EHB-ASG, x = B (230), V (211), F ( 240) */
    void set_VSTAT_A(bool value){ raw = (raw & 0xfffffff7ffffffff) | ((uint64_t)value & 0x1) << 35; }

    /** Gets SAM/x: v-signal from EHB-ASG, x = B (230), V (211), F ( 240) */
    bool get_VSTAT_A() const { return (bool)(raw >> 35 & 0x1); }
        
    /** Sets SAM/V passive */
    void set_SAM_V_PAS(bool value){ raw = (raw & 0xfffffffbffffffff) | ((uint64_t)value & 0x1) << 34; }

    /** Gets SAM/V passive */
    bool get_SAM_V_PAS() const { return (bool)(raw >> 34 & 0x1); }
        
    /** Sets SAM/x: brake light switch output EHB-ASG, x = B (230), V (211), F (240) */
    void set_BLS_A(bool value){ raw = (raw & 0xfffffffdffffffff) | ((uint64_t)value & 0x1) << 33; }

    /** Gets SAM/x: brake light switch output EHB-ASG, x = B (230), V (211), F (240) */
    bool get_BLS_A() const { return (bool)(raw >> 33 & 0x1); }
        
    /** Sets SAM/x: EHB-ASG in fallback level, x = B (230), V (211,164,251), F (240) */
    void set_INF_RFE_SAM(bool value){ raw = (raw & 0xfffffffeffffffff) | ((uint64_t)value & 0x1) << 32; }

    /** Gets SAM/x: EHB-ASG in fallback level, x = B (230), V (211,164,251), F (240) */
    bool get_INF_RFE_SAM() const { return (bool)(raw >> 32 & 0x1); }
        
    /** Sets Driver side fog lights defective */
    void set_NSW_DEF_F(bool value){ raw = (raw & 0xffffffff7fffffff) | ((uint64_t)value & 0x1) << 31; }

    /** Gets Driver side fog lights defective */
    bool get_NSW_DEF_F() const { return (bool)(raw >> 31 & 0x1); }
        
    /** Sets High beam on driver's side defective */
    void set_FL_DEF_F(bool value){ raw = (raw & 0xffffffffbfffffff) | ((uint64_t)value & 0x1) << 30; }

    /** Gets High beam on driver's side defective */
    bool get_FL_DEF_F() const { return (bool)(raw >> 30 & 0x1); }
        
    /** Sets driver's side low beam defective */
    void set_ABL_DEF_F(bool value){ raw = (raw & 0xffffffffdfffffff) | ((uint64_t)value & 0x1) << 29; }

    /** Gets driver's side low beam defective */
    bool get_ABL_DEF_F() const { return (bool)(raw >> 29 & 0x1); }
        
    /** Sets Front parking light on driver's side defective */
    void set_PL_DEF_VF(bool value){ raw = (raw & 0xffffffffefffffff) | ((uint64_t)value & 0x1) << 28; }

    /** Gets Front parking light on driver's side defective */
    bool get_PL_DEF_VF() const { return (bool)(raw >> 28 & 0x1); }
        
    /** Sets Front turn signal driver's side defective */
    void set_BLI_DEF_VF(bool value){ raw = (raw & 0xfffffffff7ffffff) | ((uint64_t)value & 0x1) << 27; }

    /** Gets Front turn signal driver's side defective */
    bool get_BLI_DEF_VF() const { return (bool)(raw >> 27 & 0x1); }
        
    /** Sets Front driver's side marker defective */
    void set_SM_DEF_VF(bool value){ raw = (raw & 0xfffffffffbffffff) | ((uint64_t)value & 0x1) << 26; }

    /** Gets Front driver's side marker defective */
    bool get_SM_DEF_VF() const { return (bool)(raw >> 26 & 0x1); }
        
    /** Sets Front passenger side side marker defective */
    void set_SM_DEF_VBF(bool value){ raw = (raw & 0xfffffffffdffffff) | ((uint64_t)value & 0x1) << 25; }

    /** Gets Front passenger side side marker defective */
    bool get_SM_DEF_VBF() const { return (bool)(raw >> 25 & 0x1); }
        
    /** Sets Instrument lights off */
    void set_INSTR_AUS(bool value){ raw = (raw & 0xfffffffffeffffff) | ((uint64_t)value & 0x1) << 24; }

    /** Gets Instrument lights off */
    bool get_INSTR_AUS() const { return (bool)(raw >> 24 & 0x1); }
        
    /** Sets Replacement turn signals on the front driver's side are active */
    void set_BLI_ERS_VF(bool value){ raw = (raw & 0xffffffffff7fffff) | ((uint64_t)value & 0x1) << 23; }

    /** Gets Replacement turn signals on the front driver's side are active */
    bool get_BLI_ERS_VF() const { return (bool)(raw >> 23 & 0x1); }
        
    /** Sets Replacement front parking light on driver's side active */
    void set_PL_ERS_VF(bool value){ raw = (raw & 0xffffffffffbfffff) | ((uint64_t)value & 0x1) << 22; }

    /** Gets Replacement front parking light on driver's side active */
    bool get_PL_ERS_VF() const { return (bool)(raw >> 22 & 0x1); }
        
    /** Sets Start Xenon4 diagnostic procedure driver's side */
    void set_DIAG_X4_F(bool value){ raw = (raw & 0xffffffffffdfffff) | ((uint64_t)value & 0x1) << 21; }

    /** Gets Start Xenon4 diagnostic procedure driver's side */
    bool get_DIAG_X4_F() const { return (bool)(raw >> 21 & 0x1); }
        
    /** Sets front brake pad wear 50% */
    void set_BBV_V_50(bool value){ raw = (raw & 0xffffffffffff7fff) | ((uint64_t)value & 0x1) << 15; }

    /** Gets front brake pad wear 50% */
    bool get_BBV_V_50() const { return (bool)(raw >> 15 & 0x1); }
        
    /** Sets front brake pad wear 100% */
    void set_BBV_V_100(bool value){ raw = (raw & 0xffffffffffffbfff) | ((uint64_t)value & 0x1) << 14; }

    /** Gets front brake pad wear 100% */
    bool get_BBV_V_100() const { return (bool)(raw >> 14 & 0x1); }
        
    /** Sets Rear brake pad wear 50% */
    void set_BBV_H_50(bool value){ raw = (raw & 0xffffffffffffdfff) | ((uint64_t)value & 0x1) << 13; }

    /** Gets Rear brake pad wear 50% */
    bool get_BBV_H_50() const { return (bool)(raw >> 13 & 0x1); }
        
    /** Sets rear brake pad wear 100% */
    void set_BBV_H_100(bool value){ raw = (raw & 0xffffffffffffefff) | ((uint64_t)value & 0x1) << 12; }

    /** Gets rear brake pad wear 100% */
    bool get_BBV_H_100() const { return (bool)(raw >> 12 & 0x1); }
        
    /** Sets EMS auxiliary power supply. Conversion formula (To raw from real): y=(x-0.0)/1.00 */
    void set_EHB_BEH(uint8_t value){ raw = (raw & 0xfffffffffffff3ff) | ((uint64_t)value & 0x3) << 10; }

    /** Gets EMS auxiliary power supply. Conversion formula (To real from raw): y=(1.00x)+0.0 */
    uint8_t get_EHB_BEH() const { return (uint8_t)(raw >> 10 & 0x3); }
        
    /** Sets readback signal Kl15R */
    void set_KL15R_ST_RL(bool value){ raw = (raw & 0xfffffffffffffdff) | ((uint64_t)value & 0x1) << 9; }

    /** Gets readback signal Kl15R */
    bool get_KL15R_ST_RL() const { return (bool)(raw >> 9 & 0x1); }
        
    /** Sets readback signal Kl15 */
    void set_KL15_ST_RL(bool value){ raw = (raw & 0xfffffffffffffeff) | ((uint64_t)value & 0x1) << 8; }

    /** Gets readback signal Kl15 */
    bool get_KL15_ST_RL() const { return (bool)(raw >> 8 & 0x1); }
        
} SAM_V_A1;



typedef union {
	uint64_t raw;
	uint8_t bytes[8];

	/** Gets CAN ID of SAM_V_A2 */
	uint32_t get_canid(){ return SAM_V_A2_CAN_ID; }
    /** Sets outside air temperature. Conversion formula (To raw from real): y=(x+40.0)/0.50 (Unit: °C) */
    void set_T_AUSSEN_B(uint8_t value){ raw = (raw & 0x00ffffffffffffff) | ((uint64_t)value & 0xff) << 56; }

    /** Gets outside air temperature. Conversion formula (To real from raw): y=(0.50x)-40.0 (Unit: °C) */
    uint8_t get_T_AUSSEN_B() const { return (uint8_t)(raw >> 56 & 0xff); }
        
    /** Sets Pressure refrigerant R134a. Conversion formula (To raw from real): y=(x-0.0)/0.10 (Unit: bar) */
    void set_P_KAELTE(uint16_t value){ raw = (raw & 0xff0000ffffffffff) | ((uint64_t)value & 0xffff) << 40; }

    /** Gets Pressure refrigerant R134a. Conversion formula (To real from raw): y=(0.10x)+0.0 (Unit: bar) */
    uint16_t get_P_KAELTE() const { return (uint16_t)(raw >> 40 & 0xffff); }
        
    /** Sets Refrigerant R134a temperature. Conversion formula (To raw from real): y=(x+10.0)/0.10 (Unit: °C) */
    void set_T_KAELTE(uint16_t value){ raw = (raw & 0xffffff0000ffffff) | ((uint64_t)value & 0xffff) << 24; }

    /** Gets Refrigerant R134a temperature. Conversion formula (To real from raw): y=(0.10x)-10.0 (Unit: °C) */
    uint16_t get_T_KAELTE() const { return (uint16_t)(raw >> 24 & 0xffff); }
        
    /** Sets Compressor main control valve flow. Conversion formula (To raw from real): y=(x-0.0)/10.00 (Unit: mA) */
    void set_I_KOMP(uint8_t value){ raw = (raw & 0xffffffffff00ffff) | ((uint64_t)value & 0xff) << 16; }

    /** Gets Compressor main control valve flow. Conversion formula (To real from raw): y=(10.00x)+0.0 (Unit: mA) */
    uint8_t get_I_KOMP() const { return (uint8_t)(raw >> 16 & 0xff); }
        
} SAM_V_A2;



typedef union {
	uint64_t raw;
	uint8_t bytes[8];

	/** Gets CAN ID of SAM_V_A3 */
	uint32_t get_canid(){ return SAM_V_A3_CAN_ID; }
    /** Sets Diagnosis rain sensor */
    void set_DIAG_RS(bool value){ raw = (raw & 0xdfffffffffffffff) | ((uint64_t)value & 0x1) << 61; }

    /** Gets Diagnosis rain sensor */
    bool get_DIAG_RS() const { return (bool)(raw >> 61 & 0x1); }
        
    /** Sets Steering column switch in stage 1 */
    void set_SCH_WI_1_RS(bool value){ raw = (raw & 0xefffffffffffffff) | ((uint64_t)value & 0x1) << 60; }

    /** Gets Steering column switch in stage 1 */
    bool get_SCH_WI_1_RS() const { return (bool)(raw >> 60 & 0x1); }
        
    /** Sets SAM_V initialization */
    void set_SAM_V_INIT(bool value){ raw = (raw & 0xf7ffffffffffffff) | ((uint64_t)value & 0x1) << 59; }

    /** Gets SAM_V initialization */
    bool get_SAM_V_INIT() const { return (bool)(raw >> 59 & 0x1); }
        
    /** Sets Wash pressed */
    void set_KL_86_RS(bool value){ raw = (raw & 0xfbffffffffffffff) | ((uint64_t)value & 0x1) << 58; }

    /** Gets Wash pressed */
    bool get_KL_86_RS() const { return (bool)(raw >> 58 & 0x1); }
        
    /** Sets Wiper out of park position */
    void set_KL_31B_RS(bool value){ raw = (raw & 0xfdffffffffffffff) | ((uint64_t)value & 0x1) << 57; }

    /** Gets Wiper out of park position */
    bool get_KL_31B_RS() const { return (bool)(raw >> 57 & 0x1); }
        
    /** Sets rain sensor on/off (interval position) */
    void set_RS_INT(bool value){ raw = (raw & 0xfeffffffffffffff) | ((uint64_t)value & 0x1) << 56; }

    /** Gets rain sensor on/off (interval position) */
    bool get_RS_INT() const { return (bool)(raw >> 56 & 0x1); }
        
} SAM_V_A3;



typedef union {
	uint64_t raw;
	uint8_t bytes[8];

	/** Gets CAN ID of SD_RS_SAM_V */
	uint32_t get_canid(){ return SD_RS_SAM_V_CAN_ID; }
    /** Sets Identification for > 8 bytes */
    void set_SAM_V_KENN(bool value){ raw = (raw & 0x7fffffffffffffff) | ((uint64_t)value & 0x1) << 63; }

    /** Gets Identification for > 8 bytes */
    bool get_SAM_V_KENN() const { return (bool)(raw >> 63 & 0x1); }
        
    /** Sets state variable 07h */
    void set_SAM_V_PGV07(bool value){ raw = (raw & 0xbfffffffffffffff) | ((uint64_t)value & 0x1) << 62; }

    /** Gets state variable 07h */
    bool get_SAM_V_PGV07() const { return (bool)(raw >> 62 & 0x1); }
        
    /** Sets state variable 06h */
    void set_SAM_V_PGV06(bool value){ raw = (raw & 0xdfffffffffffffff) | ((uint64_t)value & 0x1) << 61; }

    /** Gets state variable 06h */
    bool get_SAM_V_PGV06() const { return (bool)(raw >> 61 & 0x1); }
        
    /** Sets state variable 05h */
    void set_SAM_V_PGV05(bool value){ raw = (raw & 0xefffffffffffffff) | ((uint64_t)value & 0x1) << 60; }

    /** Gets state variable 05h */
    bool get_SAM_V_PGV05() const { return (bool)(raw >> 60 & 0x1); }
        
    /** Sets state variable 04h */
    void set_SAM_V_PGV04(bool value){ raw = (raw & 0xf7ffffffffffffff) | ((uint64_t)value & 0x1) << 59; }

    /** Gets state variable 04h */
    bool get_SAM_V_PGV04() const { return (bool)(raw >> 59 & 0x1); }
        
    /** Sets state variable 03h */
    void set_SAM_V_PGV03(bool value){ raw = (raw & 0xfbffffffffffffff) | ((uint64_t)value & 0x1) << 58; }

    /** Gets state variable 03h */
    bool get_SAM_V_PGV03() const { return (bool)(raw >> 58 & 0x1); }
        
    /** Sets state variable 02h */
    void set_SAM_V_PGV02(bool value){ raw = (raw & 0xfdffffffffffffff) | ((uint64_t)value & 0x1) << 57; }

    /** Gets state variable 02h */
    bool get_SAM_V_PGV02() const { return (bool)(raw >> 57 & 0x1); }
        
    /** Sets state variable 01h */
    void set_SAM_V_PGV01(bool value){ raw = (raw & 0xfeffffffffffffff) | ((uint64_t)value & 0x1) << 56; }

    /** Gets state variable 01h */
    bool get_SAM_V_PGV01() const { return (bool)(raw >> 56 & 0x1); }
        
    /** Sets Error message 01h. Conversion formula (To raw from real): y=(x-0.0)/1.00 */
    void set_SAM_V_FM01(uint16_t value){ raw = (raw & 0xff0000ffffffffff) | ((uint64_t)value & 0xffff) << 40; }

    /** Gets Error message 01h. Conversion formula (To real from raw): y=(1.00x)+0.0 */
    uint16_t get_SAM_V_FM01() const { return (uint16_t)(raw >> 40 & 0xffff); }
        
    /** Sets Error message 02h. Conversion formula (To raw from real): y=(x-0.0)/1.00 */
    void set_SAM_V_FM02(uint16_t value){ raw = (raw & 0xffffff0000ffffff) | ((uint64_t)value & 0xffff) << 24; }

    /** Gets Error message 02h. Conversion formula (To real from raw): y=(1.00x)+0.0 */
    uint16_t get_SAM_V_FM02() const { return (uint16_t)(raw >> 24 & 0xffff); }
        
    /** Sets Error message 03h. Conversion formula (To raw from real): y=(x-0.0)/1.00 */
    void set_SAM_V_FM03(uint16_t value){ raw = (raw & 0xffffffffff0000ff) | ((uint64_t)value & 0xffff) << 8; }

    /** Gets Error message 03h. Conversion formula (To real from raw): y=(1.00x)+0.0 */
    uint16_t get_SAM_V_FM03() const { return (uint16_t)(raw >> 8 & 0xffff); }
        
    /** Sets state variable 0Fh */
    void set_SAM_V_PGV0F(bool value){ raw = (raw & 0xffffffffffffff7f) | ((uint64_t)value & 0x1) << 7; }

    /** Gets state variable 0Fh */
    bool get_SAM_V_PGV0F() const { return (bool)(raw >> 7 & 0x1); }
        
    /** Sets state variable 0Eh */
    void set_SAM_V_PGV0E(bool value){ raw = (raw & 0xffffffffffffffbf) | ((uint64_t)value & 0x1) << 6; }

    /** Gets state variable 0Eh */
    bool get_SAM_V_PGV0E() const { return (bool)(raw >> 6 & 0x1); }
        
    /** Sets State variable 0Dh */
    void set_SAM_V_PGV0D(bool value){ raw = (raw & 0xffffffffffffffdf) | ((uint64_t)value & 0x1) << 5; }

    /** Gets State variable 0Dh */
    bool get_SAM_V_PGV0D() const { return (bool)(raw >> 5 & 0x1); }
        
    /** Sets state variable 0Ch */
    void set_SAM_V_PGV0C(bool value){ raw = (raw & 0xffffffffffffffef) | ((uint64_t)value & 0x1) << 4; }

    /** Gets state variable 0Ch */
    bool get_SAM_V_PGV0C() const { return (bool)(raw >> 4 & 0x1); }
        
    /** Sets state variable 0Bh */
    void set_SAM_V_PGV0B(bool value){ raw = (raw & 0xfffffffffffffff7) | ((uint64_t)value & 0x1) << 3; }

    /** Gets state variable 0Bh */
    bool get_SAM_V_PGV0B() const { return (bool)(raw >> 3 & 0x1); }
        
    /** Sets State variable 0Ah */
    void set_SAM_V_PGV0A(bool value){ raw = (raw & 0xfffffffffffffffb) | ((uint64_t)value & 0x1) << 2; }

    /** Gets State variable 0Ah */
    bool get_SAM_V_PGV0A() const { return (bool)(raw >> 2 & 0x1); }
        
    /** Sets state variable 09h */
    void set_SAM_V_PGV09(bool value){ raw = (raw & 0xfffffffffffffffd) | ((uint64_t)value & 0x1) << 1; }

    /** Gets state variable 09h */
    bool get_SAM_V_PGV09() const { return (bool)(raw >> 1 & 0x1); }
        
    /** Sets state variable 08h */
    void set_SAM_V_PGV08(bool value){ raw = (raw & 0xfffffffffffffffe) | ((uint64_t)value & 0x1) << 0; }

    /** Gets state variable 08h */
    bool get_SAM_V_PGV08() const { return (bool)(raw >> 0 & 0x1); }
        
} SD_RS_SAM_V;



class ECU_SAM_V {
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
                case SAM_V_A1_CAN_ID:
                    LAST_FRAME_TIMES[0] = timestamp_now;
                    FRAME_DATA[0] = value;
                    return true;
                case SAM_V_A2_CAN_ID:
                    LAST_FRAME_TIMES[1] = timestamp_now;
                    FRAME_DATA[1] = value;
                    return true;
                case SAM_V_A3_CAN_ID:
                    LAST_FRAME_TIMES[2] = timestamp_now;
                    FRAME_DATA[2] = value;
                    return true;
                case SD_RS_SAM_V_CAN_ID:
                    LAST_FRAME_TIMES[3] = timestamp_now;
                    FRAME_DATA[3] = value;
                    return true;
                default:
                    return false;
            }
        }
        
        /** Sets data in pointer to SAM_V_A1
          * 
          * If this function returns false, then the CAN Frame is invalid or has not been seen
          * on the CANBUS network yet. Meaning it's data cannot be used.
          *
          * If the function returns true, then the pointer to 'dest' has been updated with the new CAN data
          */
        bool get_SAM_V_A1(uint64_t now, uint64_t max_expire_time, SAM_V_A1* dest) const {
            if (LAST_FRAME_TIMES[0] == 0 || dest == nullptr) { // CAN Frame has not been seen on bus yet / NULL pointer
                return false;
            } else if (now > LAST_FRAME_TIMES[0] && now - LAST_FRAME_TIMES[0] > max_expire_time) { // CAN Frame has not refreshed in valid interval
                return false;
            } else { // CAN Frame is valid! return it
                dest->raw = FRAME_DATA[0];
                return true;
            }
        }
            
        /** Sets data in pointer to SAM_V_A2
          * 
          * If this function returns false, then the CAN Frame is invalid or has not been seen
          * on the CANBUS network yet. Meaning it's data cannot be used.
          *
          * If the function returns true, then the pointer to 'dest' has been updated with the new CAN data
          */
        bool get_SAM_V_A2(uint64_t now, uint64_t max_expire_time, SAM_V_A2* dest) const {
            if (LAST_FRAME_TIMES[1] == 0 || dest == nullptr) { // CAN Frame has not been seen on bus yet / NULL pointer
                return false;
            } else if (now > LAST_FRAME_TIMES[1] && now - LAST_FRAME_TIMES[1] > max_expire_time) { // CAN Frame has not refreshed in valid interval
                return false;
            } else { // CAN Frame is valid! return it
                dest->raw = FRAME_DATA[1];
                return true;
            }
        }
            
        /** Sets data in pointer to SAM_V_A3
          * 
          * If this function returns false, then the CAN Frame is invalid or has not been seen
          * on the CANBUS network yet. Meaning it's data cannot be used.
          *
          * If the function returns true, then the pointer to 'dest' has been updated with the new CAN data
          */
        bool get_SAM_V_A3(uint64_t now, uint64_t max_expire_time, SAM_V_A3* dest) const {
            if (LAST_FRAME_TIMES[2] == 0 || dest == nullptr) { // CAN Frame has not been seen on bus yet / NULL pointer
                return false;
            } else if (now > LAST_FRAME_TIMES[2] && now - LAST_FRAME_TIMES[2] > max_expire_time) { // CAN Frame has not refreshed in valid interval
                return false;
            } else { // CAN Frame is valid! return it
                dest->raw = FRAME_DATA[2];
                return true;
            }
        }
            
        /** Sets data in pointer to SD_RS_SAM_V
          * 
          * If this function returns false, then the CAN Frame is invalid or has not been seen
          * on the CANBUS network yet. Meaning it's data cannot be used.
          *
          * If the function returns true, then the pointer to 'dest' has been updated with the new CAN data
          */
        bool get_SD_RS_SAM_V(uint64_t now, uint64_t max_expire_time, SD_RS_SAM_V* dest) const {
            if (LAST_FRAME_TIMES[3] == 0 || dest == nullptr) { // CAN Frame has not been seen on bus yet / NULL pointer
                return false;
            } else if (now > LAST_FRAME_TIMES[3] && now - LAST_FRAME_TIMES[3] > max_expire_time) { // CAN Frame has not refreshed in valid interval
                return false;
            } else { // CAN Frame is valid! return it
                dest->raw = FRAME_DATA[3];
                return true;
            }
        }
            
	private:
		uint64_t FRAME_DATA[4];
		uint64_t LAST_FRAME_TIMES[4];
};
#endif // __ECU_SAM_V_H_