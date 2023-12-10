use std::{time::{Duration, Instant}, collections::VecDeque};

use w211_can::socketcan_isotp::IsoTpSocket;

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

impl LayoutBuilder {
    pub fn new() -> Self {
        Self{ command_string: String::new(), end_coord: None }
    }

    pub fn clear_lower_display(mut self) -> Self {
        self.command_string.push_str(&format!("~I0"));
        self
    }

    pub fn clear_display(mut self) -> Self {
        self.command_string.push_str(&format!("~C0"));
        self
    }

    pub fn clear_display_red_bg(mut self) -> Self {
        self.command_string.push_str(&format!("~C2"));
        self
    }

    pub fn set_normal_bg(mut self) -> Self {
        todo!()
    }

    pub fn set_red_bg(mut self) -> Self {
        todo!()
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

    pub fn start_justfy_boundary(mut self, ty: Justification, start_coord: (u8, u8), end_coord: (u8, u8)) -> Self {
        let (s_x, s_y) = start_coord;
        self.command_string.push_str(&format!("~P{s_x:02X}{s_y:02X}"));
        match ty {
            Justification::Center => self.command_string.push_str("~Z"),
            Justification::Right => self.command_string.push_str("~R"),
        }
        self.end_coord = Some(end_coord);
        self
    }

    pub fn end_justfy_boundary(mut self) -> Self {
        let (e_x, e_y) = self.end_coord.take().unwrap();
        self.command_string.push_str(&format!("~P{e_x:02X}{e_y:02X}"));
        self
    }

    pub fn finish(self) -> String {
        self.command_string
    }

}

pub struct Popup {
    duration: u32,
    fmt_str: String,
}

pub struct CDMIsoTp {
    handler: IsoTpSocket,
    display_open: bool,
    show_time: Instant,
    popup: Popup
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
            handler: w211_can::canbus::CanBus::create_isotp_socket_with_name(&can, 0x3E1, 0x1A1, 40, 8),
            display_open: false,
            popup: Popup { duration: 0, fmt_str: String::new() },
            show_time: Instant::now()
        }
    }

    fn update_buffer(&mut self, s: &str) {
        let mut buffer = vec![0x00, 0x00];
        buffer.extend_from_slice(s.as_bytes());
        buffer.push(0x00);
        let _ = self.handler.write(&buffer); // Write to string buffer before show
    }

    fn stop_display(&mut self) {
        let _ = self.handler.write(&[0x00, 0x00, 0x00]); // Stop processing
        self.display_open = false;
    }

    fn show_display(&mut self) {
        let _ = self.handler.write(&[0xFE]); // Show screen
        self.show_time = Instant::now();
        self.display_open = true;
    }

    pub fn update(&mut self) {
        if self.display_open {
            if self.popup.duration > self.show_time.elapsed().as_millis() as u32 {
                self.stop_display();
            }
        }
    }

    pub fn notify_track_change(&mut self, name: &str) {
        // IMAGES:
        // 90-99 Small lane arrows with directions?
        // 88 - Lap 
        // 87 - GPS
        // 86 - Data Rx
        // 85 - Data Tx
        // 84 - Mute
        // 83 - Unmute
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
            .clear_lower_display()
            .clear_display()
            .start_justfy_boundary(Justification::Center, (5, 0), (120, 144))
            .set_text_font(1)
            .add_text("Track changed".into())
            .set_text_font(0)
            .override_text_height_px(13)
            .new_line()
            .new_line()
            .add_text(show_test)
            .new_line()
            .add_image(0x2B)
            .end_justfy_boundary()
            .finish();
        
        self.popup.duration = 2000;
        self.update_buffer(&display_string);
        self.popup.fmt_str = display_string;
        if !self.display_open {
            std::thread::sleep(Duration::from_millis(40));
            self.show_display();
        }
    }
}