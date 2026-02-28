#[derive(Debug)]
pub enum Action {
    None,
    Quit
}

use crate::model::App;
impl App {
    fn quit( self : &mut Self ) -> Action {
        self.status.should_quit = true;
        Action::None
    }
    pub fn update( self : &mut Self, action : Action ) -> std::io::Result<Action>{
        let res = match action {
            Action::Quit => self.quit(),
            _ => Action::None
        };
        Ok(res)
    }
}

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
pub fn events_handler() -> std::io::Result<Action> {
    let res = match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => key_handler(key)?,
        _ => Action::None
    };
    Ok(res)
}

fn key_handler( key : KeyEvent ) -> std::io::Result<Action> {
    let res = match key.code {
        KeyCode::Char('q') => Action::Quit,
        _ => Action::None
    };
    Ok(res)
}
