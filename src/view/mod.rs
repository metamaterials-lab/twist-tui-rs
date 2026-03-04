pub mod commands;
pub mod configs;
pub mod data;
pub mod quit;
pub mod status;

use crate::model::App;

use ratatui::prelude::*;

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
        self.quit.render(area, buf);
    }
}
