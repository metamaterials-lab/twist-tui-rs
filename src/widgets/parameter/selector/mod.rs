pub mod option;
use self::option::Opt;
use crate::update::{Update,Action,Keys};
use ratatui::prelude::*;


#[derive(Debug)]
pub struct Selector {
    pub options : Vec<Opt>,
    pub selection : usize,
    pub hover : usize,
    pub focus : bool,
}

impl Selector {
    pub fn new<const N : usize>( options : [&str; N] ) -> Self {
        let mut res = Selector {
            options: Vec::from(options.map(|s| Opt::new(s))),
            selection: 0,
            hover : 0,
            focus : false
        };
        res.apply_selections();
        res
    }
    pub fn apply_selections( self : &mut Self ) {
        for i in 0..self.options.len() {
            self.options[i].select = if self.selection == i {true} else {false};
            self.options[i].hover = if self.hover == i {true} else {false};
            self.options[i].focus = self.focus;
        }
    }
}

impl Update<Keys> for Selector {
    fn focus( self : &mut Self ) {
        self.focus = true;
        self.apply_selections();
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
        self.apply_selections();
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action> {
        let res = match key {
            Keys::Left => {
                if self.hover > 0 { self.hover -= 1; }
                Action::None
            },
            Keys::Right => {
                if self.hover + 1 < self.options.len() { self.hover += 1; } 
                Action::None
            },
            Keys::Enter => {
                self.selection = self.hover;
                Action::None
            },
            _ => Action::None,
        };
        self.apply_selections();
        Ok(res)
    }
}

impl Widget for &Selector {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let n = self.options.len();
        let area = Layout::vertical([Constraint::Fill(1)])
            .flex(layout::Flex::Center)
            .split(area);
        let layout = Layout::horizontal
            (std::iter::repeat_n(Constraint::Ratio(1, (n + 1) as u32),n))
            .flex(layout::Flex::SpaceAround)
            .split(area[0]);
        for i in 0..n {
            self.options[i].render(layout[i], buf);
        }
    }
}
