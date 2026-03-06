use crate::widgets::Selector;

#[derive(Debug)]
pub struct Quit {
    pub focus : bool,
    pub selection : Selector
}

impl Default for Quit {
    fn default() -> Self {
        Quit {
            selection : Selector::new(["No", "Yes"]),
            focus : false
        }
    }
}
