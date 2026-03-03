pub mod view;
pub mod model;
pub mod update;

use crate::model::App;
use crate::update::events_handler;
fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut app = App::default();
        let mut que = Que::default();
        while !app.status.should_quit {
            terminal.draw(|frame| frame.render_widget(&app, frame.area()) )?;
            let action = events_handler( &app.state )?;
            que.push(action);
            let action = app.update(que.pop())?;
            que.push(action);
        }
        Ok(())
    })
}


use crate::update::Action;
use std::collections::VecDeque;
#[derive(Default)]
struct Que {
    actions : VecDeque<Action>
}

impl Que {
    pub fn push( self : &mut Self, action : Action ) {
        match action {
            Action::None => {},
            _ => self.actions.push_back(action),
        }
    }
    pub fn pop( self : &mut Self ) -> Action {
        if let Some(action) = self.actions.pop_front() { action }
        else { Action::None }
    }
}
