pub mod machine;
pub mod setup;
pub mod virtual_machine;
pub mod com;

pub use self::virtual_machine::VirtualMachine;
pub use self::machine::Machine;
pub use self::com::Com;

pub use self::setup::SerialSetup;
#[derive(Debug)]
pub enum DeviceAction {
    Setup(SerialSetup),
    Read,
    Quit,
}

use crate::update::Action;
pub trait SerialDevice {
    fn new( port : &str, baudrate : u32, timeout : u64 ) -> Self;
    fn task( self : &mut Self, action : DeviceAction ) -> Action;
    fn listen( self : &mut Self ) -> Action;
}

use std::sync::mpsc::{self, Sender};
use std::thread::{self,JoinHandle};
use std::time::Duration;
pub fn start_device_thread<T : SerialDevice + Send + 'static>( mut dev : T, queue : Sender<Action> ) -> (Sender<DeviceAction>, JoinHandle<()>) {
    let (tx,rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        loop {
            if let Ok(action) = rx.try_recv() {
                match action {
                    DeviceAction::Quit => {break},
                    _ => {
                        let res = dev.task(action);
                        match res {
                            Action::None => {},
                            _ => if let Err(_) = queue.send(res) { break },
                        }
                    },
                }
            }
            let res = dev.listen();
            match res {
                Action::None => {},
                _ => if let Err(_) = queue.send(res) { break },
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
    (tx,handle)
}


pub fn stop_device_thread( tx : &Sender<DeviceAction>, handle : JoinHandle<()> ) {
    if let Ok(_) = tx.send( DeviceAction::Quit ) {};
    if let Ok(_) = handle.join() {};
}
