use super::*;
use crate::model::{State, Status};

impl Update<Keys> for Status {
    fn focus( self : &mut Self ) {
        self.focus = true;
        self.button.focus();
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
        self.button.unfocus();
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action>{
        let res = match key {
            Keys::Enter => {
                match self.button.state {
                    ComState::Run => Action::State(StateAction::ChangeComState(ComState::Setup)),
                    ComState::Stop => Action::State(StateAction::ChangeComState(ComState::Run)),
                    _ => Action::None
                }
            },
            Keys::Tab => {
                self.unfocus();
                Action::State(StateAction::ChangeState(State::Config))
            },
            Keys::Q => Action::State(StateAction::ChangeState(State::Quit)),
            _ => Action::None,
        };
        Ok(res)
    }
}

