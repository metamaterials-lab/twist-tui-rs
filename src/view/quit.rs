use crate::model::Quit;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Clear};

impl Widget for &Quit {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        if !self.focus { return }

        let l = Layout::vertical([Constraint::Length(5)])
            .flex(layout::Flex::Center)
            .split(area);
        let l = Layout::horizontal([Constraint::Percentage(30)])
            .flex(layout::Flex::Center)
            .split(l[0]);
        let area = l[0];

        Clear.render(area, buf);

        let block = Block::bordered()
            .border_style( Style::new().red() )
            .title("Do you want to exit?");
        self.foo.render(block.inner(area), buf);
        block.render(area, buf);
    }
}
