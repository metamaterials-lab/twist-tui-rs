use ratatui::{prelude::*, style::Styled, widgets::{Block, Gauge, Paragraph}};
use crate::update::{Action, Keys, Update};

pub trait Num : PartialOrd + std::ops::Add<Output=Self> + std::ops::Sub<Output=Self> + std::fmt::Display + Copy + Clone
{
    fn as_f32( self ) -> f32;
}

impl <T> Num for T
where T : PartialOrd + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Display + Copy + Clone,
      f32 : From<T>
{
    fn as_f32( self ) -> f32 {
        f32::from( self )
    }
}

#[derive(Default,Debug)]
pub struct Numeric<T : Num> {
    pub val : T,
    pub min : T,
    pub max : T,
    pub inc : T,
    pub focus : bool,
}

impl <T: Num> Numeric<T> {
    pub fn new( val : T, min : T, max : T, inc : T ) -> Self {
        Numeric { val, min, max, inc, focus: false }
    }
    pub fn percent( self : &Self ) -> u16 {
        ( 100f32 * ( self.val - self.min ).as_f32() / ( self.max - self.min ).as_f32() ) as u16
    }
}

impl <T : Num> Update<Keys> for Numeric<T> {
    fn focus( self : &mut Self ) {
        self.focus = true;
    }
    fn unfocus( self : &mut Self ) {
        self.focus = false;
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action> {
        match key {
            Keys::Left => {
                self.val = if self.val < self.min + self.inc { self.min }
                else { self.val - self.inc }
            },
            Keys::Right => {
                self.val = if self.val + self.inc > self.max { self.max }
                else { self.val + self.inc }
            },
            _ => {}
        };
        Ok(Action::None)
    }
}

impl <T : Num> Widget for &Numeric<T> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where Self: Sized {
        
        //let widget = Paragraph::new( format!("{}", self.val) )
            //.red()
            //.bg(Color::Green)
            //.centered();
        //widget.render(area, buf);
        Gauge::default()
            .percent(self.percent())
            .label( format!("{:03.2}", self.val) )
            .style(Style::new().bold().bg(Color::Red))
            .render(area, buf);

    }
}
