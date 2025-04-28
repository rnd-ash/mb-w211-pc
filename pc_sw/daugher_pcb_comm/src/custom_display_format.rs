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
    StationaryHeater = 0x0A,
    CoolantTemp = 0x0B,
    WasherFluid = 0x0C,
    CoolantLevel = 0x0D,
    BulbWarning = 0x0E,
    SeatbeltWarning = 0x0F,
    BatteryBig = 0x10,
    CheckEngine = 0x11,
    P = 0x12,
    Electric = 0x13,
    Key = 0x14,
    Oil = 0x15,
    EpcText = 0x16,
    TrailerHook = 0x17,
    Taxi = 0x18,
    TeleaidCallSOS = 0x19,
    SosText = 0x1A,
    TyrePressureWarn = 0x1B,
    // Teleaid
    TeleaidCallInfo = 0x1C,
    TeleaidCallService = 0x1D,
    TeleaidCallFCD = 0x1E,
    // Service
    Spanner = 0x1F,
    DoubleSpanner = 0x20,
    // ??
    CarTopDownLRSaloon = 0x21,
    CarTopDownLRWagon = 0x22,
    EndCall = 0x23,
    AirmaticSaloon = 0x24,
    AirmaticWagon = 0x25,
    AdaptiveDamper = 0x26,
    SeatTilt = 0x27,
    Seat = 0x28,
    Stop = 0x29,
    TrunkOpenWagon = 0x2A,
    WindShieldHeating = 0x2B,
    Disc = 0x2C,
    SbcS = 0x2D,
    SbcH = 0x2E,
    Plus = 0x2F,
    FuelFilter = 0x30,
    FuelFilterTank = 0x31,
    SunroofSlideClose = 0x32,
    SunroofDownClose = 0x33,
    AirFilter = 0x34,
    Hold = 0x35,
    RefillWithSaloon = 0x36,
    RefillWithWagon = 0x37,
    Prohibited = 0x38,
    BatterySmall = 0x39,
    OilTemperatureSmall = 0x3A,
    // 0x3B - 0x4C - Door icons
    WagonSideView = 0x4D,
    WagonSideViewTrunk = 0x4E,
    // 4F - ??
    WagonSideViewBonnet = 0x50,
    // 51 - 2 dots
    SaloonSideView = 0x52,
    SaloonSideViewTrunk = 0x53,
    // 54 - ??
    SaloonSideViewBonnet = 0x55,
    // 56 - 2 dots
    // DTR images
    DtrDistantCar = 0x57,
    DtrSideCarView = 0x58,
    Dtr100mBar = 0x59,
    Dtr100ydBar = 0x5A,
    Dtr300ftBar = 0x5B,
    DtrSoundOn = 0x5C,
    DtrBarEmpty = 0x5D,
    // Begin settings images
    MinusInCircle = 0x5E,
    PlusInCircle = 0x5F,
    LaneFrontView = 0x60,
    Thermometer = 0x61,
    Clock = 0x62,
    Lim = 0x63,
    TopDownLamp = 0x64,
    FrontLamp = 0x65,
    SideLights = 0x66,
    TaxiLarge = 0x67,
    SeatLarge = 0x68,
    // Images for status line maybe
    TinyCar = 0x69,
    TinyLim = 0x6A,
    TinyDtr = 0x6B,
    TinyTmp = 0x6C,
    TinyKmh = 0x6D,
    TinyMph = 0x6E,
    SmallSbcS = 0x6F,
    SmallSbcH = 0x70,
    SmallHold = 0x71,

    PrevPage = 0x72, /// Why is this here?
    BoxT = 0x73,
    RadioMast = 0x74,
    SmallGas100Percent = 0x75,
    SmallGas0Percent = 0x76,
    SmallGas25Percent = 0x77,
    RearWiper = 0x78,
    NextTrack = 0x79,
    PrevTrack = 0x7A,
    SeekFwd = 0x7B,
    SeekRev = 0x7C,
    Play = 0x7D,
    PlayRev = 0x7E,
    UpArrow = 0x7F,
    DownArrow = 0x80,
    Ringing = 0x81,
    PhoneBook = 0x82,
    Sms = 0x83,
    VolOn = 0x84,
    VolMute = 0x85,

    DataRx = 0x86,
    DataTx = 0x87,
    Satalite = 0x88,
    Prohibited2 = 0x89,
    Bell = 0x8A,
    Clock2 = 0x8B,
    NextTrackSmall = 0x8C,
    Microphone = 0x8D,
    CircleEmpty = 0x8E,
    CircleFull = 0x8F,
    BoxTBold = 0x90,

    LaneTopDownArrow12 = 0x91,
    LaneTopDownArrow10 = 0x92,
    LaneTopDownArrow9 = 0x93,
    LaneTopDownArrow7 = 0x94,
    LaneTopDownArrow6 = 0x95,
    LaneTopDownArrow6_2 = 0x96,
    LaneTopDownArrow5 = 0x97,
    LaneTopDownArrow3 = 0x98,
    LaneTopDownArrow1 = 0x99,
    LaneTopDownNoArrow = 0x9A,

    UpArrow2 = 0x9B,
    DownArrow2 = 0x9C,

    PrevPage2 = 0x9D,
    NextPage2 = 0x9E,

    Decline = 0x9F,
    SmallPlus = 0xA0,
    SmallMinus = 0xA1,

    Selector0To5Level0 = 0xA2,
    Selector0To5Level1 = 0xA3,
    Selector0To5Level2 = 0xA4,
    Selector0To5Level3 = 0xA5,
    Selector0To5Level4 = 0xA6,
    Selector0To5Level5 = 0xA7,

    CurvematicIpsReserve = 0xA8, 
    CurvematicWarningLeftTurn = 0xA9,
    CurvematicWarningRightTurn = 0xAA,
    CurvematicWarningBendyRoadLeft = 0xAB,
    CurvematicWarningBendyRoadRight = 0xAC,
    CurvematicWarningRoundaboutLeft = 0xAD,
    CurvematicWarningRoundaboutRight = 0xAE,
    CurvematicWarningTJunction = 0xAF,
    CurvematicWarningFrost = 0xB0,
    CurvematicInfoLeftTurn = 0xB1,
    CurvematicInfoRightTurn = 0xB2,
    CurvematicInfoBendyRoadLeft = 0xB3,
    CurvematicInfoBendyRoadRight = 0xB4,
    CurvematicInfoRoundaboutLeft = 0xB5,
    CurvematicInfoRoundaboutRight = 0xB6,
    CurvematicInfoTJunction = 0xB7,
    CurvematicInfoFrost = 0xB8,

    GasEmpty = 0xB9,
    GasBarDashes = 0xBA,
    GasBarFullOverlay = 0xBB,
    GasBarEmptyOverlay = 0xBC,
    GasBarLevelMarkers = 0xBD,
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

#[derive(Clone)]
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
    StopDisplay,
    StopDisplayExtern
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
                if let Some(cmd) = &cmd_to_ack {
                    // Wait for ack
                }

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
                        log::debug!("Runnning {command:?}");
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
                                if expires_in.as_millis() as u64 != u64::MAX {
                                    h.spawn(async move {
                                        // Triggers cleanup operation
                                        tokio::time::sleep(expires_in).await;
                                        let _ = tx_c.clone().send(KombiCustomCommand::StopDisplay);
                                    });
                                }
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
                            KombiCustomCommand::StopDisplayExtern => {
                                display_open = false;
                                sock.write_packet(&[0x00, 0x00, 0x00]).unwrap().await
                            }
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
        let _ = self.sender.send(KombiCustomCommand::StopDisplayExtern);
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