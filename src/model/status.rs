use crate::widgets::button::Button;

#[derive(Default,Debug)]
pub enum ComState {
    #[default]
    Run,
    Setup,
    Stop
}

#[derive(Debug)]
pub struct Status {
    pub should_quit : bool,
    pub serial_port : String,
    pub button : Button<ComState>,
    pub torque : f32,
    pub angle : f32,
    pub focus : bool
}

impl Status {
    pub fn change_run( self : &mut Self ) {
        self.button.state = ComState::Run;
    }
    pub fn change_setup( self : &mut Self ) {
        self.button.state = ComState::Setup;
    }
    pub fn change_stop( self : &mut Self ) {
        self.button.state = ComState::Stop;
    }
}

impl Default for Status {
    fn default() -> Self {
        Status {
            should_quit: false,
            serial_port: "/dev/ttyUSB0".to_string(),
            button: Button::default(),
            torque: 0.0,
            angle: 0.0,
            focus: false
        }
    }
}
