#[derive(Debug)]
pub enum Units {
    DEG,
    REV
}

#[derive(Debug)]
pub enum Cmd {
    Stop,
    Move(Direction),
    Speed(f32),
    ReadData,
    ReadPos,
    Zero,
    Manual,
    Units(Units),
    ProgSpeed,
    MinSpeed,
    MaxSpeed,
}

use crate::serial::setup::Direction;
impl Cmd {
    pub fn parse( self ) -> String {
        let cmd = match self {
            Cmd::Move(dir) => match dir {
                Direction::CW  => format!("u"),
                Direction::CCW => format!("d"),
            },
            Cmd::Units(u) => match u {
                Units::REV => format!("b"),
                Units::DEG => format!("i"),
            },
            Cmd::Speed(s) => format!("e{:05.2}",s),
            Cmd::ReadData => format!("n"),
            Cmd::ReadPos  => format!("x"),
            Cmd::Stop     => format!("s"),
            Cmd::Zero     => format!("z"),
            Cmd::Manual   => format!("m"),
            Cmd::ProgSpeed=> format!("o"),
            Cmd::MaxSpeed => format!("j"),
            Cmd::MinSpeed => format!("k"),
        };
        cmd + "\r\n"
    }
}

use super::Twister;
use std::io::{Result,Error,ErrorKind};
pub fn command( machine : &mut Twister, cmd : Cmd, delay : Option<u64> ) -> Result<()> {
    let cmd = cmd.parse();
    machine.port.write(cmd.as_bytes())
        .map_err(|_| Error::new( ErrorKind::Other, "fail to write to machine" ))?;
    std::thread::sleep(std::time::Duration::from_millis(delay.unwrap_or(1)));
    Ok(())
}
