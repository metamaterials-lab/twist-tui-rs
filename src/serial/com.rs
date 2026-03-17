use super::*;
use crate::serial::utils::*;

use std::sync::mpsc::{Sender,Receiver,self};
use std::thread::JoinHandle;
use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Com {
    pub tx : Sender<DeviceAction>,
    pub rx : Receiver<Action>,
    pub com_handle : Option<JoinHandle<Result<()>>>
}

impl Com {
    pub fn new<T : SerialDevice + Send + 'static>( port : &str, baudrate : u32 ) -> Result<Self> {
        let m = T::new(port, baudrate, 5)?;
        let (tx,rx) = mpsc::channel();
        let ( tx, com_handle ) = start_device_thread(m, tx);
        Ok( Com { tx, rx, com_handle: Some(com_handle) } )
    }
    pub fn recv( self : &mut Self ) -> Vec<Action>{
        let mut res = Vec::new();
        loop {
            if let Ok(action) = self.rx.try_recv() {
                res.push(action);
            } else { break }
        }
        res
    }
    pub fn send( self : &mut Self, action : DeviceAction ) -> Result<()> {
        if let DeviceAction::None = action { return Ok(()) }
        self.tx.send(action)
            .map_err(|_| Error::new(ErrorKind::Other,"fail to send commands"))?;
        Ok(())
    }
}
impl Drop for Com {
    fn drop(&mut self) {
        if let Some(handle) = self.com_handle.take() {
            stop_device_thread(&self.tx, handle);
        }
    }
}
