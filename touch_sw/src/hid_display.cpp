#include "hid_display.hpp"

TouchScreen::TouchScreen(int panel_x, int panel_y) {
    static HIDSubDescriptor node(_hidReportDescriptor, sizeof(_hidReportDescriptor));
    HID().AppendDescriptor(&node);
    this->steps_per_logical_x = (float)LOGICAL_MAX/(float)panel_x;
    this->steps_per_logical_y = (float)LOGICAL_MAX/(float)panel_y;
}

void TouchScreen::send_hid(uint8_t identifier, uint8_t touch, int16_t x, int16_t y) {
    // Convert our real display X/Y into logical X/Y
    int logical_x = this->steps_per_logical_x*x;
    int logical_y = this->steps_per_logical_y*y;

    TouchReport rep = TouchReport {
        .contact = 0,
        .identifier = identifier,
        .touch = touch,
        .MSB_X = MSB(logical_x),
        .LSB_X = LSB(logical_x),
        .MSB_Y = MSB(logical_y),
        .LSB_Y = LSB(logical_y),
    };
    
    HID().SendReport(REPORTID_TOUCH, (uint8_t*)&rep, 7);
}