use super::*;
use std::sync::mpsc::{self, Sender};
use std::thread::{self,JoinHandle};
use std::time::Duration;
use std::io::Result;
pub fn start_device_thread<T : SerialDevice + Send + 'static>( mut dev : T, queue : Sender<Action> ) -> (Sender<DeviceAction>, JoinHandle<Result<()>>) {
    let (tx,rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        loop {
            if let Ok(action) = rx.try_recv() {
                match action {
                    DeviceAction::Quit => {break},
                    _ => {
                        let res = dev.task(action)?;
                        match res {
                            Action::None => {},
                            _ => if let Err(_) = queue.send(res) { break },
                        }
                    },
                }
            }
            let res = dev.listen()?;
            match res {
                Action::None => {},
                _ => if let Err(_) = queue.send(res) { break },
            }
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    });
    (tx,handle)
}


pub fn stop_device_thread( tx : &Sender<DeviceAction>, handle : JoinHandle<Result<()>> ) {
    if let Ok(_) = tx.send( DeviceAction::Quit ) {};
    if let Ok(_) = handle.join() { };
}
