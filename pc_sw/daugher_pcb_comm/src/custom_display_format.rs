use std::time::Duration;

use w211_can::socketcan_isotp::IsoTpSocket;


pub struct CDMIsoTp {
    handler: IsoTpSocket
}

/**
 * THIS MODULE CANNOT BE USED ON A STOCK IC211
 * 
 * This requires modified firmware, which allows the TeleAID CanIDs
 * to act as a ISOTP buffer, which gives you direct access to the displays format engine,
 * which, allows you to display anything on the IC using format strings
 */

impl CDMIsoTp {
    pub fn new(can: String) -> Self {
        Self {
            handler: w211_can::canbus::CanBus::create_isotp_socket_with_name(&can, 0x3E1, 0x1A1, 50, 8)
        }
    }

    pub fn test_beep(&mut self) {
        self.notify_track_change("Track ABC123");
    }

    pub fn notify_track_change(&mut self, name: &str) {
            let mut show_test = "".to_string();
            let mut count = 0;
            for c in name.chars() {
                show_test.push(c);
                count+=1;
                if count == 20 {
                    show_test.push_str("-~L");
                    count = 0;
                }

            }
            // IMAGES:
            // 90-99 Small lane arrows with directions?
            // 88 - Lap 
            // 87 - GPS
            // 86 - Data Rx
            // 85 - Data Tx
            // 84 - Mute
            // 83 - Unmute
            let text = format!("~I0~C0~P0500~J2~G1~ZTrack changed~G0~H0D~L~L{show_test}~L~B02B~P7890");//~L~B010~P7890");
            let mut buffer = vec![0x00, 0x00];
            buffer.extend_from_slice(text.as_bytes());
            buffer.push(0x00);
            let _ = self.handler.write(&buffer); // Write to string buffer before show
            std::thread::sleep(Duration::from_millis(40));
            let _ = self.handler.write(&[0xFE]); // Show screen
            std::thread::sleep(Duration::from_millis(2000));
            self.stop_display();
    }

    pub fn stop_display(&mut self) {
        let _ = self.handler.write(&[0x00, 0x00, 0x00]);
    }
}