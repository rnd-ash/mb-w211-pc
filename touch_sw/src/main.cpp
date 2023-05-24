#include <Arduino.h>
#include <HID.h>
#include <Wire.h>
#include <GT911.h>
#include "hid_display.hpp"

/**
 * pins
 * 
 * INT <-> 9 (PB1)
 * RST <-> 10 (PB3)
 * SDA <-> 19 (PD1)
 * SCL <-> 18 (PD0)
 * 
 */
// IPAD 3 screen
#define DISPLAY_X 2048
#define DISPLAY_Y 1536

GT911* gt911;
TouchScreen* screen;

void setup() {
  // put your setup code here, to run once:
  screen = new TouchScreen(1024, 768);
  gt911 = new GT911();
  gt911->begin(9, 13, GT911_I2C_ADDR_BA, 200000);
  GTConfig* cfg = gt911->readConfig();
  cfg->hSpace = DISPLAY_X;
  cfg->vSpace = DISPLAY_Y;
  gt911->writeConfig();
  GTInfo*  info = gt911->readInfo();
  Serial.begin(9600);
}

char buf[100];
uint8_t fingers_now[MAX_FINGER_COUNT];
uint8_t fingers_prev[MAX_FINGER_COUNT];

// Panel runs at 60Hz, so lets query input at 120Hz!
static const int INTERVAL_120HZ = 1000/120;
void loop() {
  uint64_t now = millis();
  uint8_t touches = gt911->touched(GT911_MODE_POLLING);
  GTPoint* tp = gt911->getPoints();
  memcpy(fingers_prev, fingers_now, sizeof(fingers_now));
  memset(fingers_now, 0x00, sizeof(fingers_now));
  if (touches) {
    for (int i = 0; i < min(touches, MAX_FINGER_COUNT); i++) {
      screen->send_hid(tp[i].trackId, 1, tp[i].x, tp[i].y);
      sprintf(buf, "#%d  %d,%d s:%d", tp[i].trackId, tp[i].x, tp[i].y, tp[i].area);
      fingers_now[i] = 1;
      Serial.println(buf);
    }
  }
  for (int i = 0; i < MAX_FINGER_COUNT; i++) {
    if (fingers_now[i] == 0 && fingers_prev[i] != 0) {
      screen->send_hid(i, 0, 0, 0);
    }
  }
  uint64_t elapsed = millis()-now;
  if (elapsed < INTERVAL_120HZ) {
    delay(INTERVAL_120HZ-elapsed);
  }
}