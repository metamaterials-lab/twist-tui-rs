#[derive(Debug)]
pub enum Action {
    None,
    Quit,
    ChangeState(State),
}

use crate::model::{App, State};
impl App {
    fn quit( self : &mut Self ) -> Action {
        self.status.should_quit = true;
        Action::None
    }
    fn change_state( self : &mut Self, state : State ) -> Action {
        match state {
            State::Config => {
                self.configs.focus = true;
                self.status.focus = false;
            },
            State::Status => {
                self.configs.focus = false;
                self.status.focus = true;
            },
            _ => {},
        };
        self.state = state;
        Action::None
    }
    pub fn update( self : &mut Self, action : Action ) -> std::io::Result<Action>{
        let res = match action {
            Action::Quit => self.quit(),
            Action::ChangeState(state) => self.change_state(state),
            _ => Action::None
        };
        Ok(res)
    }
}

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
pub fn events_handler( state : &State ) -> std::io::Result<Action> {
    let res = match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => key_handler(key, state)?,
        _ => Action::None
    };
    Ok(res)
}

fn key_handler( key : KeyEvent, state : &State ) -> std::io::Result<Action> {
    let res = match state {
        State::Config => { config_key_handler(key)? },
        State::Status => { status_key_handler(key)? },
        State::Quit => { quit_key_handler(key)? },
    };
    Ok(res)
}

fn config_key_handler( key : KeyEvent ) -> std::io::Result<Action> {
    let res = match key.code {
        KeyCode::Char('q') => { Action::ChangeState(State::Quit) },
        KeyCode::Tab => { Action::ChangeState(State::Status) },
        _ => Action::None,
    };
    Ok(res)
}

fn status_key_handler( key : KeyEvent ) -> std::io::Result<Action> {
    let res = match key.code {
        KeyCode::Char('q') => { Action::ChangeState(State::Quit) },
        KeyCode::Tab => { Action::ChangeState(State::Config) },
        _ => Action::None,
    };
    Ok(res)
}

fn quit_key_handler( key : KeyEvent ) -> std::io::Result<Action> {
    let res = match key.code {
        KeyCode::Char('q') => { Action::Quit },
        _ => Action::None,
    };
    Ok(res)
}
