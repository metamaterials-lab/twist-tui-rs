use ratatui::prelude::*;
use crate::update::{Action, Keys, Update};

pub trait WidgetRef {
    fn render_ref(&self, area: Rect, buf: &mut Buffer);
}
impl <T> WidgetRef for T
    where for<'a> &'a T : Widget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }
}

pub trait Param : Update<Keys> + WidgetRef {}
impl <T> Param for T 
    where T : Update<Keys> + WidgetRef {}


#[derive(Debug)]
pub struct Parameter<P : Param> {
    name : String,
    pub param : P,
    pub focus : bool,
}

impl <P : Param> Parameter<P> {
    pub fn new( name : &str, param : P ) -> Self {
        Parameter {
            name: name.to_string(),
            param,
            focus: false
        }
    }
}

impl <P : Param> Update<Keys> for Parameter<P> {
    fn focus( self : &mut Self ) {
        self.focus = true;
        self.param.focus();
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
        self.param.unfocus();
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action> {
        self.param.update(key)
    }
}

impl <P : Param> Widget for &Parameter<P> {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
            .split(area);
        let n_layout = Layout::vertical([Constraint::Length(1)])
            .flex(layout::Flex::Center)
            .split(layout[0]);
        
        format!("{}{}", if self.focus {'>'} else {' '}, self.name).render(n_layout[0], buf);
        self.param.render_ref(layout[1], buf);
    }
}
