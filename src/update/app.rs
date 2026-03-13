use super::*;
use crate::model::{App, State};
use std::io::{Error,ErrorKind};

impl Update<Action> for App {
    fn focus( self : &mut Self ) {}
    fn unfocus( self : &mut Self ) {}
    fn update( self : &mut Self, action : Action ) -> std::io::Result<Action>{
        let action = if let Action::Key(key) = action {
            match self.state {
                State::Status => { self.status.update(key)? },
                State::Config => { self.configs.update(key)? },
                State::Quit => { self.quit.update(key)? },
                _ => { Action::None }
            }
        } else { action };

        let action = if let Action::State(action) = action {
            match action {
                StateAction::Quit => quit( self ),
                StateAction::ChangeState(state) => change_state( self, state ),
                StateAction::ChangeComState(state) => change_com_state( self, state ),
                StateAction::SerialRecord(x,y) => {
                    self.status.change_angle(x, None);
                    self.status.change_torque(y, None);
                    self.data.push( x,y );
                    Action::None
                },
                StateAction::SerialData(x,xu,y,yu) => {
                    self.status.change_angle(x, Some(&xu));
                    self.status.change_torque(y, Some(&yu));
                    Action::None
                },
                StateAction::Msg(s) => {
                    return Err( Error::new(ErrorKind::Other, s) );
                }
            }
        } else { action };

        let res = if let Action::Com(action) = action {
            self.com.send(action)?;
            Action::None
        } else { Action::None };
        Ok(res)
    }
}

fn quit( app : &mut App ) -> Action {
    app.status.should_quit = true;
    Action::None
}

fn change_com_state( app : &mut App, state : ComState ) -> Action {
    match state {
        ComState::Run => {
            app.lock = false;
            app.status.change_run();
            Action::Com(DeviceAction::Stop)
        },
        ComState::Setup => {
            app.lock = true;
            app.status.change_setup();
            Action::Com(DeviceAction::Setup(app.configs.serial_setup()))
        },
        ComState::Stop => {
            app.status.change_stop();
            Action::Com(DeviceAction::None)
        },
    }
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
            if app.lock {
                focus_state(app, State::Status)
            } else {
                app.configs.focus();
                State::Config
            }
        },
        State::Status => {
            app.status.focus();
            State::Status
        },
        State::Quit => {
            app.quit.focus();
            State::Quit
        },
        State::Back => {
            focus_state(app, app.prev_state)
        }
    }
}
