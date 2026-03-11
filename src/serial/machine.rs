use super::*;
use serialport;

#[derive(Debug)]
pub struct Machine {
    port : Box<dyn serialport::SerialPort>
}

impl SerialDevice for Machine {
    fn new( port : &str, baudrate : u32, timeout : u64 ) -> Self {
        Machine {
            port : serialport::new(
                port,
                baudrate
            )
            .timeout(Duration::from_millis(timeout))
            .open()
            .expect("fail to open serial com port")
        }
    }
    fn task( self : &mut Self, action : DeviceAction ) -> Action {
        match action {
            DeviceAction::Read => {
                let buf = "n\n\r";
                self.port.write(buf.as_bytes())
                    .expect("fail to write to serial com");
                Action::None
            },
            _ => { Action::None }
        }
    }
    fn listen( self : &mut Self ) -> Action {
        let mut buf = [0u8;128];
        if let Ok(t) = self.port.read(&mut buf) {
            let buf = String::from_utf8( Vec::from( &buf[..t] ) )
                .expect("fail to parse string");
            let bufs : Vec<&str> = buf.split("\n\r").collect();
            if bufs.len() == 3 {
                let tor : Vec<&str> = bufs[0].split(' ').collect();
                let ang : Vec<&str> = bufs[1].split(' ').collect();
                if tor.len() == 2 && ang.len() == 3 {
                    if let Ok( x ) = ang[1].parse::<f32>() {
                        if let Ok( y ) = tor[0].parse::<f32>() {
                            return Action::State(crate::update::StateAction::SerialData(x,y));
                        }
                    }
                }

                //return Action::State(crate::update::StateAction::Msg( format!("{:?}, {:?}", tor, ang) ));
            }
            Action::None
        } else {
            Action::None
        }
    }
}
