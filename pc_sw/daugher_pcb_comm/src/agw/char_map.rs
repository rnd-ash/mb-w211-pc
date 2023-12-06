
pub enum CharSet {
    Any,
    One,
    Two
}

pub struct CharData {
    pub set: CharSet,
    pub code: u8
}

impl CharData {
    pub const fn new(set: CharSet, code: u8) -> Self {
        Self {
            set, code
        }
    }
}

pub fn char_to_ic_byte(c: char) -> Option<CharData> {
    match c {
        '~' => Some(CharData::new(CharSet::Any, 0x0B)),
        _ => None
    }
}