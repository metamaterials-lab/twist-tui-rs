#[derive(Debug)]
pub enum Direction { CW, CCW }
#[derive(Debug)]
pub enum Mode { ZTOR, ZANG }

#[derive(Debug)]
pub struct SerialSetup {
    pub dir : Direction,
    pub mode : Mode,
    pub lim : i16,
    pub spd : f32,
    pub cyc : i16
}

impl SerialSetup {
    pub fn new( dir : &str, mode : &str, lim : i16, spd : f32, cyc : i16 ) -> Self {
        SerialSetup { 
            dir: Direction::new(dir).unwrap(), 
            mode: Mode::new(mode).unwrap(),
            lim, spd, cyc
        }
    }
}

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
