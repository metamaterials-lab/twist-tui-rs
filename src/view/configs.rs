use crate::model::Configs;

use ratatui::prelude::*;
use ratatui::widgets::Block;

impl Widget for &Configs {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let style = if self.focus { Style::new().blue() } else { Style::new() };
        let block = Block::bordered()
            .title("Control Panel")
            .border_style(style);

        let n = self.parameters.len();
        let layout = Layout::vertical
            ( std::iter::repeat_n(Constraint::Length(1), n) )
            .flex(layout::Flex::Start)
            .split(block.inner(area));
        for i in 0..n {
            self.parameters[i].render(layout[i], buf);
        }
        block.render(area, buf);
    }
}
