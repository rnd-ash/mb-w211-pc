use atsamd_hal::usb::{UsbBus, usb_device::device::{UsbDevice, UsbDeviceState}};
use rtic_sync::channel::Sender;

use crate::can::{SERIAL_FRAME_LEN, SerialCanFrame};

pub struct UsbIsrInfo<'a> {
    pub dev: UsbDevice<'a, UsbBus>,
    pub serial: usbd_serial::SerialPort<'a, UsbBus>,
    pub buf: [u8; SERIAL_FRAME_LEN],
    pub last_state: UsbDeviceState,
    pub sender_can_frames: Sender<'static, SerialCanFrame, 200>
}