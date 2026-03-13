pub mod device;

use serialport;
pub struct VirtualMachine {
    port : Box<dyn serialport::SerialPort>,
    x : f32,
    y : f32,
    i : f32,
    d : f32,
}

