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
    pub torque_units : String,
    pub angle : f32,
    pub angle_units : String,
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
    pub fn change_torque( self : &mut Self, t : f32, u : Option<&str> ) {
        self.torque = t;
        if let Some(u) = u { self.torque_units = u.to_string() }
    }
    pub fn change_angle( self : &mut Self, a : f32, u : Option<&str> ) {
        self.angle = a;
        if let Some(u) = u { self.angle_units = u.to_string() }
    }
}

impl Status {
    pub fn new( port : &str ) -> Self {
        Status {
            should_quit: false,
            serial_port: port.to_string(),
            button: Button::default(),
            torque: 0.0,
            torque_units: "LBIN".to_string(),
            angle: 0.0,
            angle_units: "DEG".to_string(),
            focus: false
        }
    }
}
