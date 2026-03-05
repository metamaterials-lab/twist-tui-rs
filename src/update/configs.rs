use super::*;
use crate::model::{State, Configs};

impl Update<Keys> for Configs {
    fn focus( self : &mut Self ) {
        self.focus = true;
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action>{
        let res = match key {
            Keys::Tab => {
                self.unfocus();
                Action::State(StateAction::ChangeState(State::Status))
            },
            Keys::Q => Action::State(StateAction::ChangeState(State::Quit)),
            _ => Action::None,
        };
        Ok(res)
    }
}
