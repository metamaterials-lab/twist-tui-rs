use crate::model::{App, Commands, Configs, Data, Quit, State, Status};

use ratatui::prelude::*;
use ratatui::widgets::{Block, Clear, Paragraph};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized
    {
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(1)
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
        self.commands.render(vertical_layout[2], buf);

        match self.state {
            State::Quit => self.quit.render(area, buf),
            _ => {}
        };
    }
}

impl Widget for &Status {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let style = if self.focus { Style::new().blue() } else { Style::new() };
        let block = Block::bordered()
            .title("Status")
            .border_style( style );
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

impl Widget for &Quit {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5)])
            .flex(layout::Flex::Center)
            .split(area);
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30)])
            .flex(layout::Flex::Center)
            .split(layout[0]);
        Clear.render(layout[0], buf);
        let block = Block::bordered()
            .title("Do you want to exit?");
        let text = "  No  ".red().bg(Color::White) + "    ".white() + "  Yes  ".white();
        let content = Paragraph::new(text.centered());
        let content_area = Layout::vertical([Constraint::Length(2)])
            .flex(layout::Flex::Center)
            .split(block.inner(layout[0]));
        block.render(layout[0], buf);
        content.render(content_area[0], buf);
    }
}
