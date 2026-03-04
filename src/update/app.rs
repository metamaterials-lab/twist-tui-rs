use super::*;
use crate::model::{App, State};

impl Update<Action> for App {
    fn update( self : &mut Self, action : Action ) -> std::io::Result<Action>{
        let action = if let Action::Key(key) = action {
            match self.state {
                State::Status => { self.status.update(key)? },
                State::Config => { self.configs.update(key)? },
                State::Quit => { self.quit.update(key)? },
                _ => { Action::None }
            }
        } else { action };

        let res = if let Action::State(action) = action {
            match action {
                StateAction::Quit => quit( self ),
                StateAction::ChangeState(state) => change_state( self, state ),
            }
        } else { Action::None };
        Ok(res)
    }
}

fn quit( app : &mut App ) -> Action {
    app.status.should_quit = true;
    Action::None
}

fn change_state( app : &mut App, state : State ) -> Action {
    let state = focus_state(app, state);
    app.prev_state = app.state;
    app.state = state;
    Action::None
}

fn focus_state( app : &mut App, state : State ) -> State {
    match state {
        State::Config => {
            app.configs.focus = true;
            State::Config
        },
        State::Status => {
            app.status.focus = true;
            State::Status
        },
        State::Quit => {
            app.quit.focus = true;
            State::Quit
        },
        State::Back => {
            focus_state(app, app.prev_state)
        }
    }
}
