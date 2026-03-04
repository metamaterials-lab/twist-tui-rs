use super::*;
use crate::model::{State, Quit};

impl Update<Keys> for Quit {
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action>{
        let res = match key {
            Keys::Q => Action::State(StateAction::Quit),
            _ => {
                self.focus = false;
                Action::State(StateAction::ChangeState(State::Back))
            },
        };
        Ok(res)
    }
}
