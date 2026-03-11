use super::*;
use serialport;
use std::time::Duration;
use std::io::{Result,Error,ErrorKind};

pub struct VirtualMachine {
    port : Box<dyn serialport::SerialPort>,
    x : f32,
    y : f32,
    i : f32,
    d : f32,
}

impl SerialDevice for VirtualMachine {
    fn new( port : &str, baudrate : u32, timeout : u64 ) -> Result<Self> {
        let port = serialport::new(
                port,
                baudrate
            )
            .timeout(Duration::from_millis(timeout))
            .open();
        if let Ok(port) = port {
            Ok( VirtualMachine { port, x: 0.0, y: 0.0, i : 0.0, d : 0.05 } )
        } else {
            Err( Error::new( ErrorKind::NotConnected, "fail to connect to machine" ) )
        }
    }
    fn task( self : &mut Self, _ : DeviceAction ) -> Result<Action> {
        Ok(Action::None)
    }
    fn listen( self : &mut Self ) -> Result<Action> {
        let mut buf = [0u8;128];
        if let Ok(t) = self.port.read(&mut buf) {
            if t > 0 {
                match buf[0] as char {
                    'n' => {
                        let buf = format!("{:3.2} LBIN\r\n {:3.2} DEG\r\n", self.y, self.x);
                        self.x += self.i * self.d;
                        self.y = (self.x - 1.0).abs().log2();
                        self.port.write_all(buf.as_bytes())
                            .map_err(|_| Error::new(ErrorKind::NotConnected, "fail to send response"))?;
                    },
                    's' => { self.i =  0.0; },
                    'u' => { self.i =  1.0; },
                    'd' => { self.i = -1.0; },
                    'e' => { 
                        if let Ok(b) = String::from_utf8(buf[1..(t-2)].to_vec()) {
                            if let Ok(n) = b.parse::<f32>() {
                                self.d = n;
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        Ok(Action::None)
    }
}
