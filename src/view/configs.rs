use crate::model::Configs;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

impl Widget for &Configs {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let style = if self.focus { Style::new().blue() } else { Style::new() };
        let block = Block::bordered()
            .title("Control Panel")
            .border_style(style);
        let content = Paragraph::new("Hello World");

        let block_area = area;
        let content_area = block.inner(block_area);

        block.render(block_area, buf);
        content.render(content_area, buf);
    }
}
