pub mod cmd;
pub mod device;
pub mod utils;

use serialport;
use crate::serial::SerialSetup;

#[derive(Debug)]
pub struct Twister {
    port : Box<dyn serialport::SerialPort>,
    setup : SerialSetup,
    cyc : i16,
    dir : bool,
    record : bool,
}
