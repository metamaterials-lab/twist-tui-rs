use crate::model::Data;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

impl Widget for &Data {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let block = Block::bordered()
            .title("Data");
        let content = Paragraph::new("Hello World");

        let block_area = area;
        let content_area = block.inner(block_area);

        block.render(block_area, buf);
        content.render(content_area, buf);
    }
}
