use crate::model::Commands;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

impl Widget for &Commands {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let text = 
            "<Up><Down>".blue() + " Navigation    ".white() +
            "<Tab>".blue() + " Focus    ".white() + 
            "<Right><Left>".green() + " Selection    ".white() +
            "q".red() + " Quit ".white();
        let block = Block::new();
        let content = Paragraph::new(text.centered());

        let block_area = area;
        let content_area = block.inner(block_area);

        block.render(block_area, buf);
        content.render(content_area, buf);
    }
}
