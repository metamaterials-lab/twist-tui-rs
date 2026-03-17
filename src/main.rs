pub mod view;
pub mod model;
pub mod update;
pub mod widgets;
pub mod serial;


use crate::model::App;
use crate::update::{events_handler, Update};

use crate::serial::{Com, VirtualMachine};
fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let virt_com = Com::new::<VirtualMachine>("/home/luisdfj/ttyUSB0", 115200)?;
        let mut app = App::new( "/home/luisdfj/ttyUSB1", 115200 )?;
        while !app.status.should_quit {
            terminal.draw(|frame| frame.render_widget(&app, frame.area()) )?;
            let action = events_handler()?;

            app.update(action)?;
            let actions = app.com.recv();
            for action in actions {
                app.update(action)?;
            }
            app.com.send(serial::DeviceAction::Read)?;
        }
        drop(virt_com);
        Ok(())
    })
}
