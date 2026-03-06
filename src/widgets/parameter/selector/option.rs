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

fn selected_styles( flag : bool ) -> Style {
    if flag { Style::new().white().bold() }
    else { Style::new().dark_gray() }
}
fn hover_styles( flag : bool, style : Style ) -> Style {
    if flag { style.reversed() }
    else { style }
}

impl Widget for &Opt {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {

        let style = selected_styles(self.select);
        let style = hover_styles(self.focus && self.hover, style);

        let l = Layout::vertical([Constraint::Length(1)])
            .flex(layout::Flex::Center)
            .split(area);
        let area = l[0];
        let block = Block::new()
            .style(style);
        let content = Paragraph::new(self.name.clone())
            .centered();
        content.render(block.inner(area), buf);
        block.render(area, buf);
    }
}
