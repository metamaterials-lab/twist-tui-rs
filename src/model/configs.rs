use crate::update::Update;
use crate::widgets::parameter::P;


#[derive(Debug)]
pub struct Configs {
    pub focus : bool,
    pub select : usize,
    pub parameters : Vec<P>,
}

impl Default for Configs {
    fn default() -> Self {
        let parameters = vec![
            P::new_s("Type", [
                "A", "B", "C"
            ]),
            P::new_i("Num", 10, 0, 15, 1)
        ];
        let mut res = Configs { 
            focus: false,
            select: 0,
            parameters
        };
        res.focus();
        res
    }
}
