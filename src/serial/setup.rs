#[derive(Debug,Clone, Copy)]
pub enum Direction { CW, CCW }
#[derive(Debug,Clone, Copy)]
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
    pub fn sign<T:From<i8>>( self : &Self ) -> T {
        match self {
            Direction::CCW => 1.into(),
            Direction::CW => (-1).into(),
        }
    }
}
impl std::ops::Not for Direction {
    type Output = Direction;
    fn not(self) -> Self::Output {
        match self {
            Direction::CW => Direction::CCW,
            Direction::CCW => Direction::CW,
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
