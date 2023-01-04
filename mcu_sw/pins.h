#ifndef __PINS_H_
#define __PINS_H_

#define PIN_V_SENSE PORT_PA07      // Voltage sense (ADC)
#define PIN_C_SENSE_PC PORT_PA04   // Current sense for PC (ADC)
#define PIN_C_SENSE_AMP PORT_PB09  // Current sense for AMP (ADC)

#define PIN_AMP_MOSFET PORT_PB04   // AMP Mosfet
#define PIN_PC_MOSFET PORT_PB05    // PC Mosfet

#define PIN_PC_PWR_BTN PORT_PB07   // PC Power button output
#define PIN_PC_RESET_BTN PORT_PB06 // PC Reset button output

#define PIN_FIO_PWR_BTN PORT_PB01  // Front panel power button input
#define PIN_FIO_HZD_BTN PORT_PB00  // Front panel hazard button input

#define PIN_AMP_STANDBY PORT_PA27  // Amplifier standby
#define PIN_AMP_MUTE PORT_PA05     // Amplifier mute


#define PIN_CANC_TX PORT_PB12 // CAN C Tx
#define PIN_CANC_RX PORT_PB13 // CAN C Rx
#define PIN_CANC_SD PORT_PA12 // CAN C Shutdown

#define PIN_CANB_TX PORT_PA22 // CAN B Tx
#define PIN_CANB_RX PORT_PA23 // CAN B Rx

#endif