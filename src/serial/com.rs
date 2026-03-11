use super::*;

use std::sync::mpsc::{Sender,Receiver};
use std::thread::JoinHandle;
#[derive(Debug)]
pub struct Com {
    pub tx : Sender<DeviceAction>,
    pub rx : Receiver<Action>,
    pub com_handle : Option<JoinHandle<()>>
}

impl Com {
    pub fn new( port : &str, baudrate : u32, virt : bool ) -> Self {
        if virt { 
            let m = VirtualMachine::new(port, baudrate, 5);
            let (tx,rx) = mpsc::channel();
            let ( tx, com_handle ) = start_device_thread(m, tx);
            Com { tx, rx, com_handle: Some(com_handle) }
        } else {
            let m = Machine::new(port, baudrate, 5);
            let (tx,rx) = mpsc::channel();
            let ( tx, com_handle ) = start_device_thread(m, tx);
            Com { tx, rx, com_handle: Some(com_handle) }
        }
    }
    pub fn recv( self : &mut Self ) -> Action {
        if let Ok(action) = self.rx.try_recv() { action }
        else { Action::None }
    }
    pub fn send( self : &mut Self, action : DeviceAction ) {
        self.tx.send(action)
            .expect("fail to send action")
    }
}
impl Drop for Com {
    fn drop(&mut self) {
        let handle = self.com_handle.take();
        if let Some(handle) = handle {
            stop_device_thread(&self.tx, handle);
        }
    }
}
