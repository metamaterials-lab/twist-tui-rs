use super::*;
use crate::model::{State, Configs};

impl Update<Keys> for Configs {
    fn focus( self : &mut Self ) {
        self.focus = true;
        for i in 0..self.parameters.len() {
            if i == self.select { self.parameters[i].focus(); }
            else { self.parameters[i].unfocus(); }
            self.parameters[i].preview(false);
        }
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
        self.parameters[self.select].unfocus();
        for p in &mut self.parameters {
            p.preview(true);
        }
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action>{
        let res = match key {
            Keys::Right | Keys::Left | Keys::Enter => {
                self.parameters[self.select].update(key)?
            },
            Keys::Up | Keys::Down => {
                move_selection(self, key);
                self.focus();
                Action::None
            },
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

fn move_selection( configs : &mut Configs, key : Keys ) {
    match key {
        Keys::Up => {
            if configs.select > 0 { configs.select -= 1 }
        },
        Keys::Down => {
            let n = configs.parameters.len();
            if configs.select + 1 < n { configs.select += 1 }
        },
        _ => {}
    }
}
