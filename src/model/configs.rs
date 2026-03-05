use crate::widgets::{Opt, Parameter, Selector};

#[derive(Debug)]
pub struct Configs {
    pub focus : bool,
    pub parameters : Vec<Parameter>,
}

impl Default for Configs {
    fn default() -> Self {
        let parameters = vec![
            Parameter::new("Type", 
                Selector::new([
                    Opt::new("Exp 1"),
                    Opt::new("Exp 2"),
                    Opt::new("Exp 3"),
                ])
            ),
            Parameter::new("Style", 
                Selector::new([
                    Opt::new("Stl 1"),
                    Opt::new("Stl 2"),
                    Opt::new("Stl 3"),
                ])
            )
        ];
        Configs { 
            focus: true,
            parameters
        }
    }
}
