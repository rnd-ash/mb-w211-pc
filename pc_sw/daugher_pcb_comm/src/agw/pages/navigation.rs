use std::time::Instant;

use super::{AgwPageFsm, build_agw_packet_checksum_in_place};

pub struct NaviPage {
    last_rotate: Instant
}

impl NaviPage {
    pub fn new() -> Self {
        Self {
            last_rotate: Instant::now()
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct NaviPageState {
    pub (crate) current_road: String,
    pub (crate) next_road: String,
    pub (crate) meta: Vec<u8>,
}

impl Default for NaviPageState {
    fn default() -> Self {
        Self { 
            current_road: "".into(), 
            next_road: "W211-E55".into(), 
            //
            meta: vec![0x17, 0x00, 0x00, 0xCC, 0xCC, 0x01, 0x00, 0x03]
        }
    }
}

#[derive(Debug, Clone)]
pub enum NaviPageCmd {
    CurrentRoad(String),
    TargetRoad(String),
    CompassHeading(NaviHeading),
}

impl AgwPageFsm<NaviPageState, NaviPageCmd> for NaviPage {
    fn build_pkg_20(&self, _state: &NaviPageState) -> Vec<u8> {
        vec![0x04, 0x20, 0x02, 0x11, 0xC2]
    }

    fn build_pkg_24(&self, _state: &NaviPageState) -> Vec<u8> {
        vec![0x04, 0x24, 0x03, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8E]
    }

    fn build_pkg_26(&self, state: &NaviPageState) -> Vec<u8> {
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

    fn on_page_idle(&mut self, state: &mut NaviPageState) -> Option<Vec<u8>> {
        if self.last_rotate.elapsed().as_millis() > 2000 {
            self.last_rotate = Instant::now();
            let b = state.meta[6].wrapping_add(0x10);
            state.meta[6] = b;
            state.meta[3] = b;
            state.meta[4] = 0xFF-b;
            state.next_road = format!("B[6] = 0x{:02X?}", b);
            Some(self.build_pkg_26(state))
        } else {
            None
        }
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
            NaviPageCmd::CompassHeading(_ch) => {

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NaviHeading {
    S,
    SE,
    SW,
    N,
    NE,
    NW,
    W,
    E,
}