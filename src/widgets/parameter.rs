use ratatui::prelude::*;

use crate::widgets::Selector;


#[derive(Debug)]
pub struct Parameter {
    name : String,
    pub param : Selector,
    pub focus : bool,
}

impl Parameter {
    pub fn new( name : &str, param : Selector ) -> Self {
        Parameter {
            name: name.to_string(),
            param,
            focus: false
        }
    }
}

impl Widget for &Parameter {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let layout = Layout::horizontal([Constraint::Ratio(1, 4), Constraint::Fill(1)])
            .split(area);
        let n_layout = Layout::vertical([Constraint::Length(1)])
            .flex(layout::Flex::Center)
            .split(layout[0]);
        self.name.clone().render(n_layout[0], buf);
        self.param.render(layout[1], buf);
    }

}
