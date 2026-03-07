pub mod machine;
pub use self::machine::{Direction,Mode};

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
            lim,
            spd,
            cyc
        }
    }

}
