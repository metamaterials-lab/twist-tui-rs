pub mod view;
pub mod model;
pub mod update;
pub mod widgets;
pub mod serial;


use crate::model::App;
use crate::update::{events_handler, Update};

use crate::serial::{Com, SerialDevice, VirtualMachine, start_device_thread, stop_device_thread};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
fn main() -> std::io::Result<()> {


    ratatui::run(|terminal| {
        let virt_com = Com::new("/home/luisdfj/ttyUSB0", 115200, true);

        let mut app = App::default();
        //let mut que = Que::default();
        while !app.status.should_quit {
            app.com.send(serial::DeviceAction::Read);

            terminal.draw(|frame| frame.render_widget(&app, frame.area()) )?;
            let action = events_handler()?;
            //que.push(action);
            app.update(action)?;

            let action = app.com.recv();
            app.update(action)?;

            //que.push(action);
        }

        drop(virt_com);
        Ok(())
    })
}


//use crate::update::Action;
//use std::collections::VecDeque;
//use std::time::Duration;
//#[derive(Default)]
//struct Que {
    //actions : VecDeque<Action>
//}
//
//impl Que {
    //pub fn push( self : &mut Self, action : Action ) {
        //match action {
            //Action::None => {},
            //_ => self.actions.push_back(action),
        //}
    //}
    //pub fn pop( self : &mut Self ) -> Action {
        //if let Some(action) = self.actions.pop_front() { action }
        //else { Action::None }
    //}
//}
