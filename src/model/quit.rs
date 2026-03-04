use crate::widgets::option::Opt;

#[derive(Debug)]
pub struct Quit {
    pub focus : bool,
    pub foo : Opt
}

impl Default for Quit {
    fn default() -> Self {
        Quit { foo: Opt::new("Hello"), focus : false }
    }
}
