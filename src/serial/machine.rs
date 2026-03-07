#[derive(Debug)]
pub enum Direction { CW, CCW }
#[derive(Debug)]
pub enum Mode { ZTOR, ZANG }

impl Direction {
    pub fn new( s : &str ) -> Option<Self> {
        match s {
            "CW" => Some( Direction::CW ),
            "CCW" => Some( Direction::CCW ),
            _ => None
        }
    }
}
impl Mode {
    pub fn new( s : &str ) -> Option<Self> {
        match s {
            "0-TOR" => Some( Mode::ZTOR ),
            "0-ANG" => Some( Mode::ZANG ),
            _ => None
        }
    }
}
