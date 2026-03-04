use super::*;
use crate::model::{State, Status};

impl Update<Keys> for Status {
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action>{
        let res = match key {
            Keys::Tab => {
                self.focus = false;
                Action::State(StateAction::ChangeState(State::Config))
            },
            Keys::Q => Action::State(StateAction::ChangeState(State::Quit)),
            _ => Action::None,
        };
        Ok(res)
    }
}

