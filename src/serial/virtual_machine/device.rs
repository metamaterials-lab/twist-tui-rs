use crate::serial::{SerialDevice,DeviceAction};
use crate::update::Action;

use super::VirtualMachine;
use std::time::Duration;
use std::io::{Result,Error,ErrorKind};
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
                for cmd in buf.split(|&b| b as char == '\n') {
                    commands(self, cmd)?;
                }
            }
        }
        Ok(Action::None)
    }
}

fn commands( vm : &mut VirtualMachine, buf : &[u8] ) -> Result<()> {
    match buf[0] as char {
        'n' => {
            let buf = format!("{:3.2} LBIN\r\n {:3.2} DEG\r\n", vm.y, vm.x);
            vm.x += vm.i * vm.d;
            vm.y = (vm.x.abs() + 1.0).log2();
            vm.port.write_all(buf.as_bytes())
                .map_err(|_| Error::new(ErrorKind::NotConnected, "fail to send response"))?;
        },
        's' => { vm.i =  0.0; },
        'u' => { vm.i =  1.0; },
        'd' => { vm.i = -1.0; },
        'e' => { 
            if let Ok(b) = String::from_utf8(buf[1..6].to_vec()) {
                if let Ok(n) = b.parse::<f32>() {
                    vm.d = n;
                }
            }
        },
        _ => {}
    };
    Ok(())
}
