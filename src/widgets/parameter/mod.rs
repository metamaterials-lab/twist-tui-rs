pub mod selector;
pub mod numeric;
pub mod generic;

use self::selector::Selector;
use self::numeric::Numeric;
use self::generic::Parameter;

use ratatui::prelude::*;
use crate::update::{Action, Keys, Update};


#[derive(Debug)]
pub enum P {
    S(Parameter<Selector>),
    F(Parameter<Numeric<f32>>),
    I(Parameter<Numeric<i16>>)
}

impl P {
    pub fn new_s<const N : usize>( name : &str, options : [&str;N] ) -> Self {
        P::S( Parameter::new(name,Selector::new(options) ) )
    }
    pub fn new_f( name : &str, val : f32, min : f32, max : f32, inc : f32 ) -> Self {
        P::F( Parameter::new(name, Numeric::new(val, min, max, inc)) )
    }
    pub fn new_i( name : &str, val : i16, min : i16, max : i16, inc : i16 ) -> Self {
        P::I( Parameter::new(name, Numeric::new(val, min, max, inc)) )
    }
    pub fn preview( self : &mut Self, f : bool ) {
        match self {
            P::S(s) => s.param.preview = f,
            P::F(s) => s.param.preview = f,
            P::I(s) => s.param.preview = f,
        }
    }
    pub fn get_s( self : &Self ) -> Option<&str> {
        match self {
            P::S(s) => Some(s.param.options[s.param.selection].get()),
            _ => None,
        }
    }
    pub fn get_i( self : &Self ) -> Option<i16> {
        match self {
            P::I(s) => Some(s.param.val),
            _ => None,
        }
    }
    pub fn get_f( self : &Self ) -> Option<f32> {
        match self {
            P::F(s) => Some(s.param.val),
            _ => None,
        }
    }
}

impl Update<Keys> for P {
    fn focus( self : &mut Self ) {
        match self {
            P::S(s) => s.focus(),
            P::F(s) => s.focus(),
            P::I(s) => s.focus(),
        }
    }
    fn unfocus( self : &mut Self ) {
        match self {
            P::S(s) => s.unfocus(),
            P::F(s) => s.unfocus(),
            P::I(s) => s.unfocus(),
        }
    }
    fn update( self : &mut Self, key : Keys ) -> std::io::Result<Action> {
        match self {
            P::S(s) => s.update(key),
            P::F(s) => s.update(key),
            P::I(s) => s.update(key),
        }
    }
}

impl Widget for &P {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized {
        match self {
            P::S(s) => s.render(area,buf),
            P::F(s) => s.render(area,buf),
            P::I(s) => s.render(area,buf),
        }
    }
}
