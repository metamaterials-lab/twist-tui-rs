use ratatui::{prelude::*, widgets::{Block, Paragraph}};

#[derive(Debug,Default)]
pub struct Opt {
    name : String,
    pub focus : bool,
    pub select : bool,
    pub hover : bool,
}

impl Opt {
    pub fn new( name : &str ) -> Self {
        Opt { name: name.to_string(), ..Opt::default() }
    }
}

impl Widget for &Opt {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {

        let modifier = if self.select { Modifier::BOLD } else { Modifier::empty() };
        let style = if self.focus && self.hover { Color::White } else { Color::DarkGray };

        let l = Layout::vertical([Constraint::Length(3)])
            .flex(layout::Flex::Center)
            .split(area);
        let area = l[0];
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style( style );
        let content = Paragraph::new(self.name.clone())
            .style( style )
            .style( modifier )
            .centered();
        content.render(block.inner(area), buf);
        block.render(area, buf);
    }
}
