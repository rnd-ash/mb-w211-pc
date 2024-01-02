use std::time::Instant;

use super::{AgwPageFsm, build_agw_packet_checksum_in_place};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Heading {
    N = 0,
    S = 1,
    E = 2,
    W = 3,
    NE = 4,
    NW = 5,
    SE = 6,
    SW = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DistanceUnit{
    Km = 0,
    M = 2,
    Mi = 4,
    Ft = 6
}

impl Default for DistanceUnit {
    fn default() -> Self {
        Self::M
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct DistanceDisplay {
    pub show_bar: bool,
    pub show_text: bool,
    pub unit: DistanceUnit,
    pub distance: u16,
    pub bar_fill: u8
}

impl DistanceDisplay {
    pub fn into_buffer(&self) -> [u8; 5] {
        let mut res = [0; 5];
        
        res[0] |= (self.show_bar as u8) & 0b1;
        res[0] |= ((self.show_text as u8) & 0b1) << 1;
        res[0] |= ((self.unit as u8) & 0b111) << 2;
        
        res[2..4].copy_from_slice(&self.distance.to_be_bytes());
        res[4] = self.bar_fill;
        res
    }
}

pub struct NaviPage {
    _last_rotate: Instant,
}

impl NaviPage {
    pub fn new() -> Self {
        Self {
            _last_rotate: Instant::now(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct NaviPageState {
    pub current_road: String,
    pub next_road: String,
    pub meta: Vec<u8>,
    pub distance_display_info: DistanceDisplay
}

impl Default for NaviPageState {
    fn default() -> Self {
        Self { 
            current_road: "NO ROAD".into(), 
            next_road: "NO ROAD".into(), 
            distance_display_info: DistanceDisplay::default(),
            meta: vec![0x17, 0x00, 0x00, 0xCC, 0xCC, 0x01, 0x00, 0x03]
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NaviPageCmd {
    CurrentRoad(String),
    TargetRoad(String),
    CompassHeading(Heading),
    DistanceData(DistanceDisplay),
}

impl AgwPageFsm<NaviPageState, NaviPageCmd> for NaviPage {
    fn build_pkg_20(&self, _state: &NaviPageState) -> Vec<u8> {
        vec![0x04, 0x20, 0x02, 0x11, 0xC2]
    }

    fn build_pkg_24(&self, _state: &NaviPageState) -> Vec<u8> {
        vec![0x04, 0x24, 0x03, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8E]
    }

    fn build_pkg_26(&self, state: &NaviPageState) -> Vec<u8> {
        /*
        let mut buf = vec![0x04, 0x26, 0x01, 0x00, 0x03];
        // First string
        buf.push(3 + state.next_road.len() as u8);
        buf.push(0x10); // Todo format
        buf.extend_from_slice(state.next_road.as_bytes());
        buf.push(0x00);
        // Second string
        buf.push(3 + state.current_road.len() as u8);
        buf.push(0x10); // Todo format
        buf.extend_from_slice(state.current_road.as_bytes());
        buf.push(0x00);
        // Symbol data

        let mut meta_array = vec![];
        meta_array.extend_from_slice(&state.distance_display_info.into_buffer());
        meta_array.extend_from_slice(&state.meta);

        buf.push(2 + meta_array.len() as u8);
        buf.push(0x80);
        buf.extend_from_slice(&meta_array);
        buf.push(0x00);
        build_agw_packet_checksum_in_place(buf)
        */
        let mut buf = vec![0x04, 0x26, 0x01, 0x00, 0x03];
        // First string
        buf.push(3 + state.next_road.len() as u8);
        buf.push(0x10); // Todo format
        buf.extend_from_slice(state.next_road.as_bytes());
        buf.push(0x00);
        // Second string
        buf.push(3 + state.current_road.len() as u8);
        buf.push(0x10); // Todo format
        buf.extend_from_slice(state.current_road.as_bytes());
        buf.push(0x00);
        // Symbol data
        buf.push(2 + state.meta.len() as u8);
        buf.push(0x80);
        buf.extend_from_slice(&state.meta);
        buf.push(0x00);
        build_agw_packet_checksum_in_place(buf)
    }

    fn build_pkg_28(&self, _state: &NaviPageState) -> Vec<u8> {
        vec![] // Not used
    }

    fn build_pkg_29(&self, _state: &NaviPageState) -> Vec<u8> {
        vec![] // Not used
    }

    fn on_page_idle(&mut self, _state: &mut NaviPageState) -> Option<Vec<u8>> {
        None
    }

    fn on_event(&mut self, cmd: NaviPageCmd, state: NaviPageState) -> (NaviPageState, Option<Vec<u8>>) {
        let mut mod_state = state.clone();
        let mut res = None;
        match cmd {
            NaviPageCmd::CurrentRoad(cr) => {
                mod_state.current_road = cr;
            },
            NaviPageCmd::TargetRoad(tr) => {
                mod_state.next_road = tr;
            },
            NaviPageCmd::CompassHeading(heading) => {
                //mod_state.meta[1] &= 0b00011111;
                //mod_state.meta[1] |= (heading as u8) << 5;
            },
            NaviPageCmd::DistanceData(d) => {
                mod_state.distance_display_info = d;
            },
        }

        if  mod_state != state {
            res = Some(self.build_pkg_26(&mod_state));
        }

        (mod_state, res)
    }

    fn name(&self) -> &'static str {
        "NAVI"
    }

    fn get_id(&self) -> u8 {
        0x04
    }
}