use crate::widgets::{option::Opt, selector::Selector};

#[derive(Debug)]
pub struct Quit {
    pub focus : bool,
    pub selection : Selector
}

impl Default for Quit {
    fn default() -> Self {
        Quit {
            selection : Selector::new([Opt::new("No"), Opt::new("Yes")]),
            focus : false
        }
    }
}
