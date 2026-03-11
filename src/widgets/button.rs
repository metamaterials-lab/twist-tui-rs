use ratatui::prelude::*;

use crate::update::{Action, Keys, Update};


#[derive(Default,Debug)]
pub struct Button<T : std::fmt::Debug> {
    pub state : T,
    pub focus : bool
}

impl <T : std::fmt::Debug> Update<Keys> for Button<T> {
    fn focus( self : &mut Self ) {
        self.focus = true;
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
    }
    fn update( self : &mut Self, _ : Keys ) -> std::io::Result<Action> {
        Ok(Action::None)
    }
}

impl <T : std::fmt::Debug> Widget for &Button<T> {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {

        let style = Style::new().black().bg(Color::Cyan);
        let style = if self.focus { style.bg(Color::LightCyan) } else { style };
        format!( " {:^11} ",format!("{:?}", self.state) )
            .bold()
            .style( style )
            .render(area, buf);
    }
}
