use super::*;
use serialport;
use std::time::Duration;
use std::io::{Result,Error,ErrorKind};

#[derive(Debug)]
pub struct Machine {
    port : Box<dyn serialport::SerialPort>
}

impl SerialDevice for Machine {
    fn new( port : &str, baudrate : u32, timeout : u64 ) -> Result<Self> {
        let port = serialport::new(
                port,
                baudrate
            )
            .timeout(Duration::from_millis(timeout))
            .open();
        if let Ok(port) = port {
            Ok( Machine { port } )
        } else {
            Err( Error::new( ErrorKind::NotConnected, "fail to connect to machine" ) )
        }
    }
    fn task( self : &mut Self, action : DeviceAction ) -> Result<Action> {
        let res = match action {
            DeviceAction::Read => {
                let buf = "n\r\n";
                self.port.write(buf.as_bytes())
                    .map_err(|_| Error::new( ErrorKind::Other, "fail to write to machine" ))?;
                Action::None
            },
            DeviceAction::Setup(_) => {
                let buf = "u\r\n";
                self.port.write(buf.as_bytes())
                    .map_err(|_| Error::new( ErrorKind::Other, "fail to write to machine" ))?;
                Action::State(StateAction::ChangeComState(ComState::Stop))
            },
            DeviceAction::Stop => {
                let buf = "s\r\n";
                self.port.write(buf.as_bytes())
                    .map_err(|_| Error::new( ErrorKind::Other, "fail to write to machine" ))?;
                Action::None
            },
            _ => { Action::None }
        };
        Ok(res)
    }
    fn listen( self : &mut Self ) -> Result<Action> {
        let mut buf = [0u8;128];
        let res = if let Ok(t) = self.port.read(&mut buf) {
            if let Ok(buf) = String::from_utf8( Vec::from( &buf[..t] ) ) {
                parse_buffer(buf)
            } else { Action::None }
        } else {
            Action::None
        };
        Ok(res)
    }
}

use crate::update::{Action,StateAction};
use crate::model::status::ComState;
fn parse_buffer( buf : String ) -> Action {
    let bufs : Vec<&str> = buf.split("\r\n").collect();
    if bufs.len() == 3 {
        let tor : Vec<&str> = bufs[0].split(' ').collect();
        let ang : Vec<&str> = bufs[1].split(' ').collect();
        if tor.len() == 2 && ang.len() == 3 {
            if let Ok( x ) = ang[1].parse::<f32>() {
                if let Ok( y ) = tor[0].parse::<f32>() {
                    return Action::State(
                        StateAction::SerialDataUnits(x, ang[2].to_string(), y, tor[1].to_string())
                    );
                }
            }
        }
        //return Action::State(crate::update::StateAction::Msg( format!("{:?}, {:?}", tor, ang) ));
    }
    Action::None
}
