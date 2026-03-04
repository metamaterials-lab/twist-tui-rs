use super::*;
use crate::model::{State, Configs};

impl Update<Keys> for Configs {
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action>{
        let res = match key {
            Keys::Tab => {
                self.focus = false;
                Action::State(StateAction::ChangeState(State::Status))
            },
            Keys::Q => Action::State(StateAction::ChangeState(State::Quit)),
            _ => Action::None,
        };
        Ok(res)
    }
}
