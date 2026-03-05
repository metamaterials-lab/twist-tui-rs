use super::*;
use crate::model::{State, Quit};

impl Update<Keys> for Quit {
    fn focus( self : &mut Self ) {
        self.focus = true;
        self.selection.focus();
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
        self.selection.unfocus();
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action>{
        let res = match key {
            Keys::Left | Keys::Right => self.selection.update(key)?,
            Keys::Enter => {
                self.selection.update(key)?;
                if self.selection.selection == 1 {
                    Action::State(StateAction::Quit)
                } else {
                    self.unfocus();
                    Action::State(StateAction::ChangeState(State::Back))
                }
            },
            _ => {
                self.unfocus();
                Action::State(StateAction::ChangeState(State::Back))
            },
        };
        Ok(res)
    }
}
