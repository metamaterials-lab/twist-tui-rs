pub mod machine;
pub mod setup;
pub mod virtual_machine;
pub mod com;
pub mod utils;

pub use self::virtual_machine::VirtualMachine;
pub use self::machine::Machine;
pub use self::com::Com;

pub use self::setup::SerialSetup;

#[derive(Debug)]
pub enum DeviceAction {
    None,
    Stop,
    Setup(SerialSetup),
    Read,
    Quit,
}

use crate::update::Action;
pub trait SerialDevice
where Self : Sized
{
    fn new( port : &str, baudrate : u32, timeout : u64 ) -> std::io::Result<Self>;
    fn task( self : &mut Self, action : DeviceAction ) -> std::io::Result<Action>;
    fn listen( self : &mut Self ) -> std::io::Result<Action>;
}

