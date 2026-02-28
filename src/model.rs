#[derive(Default,Debug)]
pub struct App {
    pub status : Status,
    pub configs : Configs,
    pub data : Data,
}

#[derive(Default,Debug)]
pub struct Status {
    pub should_quit : bool
}

#[derive(Default,Debug)]
pub struct Configs {}

#[derive(Default,Debug)]
pub struct Data {
    pub x : Vec<f32>,
    pub y : Vec<f32>,
    pub x_lim : (f32,f32),
    pub y_lim : (f32,f32),
}

