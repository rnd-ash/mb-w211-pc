use std::time::{Duration, Instant};
use tokio::{runtime::Runtime, sync::mpsc::{unbounded_channel, UnboundedSender}};

#[repr(u16)]
pub enum Image {
    // Warning icons
    GlowPlug = 0x00,
    BrakeWear = 0x01,
    FuelLow = 0x02,
    ParkBrake = 0x03,
    ParkBrakeOnWarn = 0x04,
    BrakeText = 0x05,
    SrsText = 0x06,
    EspText = 0x07,
    AbsText = 0x08,
    BasText = 0x09,
    CoolantTemp = 0x0A,
    WasherFluid = 0x0B,
    CoolantLevel = 0x0C,
    BulbWarning = 0x0D,
    SeatbeltWarning = 0x0E,
    BatteryBig = 0x0F,
    CheckEngine = 0x10,
    P = 0x11,
    Electric = 0x12,
    Key = 0x13,
    Oil = 0x14,
    EpcText = 0x15,
    TrailerHook = 0x16,
    Taxi = 0x17,
    TeleaidCallSOS = 0x18,
    SosText = 0x19,
    TyrePressureWarn = 0x1A,
    // Teleaid
    TeleaidCallInfo = 0x1B,
    TeleaidCallService = 0x1C,
    TeleaidCallFCD = 0x1D,
    // Service
    Spanner = 0x1E,
    DoubleSpanner = 0x1F,
    // ??
    CarTopDownLRSaloon = 0x20,
    CarTopDownLRWagon = 0x21,
    EndCall = 0x22,
    AirmaticSaloon = 0x23,
    AirmaticWagon = 0x24,
    AdaptiveDamper = 0x25,
    SeatTilt = 0x26,
    Seat = 0x27,
    Stop = 0x28,
    TrunkOpenWagon = 0x29,
    //2A ??
    Disc = 0x2B,
    SbcS = 0x2C,
    SbcH = 0x2D,
    Plus = 0x2E,
    FuelFilter = 0x2F,
    FuelFilterTank = 0x30,
    SunroofSlideClose = 0x31,
    SunroofDownClose = 0x32,
    AirFilter = 0x33,
    Hold = 0x34,
    RefillWithSaloon = 0x35,
    RefillWithWagon = 0x36,
    Prohibited = 0x37,
    BatterySmall = 0x38,
    OilTemperatureSmall = 0x39,
    // 0x3A - 0x4B - Door icons
    WagonSideView = 0x4C,
    SaloonSideView = 0x51,
    // DTR images
    DtrDistantCar = 0x56,
    DtrSideCarView = 0x57,
    Dtr100mBar = 0x58,
    Dtr100ydBar = 0x59,
    Dtr300ftBar = 0x5A,
    DtrSoundOn = 0x5B,
    DtrBarEmpty = 0x5C,
    // Begin settings images
    MinusInCircle = 0x5D,
    PlusInCircle = 0x5E,
    LaneFrontView = 0x5F,
    Thermometer = 0x60,
    Clock = 0x61,
    Lim = 0x62,
    TopDownLamp = 0x63,
    FrontLamp = 0x64,
    SideLights = 0x65,
    TaxiLarge = 0x66,
    SeatLarge = 0x67,
    // Images for status line maybe
    TinyCar = 0x68,
    TinyLim = 0x69,
    TinyDtr = 0x6A,
    TinyTmp = 0x6B,
    TinyKmh = 0x6C,
    TinyMph = 0x6D,
    SmallSbcS = 0x6E,
    SmallSbcH = 0x6F,
    SmallHold = 0x70,

    PrevPage = 0x71, /// Why is this here?
    BoxT = 0x72,
    RadioMast = 0x73,
    SmallGas100Percent = 0x74,
    SmallGas0Percent = 0x75,
    SmallGas25Percent = 0x76,
    RearWiper = 0x77,
    NextTrack = 0x78,
    PrevTrack = 0x79,
    SeekFwd = 0x7A,
    SeekRev = 0x7B,
    Play = 0x7C,
    PlayRev = 0x7D,
    UpArrow = 0x7E,
    DownArrow = 0x7F,
    Ringing = 0x80,
    PhoneBook = 0x81,
    Sms = 0x82,
    VolOn = 0x83,
    VolMute = 0x84,

    DataRx = 0x85,
    DataTx = 0x86,
    Satalite = 0x87,
    Prohibited2 = 0x88,
    Bell = 0x89,
    Clock2 = 0x8A,
    NextTrackSmall = 0x8B,
    Microphone = 0x8C,
    CircleEmpty = 0x8D,
    CircleFull = 0x8E,
    BoxTBold = 0x8F,

    LaneTopDownArrow12 = 0x90,
    LaneTopDownArrow10 = 0x91,
    LaneTopDownArrow9 = 0x92,
    LaneTopDownArrow7 = 0x93,
    LaneTopDownArrow6 = 0x94,
    LaneTopDownArrow6_2 = 0x95,
    LaneTopDownArrow5 = 0x96,
    LaneTopDownArrow3 = 0x97,
    LaneTopDownArrow1 = 0x98,
    LaneTopDownNoArrow = 0x99,

    UpArrow2 = 0x9A,
    DownArrow2 = 0x9B,

    PrevPage2 = 0x9C,

    Decline = 0x9E,
    SmallPlus = 0x9F,
    SmallMinus = 0xA0,

    Selector0To5Level0 = 0xA1,
    Selector0To5Level1 = 0xA2,
    Selector0To5Level2 = 0xA3,
    Selector0To5Level3 = 0xA4,
    Selector0To5Level4 = 0xA5,
    Selector0To5Level5 = 0xA6,

    CurvematicIpsReserve = 0xA7, 
    CurvematicWarningLeftTurn = 0xA8,
    CurvematicWarningRightTurn = 0xA9,
    CurvematicWarningBendyRoadLeft = 0xAA,
    CurvematicWarningBendyRoadRight = 0xAB,
    CurvematicWarningRoundaboutLeft = 0xAC,
    CurvematicWarningRoundaboutRight = 0xAD,
    CurvematicWarningTJunction = 0xAE,
    CurvematicWarningFrost = 0xAF,
    CurvematicInfoLeftTurn = 0xB0,
    CurvematicInfoRightTurn = 0xB1,
    CurvematicInfoBendyRoadLeft = 0xB2,
    CurvematicInfoBendyRoadRight = 0xB3,
    CurvematicInfoRoundaboutLeft = 0xB4,
    CurvematicInfoRoundaboutRight = 0xB5,
    CurvematicInfoTJunction = 0xB6,
    CurvematicInfoFrost = 0xB7,

    GasEmpty = 0xB8,
    GasBarDashes = 0xB9,
    GasBarFullOverlay = 0xBA,
    GasBarEmptyOverlay = 0xBB,
    GasBarLevelMarkers = 0xBC,
}


pub struct LayoutBuilder {
    command_string: String,
    end_coord: Option<(u8, u8)>
}

pub enum Justification {
    Center,
    Right
}

#[repr(u8)]
pub enum YaxisSetting {
    /// Current Y position is the bottom of the text
    BottomToTop = 0,
    /// Current Y position is the center of the text
    Center = 1,
    /// Current Y position is the top of the text
    TopToBottom = 2,
}

bitflags::bitflags! {
    pub struct StatusLineClearFlag: u8 {
        const CLEAR_TEMPERATURE = 1;
        const CLEAR_TRIP = 2;
        const CLEAR_ODOMETER = 4;
        const CLEAR_GEARS = 8;
    }
}

impl LayoutBuilder {
    pub fn new() -> Self {
        Self{ command_string: String::new(), end_coord: None }
    }

    pub fn set_status_line(mut self, flags: StatusLineClearFlag) -> Self {
        self.command_string.push_str(&format!("~I{:1X}", flags));
        self
    }

    pub fn refresh_display(mut self, clear: bool, bg_red: bool) -> Self {
        let cmd = match clear {
            true => "C",
            false => "U",
        };
        let val = match bg_red {
            true => "2",
            false => "0",
        };

        self.command_string.push_str(&format!("~{cmd}{val}"));
        self
    }

    pub fn make_next_element_blink(mut self) -> Self {
        self.command_string.push_str("~F20");
        self
    }

    pub fn new_line(mut self) -> Self {
        self.command_string.push_str("~L");
        self
    }

    pub fn add_text(mut self, s: String) -> Self {
        self.command_string.push_str(&s);
        self
    }

    pub fn add_image(mut self, image_id: u16) -> Self {
        self.command_string.push_str(&format!("~B{image_id:03X}"));
        self
    }

    pub fn add_navi_image(mut self, image_id: u16) -> Self {
        self.command_string.push_str(&format!("~N{image_id:03X}"));
        self
    }

    pub fn set_cursor_pos(mut self, x: u8, y: u8) -> Self {
        self.command_string.push_str(&format!("~P{x:02X}{y:02X}"));
        self
    }

    pub fn set_text_font(mut self, font_id: u8) -> Self {
        self.command_string.push_str(&format!("~G{font_id:01X}"));
        self
    }

    pub fn override_text_height_px(mut self, height: u8) -> Self {
        self.command_string.push_str(&format!("~H{height:02X}"));
        self
    }

    pub fn start_justfy_boundary(mut self, start_coord: (u8, u8), end_coord: (u8, u8)) -> Self {
        let (s_x, s_y) = start_coord;
        self.command_string.push_str(&format!("~P{s_x:02X}{s_y:02X}"));
        //match ty {
        //    Justification::Center => self.command_string.push_str("~Z"),
        //    Justification::Right => self.command_string.push_str("~R"),
        //}
        self.end_coord = Some(end_coord);
        self
    }

    pub fn set_next_text_justification(mut self, ty: Justification) -> Self {
        match ty {
            Justification::Center => self.command_string.push_str("~Z"),
            Justification::Right => self.command_string.push_str("~R"),
        }
        self
    }

    pub fn end_justfy_boundary(mut self) -> Self {
        let (e_x, e_y) = self.end_coord.take().unwrap();
        self.command_string.push_str(&format!("~P{e_x:02X}{e_y:02X}"));
        self
    }

    pub fn cut_pixels_from_next_element(mut self, x_crop: Option<(u8, u8)>, y_crop: Option<(u8, u8)>) -> Self {
        if let Some((s_x, e_x)) = x_crop {
            self.command_string.push_str(&format!("~<{s_x:02X}{e_x:02X}"));
        }
        if let Some((s_y, e_y)) = y_crop {
            self.command_string.push_str(&format!("~-{s_y:02X}{e_y:02X}"));
        }
        self
    }

    /// Sets how the next element on the display's Y position shall be assigned
    pub fn set_next_element_y_positioning_method(mut self, setting: YaxisSetting) -> Self {
        self.command_string.push_str(&format!("~J{:1X}", setting as u8));
        self
    }

    /// Draws a line between 2 coordinates
    /// If `start_pos` is empty, then the line starts from the current
    /// coordinate of the display cursor
    pub fn draw_line(mut self, start_pos: Option<(u8, u8)>, end_pos: (u8, u8)) -> Self {
        if let Some((x, y)) = start_pos {
           self = self.set_cursor_pos(x, y);
        }
        let (e_x, e_y) = end_pos;
        self.command_string.push_str(&format!("~V{e_x:02X}{e_y:02X}"));
        self
    }

    /// Draws a rectange between 2 opposite coordinates
    /// If `start_pos` is empty, then the rectange top left coordinate is set to the current
    /// display cursor position
    /// 
    /// - `end-pos` - The bottom right coordinate of the rectange
    pub fn draw_rect(mut self, start_pos: Option<(u8, u8)>, end_pos: (u8, u8)) -> Self {
        if let Some((x, y)) = start_pos {
           self = self.set_cursor_pos(x, y);
        }
        let (e_x, e_y) = end_pos;
        self.command_string.push_str(&format!("~Q{e_x:02X}{e_y:02X}"));
        self
    }

    pub fn set_art_ring(mut self, enable: bool) -> Self {
        self.command_string.push_str(&format!("~Y{:01X}", enable as u8));
        self
    }

    pub fn finish(self) -> String {
        self.command_string
    }

}

pub struct CDMIsoTp {
    sender: UnboundedSender<KombiCustomCommand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ToneType {
    ShortBeep = 0x01,
    LongBeep = 0x02,
    Chime = 0x03
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ToneRepeatType {
    None = 0x00,
    Fast = 0x01,
    Middle = 0x02,
    Slow = 0x03
}

/**
 * THIS MODULE CANNOT BE USED ON A STOCK IC211
 * 
 * This requires modified firmware, which allows the TeleAID CanIDs
 * to act as a ISOTP buffer, which gives you direct access to the displays format engine,
 * which, allows you to display anything on the IC using format strings
 */

 #[derive(Debug, Clone)]
pub enum KombiCustomCommand {
    StopBuzzer,
    Buzzer(ToneType, ToneRepeatType),
    DisplayText(String, Duration),
    StopDisplay
}

impl CDMIsoTp {
    pub fn new(rt: &Runtime, can: String) -> Self {
        let sock = w211_can::canbus::CanBus::create_isotp_socket_with_name(&can, 0x3E1, 0x1A1, 50, 0);
        
        let (tx, mut rx) = unbounded_channel::<KombiCustomCommand>();
        let txd = tx.clone();
        let h = rt.handle().clone();
        rt.spawn(async move {
            let mut display_open = false;
            let mut cmd_to_ack: Option<KombiCustomCommand> = None;
            let mut last_cmd_time = Instant::now();
            let mut timeout_ms = 0;
            loop {
                tokio::select! {
                    Ok(response) = sock.read_packet().unwrap() => {
                        if response[0] == 0x00 { // ack
                            if cmd_to_ack.is_none() {
                                log::warn!("Custom display proto detected random ack?");
                            }
                            cmd_to_ack = None;
                        } else if response[0] == 0x01 {
                            // Page manually shut
                            display_open = false;
                        } else if response[0] == 0x02 {
                            // TODO - Button events
                        }
                    },
                    Some(command) = rx.recv() => {
                        let mut req_ack = false;
                        let response = match command.clone() {
                            KombiCustomCommand::StopBuzzer => {
                                req_ack = true;
                                sock.write_packet(&[0xFE, 0x10, 0x00, 0x00]).unwrap().await
                            },
                            KombiCustomCommand::Buzzer(tty, trt) => {
                                let p = vec![0xFE, 0x10, tty as u8, trt as u8];
                                req_ack = true;
                                sock.write_packet(&p).unwrap().await
                            },
                            KombiCustomCommand::DisplayText(text, expires_in) => {
                                if !display_open {
                                    // Update buffer
                                    let mut buffer = vec![];
                                    buffer.extend_from_slice(&[0x00, 0x00]);
                                    buffer.extend_from_slice(text.as_bytes());
                                    buffer.push(0x00);
                                    let _ = sock.write_packet(&buffer).unwrap().await;
                                    let _ = sock.write_packet(&[0xFE, 0x00]).unwrap().await;
                                } else {
                                    // Already open, just update buffer
                                    let mut buffer = vec![];
                                    buffer.extend_from_slice(&[0xFE, 0x01, 0x00]);
                                    buffer.extend_from_slice(text.as_bytes());
                                    buffer.push(0x00);
                                    let _ = sock.write_packet(&buffer).unwrap().await;
                                }
                                // Then show display
                                display_open = true;
                                req_ack = true;
                                timeout_ms = expires_in.as_millis();
                                let tx_c = txd.clone();
                                last_cmd_time = Instant::now();
                                h.spawn(async move {
                                    // Triggers cleanup operation
                                    tokio::time::sleep(expires_in).await;
                                    let _ = tx_c.clone().send(KombiCustomCommand::StopDisplay);
                                });
                                Ok(())
                            },
                            KombiCustomCommand::StopDisplay => {
                                if display_open && last_cmd_time.elapsed().as_millis() >= timeout_ms {
                                    display_open = false;
                                    sock.write_packet(&[0x00, 0x00, 0x00]).unwrap().await
                                } else {
                                    Ok(())
                                }
                            },
                        };
                        if response.is_ok() && req_ack { // Only await ack if tx buffer != 0x00
                            cmd_to_ack = Some(command);
                        } else {
                            cmd_to_ack = None;
                        }
                    }
                }
            }
        });
        
        Self {
            sender: tx,
        }
    }

    pub fn sound_buzzer(&self, tone: ToneType, repeat: ToneRepeatType) {
        let _ = self.sender.send(KombiCustomCommand::Buzzer(tone, repeat));
    }

    pub fn stop_buzzer(&self) {
        let _ = self.sender.send(KombiCustomCommand::StopBuzzer);
    }

    pub fn stop_display(&self) {
        let _ = self.sender.send(KombiCustomCommand::StopDisplay);
    }

    pub fn show_display(&self, text: String, duration: u32) {

        let d = match duration {
            u32::MAX => Duration::from_millis(u64::MAX),
            _ => Duration::from_millis(duration as u64)
        };

        let _ = self.sender.send(KombiCustomCommand::DisplayText(text, d));
    }

    pub fn notify_track_change(&self, name: &str) {
        let mut show_test = "".to_string();
        let mut count = 0;
        for c in name.chars() {
            show_test.push(c);
            count+=1;
            if count == 20 {
                if c == ' ' { // Space, don't put a '-'
                    show_test.remove(show_test.len()-1); // Remove space, replace with new line
                    show_test.push_str("~L");
                } else {
                    show_test.push_str("-~L");
                }
                count = 0;
            }
        }

        let display_string = LayoutBuilder::new()
            .set_status_line(StatusLineClearFlag::CLEAR_TRIP | StatusLineClearFlag::CLEAR_TEMPERATURE)
            .refresh_display(true, false)
            .start_justfy_boundary((5, 0), (120, 144))
            .set_next_element_y_positioning_method(YaxisSetting::TopToBottom)
            .set_text_font(1)
            .set_next_text_justification(Justification::Center)
            .add_text("Track changed".into())
            .new_line()
            .set_text_font(0) // Monospace
            //.override_text_height_px(13)
            .new_line()
            .add_text(show_test)
            .new_line()
            .add_image(Image::Disc as u16)
            .end_justfy_boundary()
            .finish();
        
        log::info!("Display cmd: '{display_string}'");
        let _ = self.sender.send(KombiCustomCommand::DisplayText(display_string, Duration::from_millis(2500)));
    }
}