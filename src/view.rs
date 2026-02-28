use crate::model::{App, Configs, Data, Status};

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized
    {
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1)
            ])
            .split(area);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Fill(1)
            ])
            .split(vertical_layout[1]);


        self.status.render(vertical_layout[0], buf);
        self.configs.render(horizontal_layout[0], buf);
        self.data.render(horizontal_layout[1], buf);
    }
}

impl Widget for &Status {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let block = Block::bordered()
            .title("Status");
        let content = Paragraph::new("Hello World");

        let block_area = area;
        let content_area = block.inner(block_area);

        block.render(block_area, buf);
        content.render(content_area, buf);
    }
}

impl Widget for &Configs {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let block = Block::bordered()
            .title("Control Panel");
        let content = Paragraph::new("Hello World");

        let block_area = area;
        let content_area = block.inner(block_area);

        block.render(block_area, buf);
        content.render(content_area, buf);
    }
}

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
