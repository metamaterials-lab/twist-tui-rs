use crate::widgets::button::Button;

#[derive(Debug)]
pub struct Status {
    pub should_quit : bool,
    pub serial_port : String,
    pub button : Button,
    pub torque : f32,
    pub angle : f32,
    pub focus : bool
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
