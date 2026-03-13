#[derive(Debug)]
pub enum Cmd {
    Stop,
    Move(Direction),
    Speed(f32),
    ReadData,
    ReadPos,
}

use crate::serial::setup::Direction;
impl Cmd {
    pub fn parse( self ) -> String {
        let cmd = match self {
            Cmd::Stop => "s",
            Cmd::Move(dir) => match dir {
                Direction::CW => "u",
                Direction::CCW => "d",
            },
            Cmd::Speed(_) => "e00.50",
            Cmd::ReadData => "n",
            Cmd::ReadPos => "x",
        };
        cmd.to_string() + "\r\n"
    }
}

use super::Twister;
use std::io::{Result,Error,ErrorKind};
pub fn command( machine : &mut Twister, cmd : Cmd ) -> Result<()> {
    let cmd = cmd.parse();
    machine.port.write(cmd.as_bytes())
        .map_err(|_| Error::new( ErrorKind::Other, "fail to write to machine" ))?;
    Ok(())
}
