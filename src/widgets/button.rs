use ratatui::{prelude::*, style::Styled};

use crate::update::{Action, Keys, Update};

#[derive(Default,Debug)]
pub enum State {
    #[default]
    Run,
    Setup,
    Stop
}

#[derive(Default,Debug)]
pub struct Button {
    pub state : State,
    pub focus : bool
}

impl Update<Keys> for Button {
    fn focus( self : &mut Self ) {
        self.focus = true;
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action> {
        if let Keys::Enter = key {
            let res = match self.state {
                State::Run => {
                    self.state = State::Setup;
                    Action::None
                },
                State::Setup => {
                    self.state = State::Stop;
                    Action::None
                },
                State::Stop => {
                    self.state = State::Run;
                    Action::None
                }
            };
            return Ok(res);
        } else { 
            return Ok(Action::None);
        }
    }
}

impl Widget for &Button {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {

        let style = if self.focus { Style::new().reversed() } else { Style::new() };

        format!( "| {:?} |", self.state )
            .bold()
            .style( style )
            .render(area, buf);
    }
}
