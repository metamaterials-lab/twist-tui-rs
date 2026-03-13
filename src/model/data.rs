#[derive(Debug)]
pub struct Data {
    pub data : Vec<(f64,f64)>,
    pub x_lim : (f64,f64),
    pub y_lim : (f64,f64),
}

impl Default for Data {
    fn default() -> Self {
        Data {
            data : Vec::new(),
            x_lim: (0.0,1.0),
            y_lim: (0.0,1.0)
        }
    }
}

impl Data {
    pub fn push( self : &mut Self, x : f32, y : f32 ) {
        self.data.push((x.into(),y.into()));
        self.x_lim.0 = self.x_lim.0.min(x.into());
        self.x_lim.1 = self.x_lim.1.max(x.into());
        self.y_lim.0 = self.y_lim.0.min(y.into());
        self.y_lim.1 = self.y_lim.1.max(y.into());
    }
}
