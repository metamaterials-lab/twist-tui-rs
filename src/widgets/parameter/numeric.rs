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
    pub preview : bool,
}

impl <T: Num> Numeric<T> {
    pub fn new( val : T, min : T, max : T, inc : T ) -> Self {
        Numeric { val, min, max, inc, focus: false, preview: false }
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
        let color = if self.focus { Color::LightRed } else { Color::Red };
        if self.preview {
            format!("{:.2}", self.val)
                .bold()
                .into_centered_line()
                .render(area, buf);
        } else {
            Gauge::default()
                .percent(self.percent())
                .label( format!("{:.2}", self.val).bold() )
                .gauge_style(color)
                .render(area, buf);
        }

    }
}
