use crate::serial::{DeviceAction,SerialDevice,Twister};
use std::io::{Result,Error,ErrorKind};
use std::time::Duration;
use crate::model::configs::Configs;
use crate::update::{Action,StateAction};
use crate::model::status::ComState;
use super::cmd::{Cmd,command,Units};
use super::utils::control;

impl SerialDevice for Twister {
    fn new( port : &str, baudrate : u32, timeout : u64 ) -> Result<Self> {
        let port = serialport::new(
                port,
                baudrate
            )
            .timeout(Duration::from_millis(timeout))
            .open();
        if let Ok(port) = port {
            Ok( Twister { port, setup: Configs::default().serial_setup(),
            cyc: 0, dir: false, record:false } )
        } else {
            Err( Error::new( ErrorKind::NotConnected, "fail to connect to machine" ) )
        }
    }
    fn task( self : &mut Self, action : DeviceAction ) -> Result<Action> {
        let res = match action {
            DeviceAction::Read => {
                command(self, Cmd::ReadData,None)?;
                Action::None
            },
            DeviceAction::Setup(setup) => {
                let d = Some(50);
                command(self, Cmd::Stop,d)?;
                command(self, Cmd::Manual,d)?;
                command(self, Cmd::ProgSpeed,d)?;
                command(self, Cmd::Units(Units::DEG),d)?;
                command(self, Cmd::Speed(setup.spd),d)?;
                command(self, Cmd::Zero,d)?;
                command(self, Cmd::Move(setup.dir),None)?;
                self.setup = setup;
                self.record = true;
                self.dir = false;
                self.cyc = 0;
                Action::State(StateAction::ChangeComState(ComState::Stop))
            },
            DeviceAction::Stop => {
                command(self, Cmd::Stop,Some(20))?;
                self.record = false;
                Action::None
            },
            DeviceAction::ChangeDir => {
                let d = Some(20);
                command(self, Cmd::Stop,d)?;
                command(self, Cmd::Move(if self.dir { !self.setup.dir } else { self.setup.dir }),None)?;
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
                control(self, buf)
                    .map_or(Action::None, |x| x)
            } else { Action::None }
        } else {
            Action::None
        };
        Ok(res)
    }
}
