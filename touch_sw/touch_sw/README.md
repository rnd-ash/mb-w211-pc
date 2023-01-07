# Touch software for W211 car PC

This runs on an Arduino Micro.

The micro is connected to a touch panel on top of an Ipad3 display, running the GT911 touch IC.

This runs at a refresh rate of 120Hz (Double the speed of the LCD display), and emulates a HID touch screen.

## Pinout

|Arduino Pin|Function|
|:-:|:-:|
|9|INT|
|10|RST|
|19|SDA|
|18|SCL|
