#[derive(Default,Debug)]
pub struct App {
    pub state : State,
    pub status : Status,
    pub configs : Configs,
    pub data : Data,
    pub commands : Commands,
    pub quit : Quit
}

#[derive(Default,Debug)]
pub enum State {
    #[default]
    Config,
    Status,
    Quit,
}

#[derive(Default,Debug)]
pub struct Status {
    pub should_quit : bool,
    pub focus : bool
}

#[derive(Debug)]
pub struct Configs {
    pub focus : bool
}

#[derive(Default,Debug)]
pub struct Data {
    pub x : Vec<f32>,
    pub y : Vec<f32>,
    pub x_lim : (f32,f32),
    pub y_lim : (f32,f32),
}

#[derive(Default,Debug)]
pub struct Commands {}

#[derive(Default,Debug)]
pub struct Quit {}


impl Default for Configs {
    fn default() -> Self {
        Configs { focus: true }
    }
}
