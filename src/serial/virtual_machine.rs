use super::*;
use serialport;

pub struct VirtualMachine {
    port : Box<dyn serialport::SerialPort>,
    x : f32,
    y : f32
}

impl SerialDevice for VirtualMachine {
    fn new( port : &str, baudrate : u32, timeout : u64 ) -> Self {
        VirtualMachine {
            port: serialport::new(
                port,
                baudrate
            )
            .timeout(Duration::from_millis(timeout))
            .open()
            .expect("fail to open virtual machine"),
            x : 0.0,
            y : 0.0,
        }
    }
    fn task( self : &mut Self, _ : DeviceAction ) -> Action {
        Action::None
    }
    fn listen( self : &mut Self ) -> Action {
        let mut buf = [0u8;128];
        if let Ok(t) = self.port.read(&mut buf) {
            if t > 0 {
                match buf[0] as char {
                    'n' => {
                        let buf = format!("{:3.2} DEG\n\r {:3.2} LBIN\n\r", self.x, self.y);
                        self.x += 0.01;
                        self.y += 0.04;
                        self.port.write_all(buf.as_bytes())
                            .expect("fail to write to virtual machine");
                    },
                    _ => {}
                }
            }
        }
        Action::None
    }
}
