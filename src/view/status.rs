use crate::model::Status;

use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::Block;

impl Widget for &Status {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let style = if self.focus { Color::LightMagenta } else { Color::White };
        let block = Block::bordered()
            .title("Status")
            .border_style( style );

        let layout = Layout::horizontal([
            Constraint::Fill(2),
            Constraint::Fill(2),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
            .horizontal_margin(2)
            .split(block.inner(area));

        block.render(area, buf);

        format!("Port: {}", self.serial_port)
            .render(layout[0], buf);
        self.button.render(layout[1], buf);
        format!("   {:03.2} LBIN   ", self.torque)
            .bold()
            .bg(Color::Cyan)
            .black()
            .render(layout[2], buf);
        format!("   {:03.2} DEG   ", self.angle)
            .bold()
            .bg(Color::Cyan)
            .black()
            .render(layout[3], buf);
    }
}
