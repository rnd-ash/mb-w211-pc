#ifndef __HID_DISPLAY_H_
#define __HID_DISPLAY_H_

#include <Arduino.h>
#include <HID.h>

#define MAX_FINGER_COUNT 10
#define REPORTID_TOUCH 0x04

#define LOGICAL_MAX 10000

#define LSB(v) ((v >> 8) & 0xff)
#define MSB(v) (v & 0xff)

static const uint8_t _hidReportDescriptor[] PROGMEM = {
  0x05, 0x0D,                    // USAGE_PAGE(Digitizers)
  0x09, 0x04,                    // USAGE     (Touch Screen)
  0xA1, 0x01,                    // COLLECTION(Application)
  0x85, REPORTID_TOUCH,          //   REPORT_ID (Touch)

  // define the maximum amount of fingers that the device supports
  0x09, 0x55,                    //   USAGE (Contact Count Maximum)
  0x25, MAX_FINGER_COUNT,   //   LOGICAL_MAXIMUM (CONTACT_COUNT_MAXIMUM)
  0xB1, 0x02,                    //   FEATURE (Data,Var,Abs)

  // define the actual amount of fingers that are concurrently touching the screen
  0x09, 0x54,                    //   USAGE (Contact count)
  0x95, 0x01,                    //   REPORT_COUNT(1)
  0x75, 0x08,                    //   REPORT_SIZE (8)
  0x81, 0x02,                    //   INPUT (Data,Var,Abs)

  // declare a finger collection
  0x09, 0x22,                    //   USAGE (Finger)
  0xA1, 0x02,                    //   COLLECTION (Logical)

  // declare an identifier for the finger
  0x09, 0x51,                    //     USAGE (Contact Identifier)
  0x75, 0x08,                    //     REPORT_SIZE (8)
  0x95, 0x01,                    //     REPORT_COUNT (1)
  0x81, 0x02,                    //     INPUT (Data,Var,Abs)

  // declare Tip Switch and In Range
  0x09, 0x42,                    //     USAGE (Tip Switch)
  0x09, 0x32,                    //     USAGE (In Range)
  0x15, 0x00,                    //     LOGICAL_MINIMUM (0)
  0x25, 0x01,                    //     LOGICAL_MAXIMUM (1)
  0x75, 0x01,                    //     REPORT_SIZE (1)
  0x95, 0x02,                    //     REPORT_COUNT(2)
  0x81, 0x02,                    //     INPUT (Data,Var,Abs)

  // declare the remaining 6 bits of the first data byte as constant -> the driver will ignore them
  0x95, 0x06,                    //     REPORT_COUNT (6)
  0x81, 0x03,                    //     INPUT (Cnst,Ary,Abs)

  // define absolute X and Y coordinates of 16 bit each (percent values multiplied with 100)
  0x05, 0x01,                    //     USAGE_PAGE (Generic Desktop)
  0x09, 0x30,                    //     Usage (X)
  0x09, 0x31,                    //     Usage (Y)
  0x16, 0x00, 0x00,              //     Logical Minimum (0)
  0x26, 0x10, 0x27,              //     Logical Maximum (10000)
  0x36, 0x00, 0x00,              //     Physical Minimum (0)
  0x46, 0x10, 0x27,              //     Physical Maximum (10000)
  0x66, 0x00, 0x00,              //     UNIT (None)
  0x75, 0x10,                    //     Report Size (16),
  0x95, 0x02,                    //     Report Count (2),
  0x81, 0x02,                    //     Input (Data,Var,Abs)
  0xC0,                          //   END_COLLECTION
  0xC0                           // END_COLLECTION
};

typedef struct {
  uint8_t contact;
  uint8_t identifier;
  uint8_t touch;
  uint8_t MSB_X;
  uint8_t LSB_X;
  uint8_t MSB_Y;
  uint8_t LSB_Y;

} TouchReport __attribute__((packed));

class TouchScreen {
    public:
        TouchScreen(int panel_x, int panel_y);
        void send_hid(uint8_t identifier, uint8_t touch, int16_t x, int16_t y);
    private:
        float steps_per_logical_x;
        float steps_per_logical_y;
};

#endif

