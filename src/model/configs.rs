use crate::serial::SerialSetup;
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
            P::new_s("Dir", ["CW", "CCW"]),
            P::new_s("Mode", ["0-TOR", "0-ANG"]),
            P::new_i("Trav. Lim [°]", 30, 0, 2*360, 1),
            P::new_f("Speed [°/s]", 0.5, 0.05, 5.0, 0.01),
            P::new_i("Cycles", 1, 1, 10, 1),
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

impl Configs {
    pub fn serial_setup( self : &Self ) -> SerialSetup {
        SerialSetup::new(
            self.parameters[0].get_s().expect("fail to read direction"),
            self.parameters[1].get_s().expect("fail to read experiment mode"),
            self.parameters[2].get_i().expect("fail to read travel limit"),
            self.parameters[3].get_f().expect("fail to read speed"),
            self.parameters[4].get_i().expect("fail to read num cyles")
        )
    }
}
