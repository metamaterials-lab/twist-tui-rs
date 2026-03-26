#[derive(Debug)]
pub struct Data {
    pub data : Vec<[Vec<(f64,f64)>;2]>,
    state : (usize,usize),
    pub x_lim : (f64,f64),
    pub y_lim : (f64,f64),
    pub x_unit : String,
    pub y_unit : String,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            data : vec![ [Vec::new(),Vec::new()] ],
            state: (0,0),
            x_lim: (0.0,1.0),
            y_lim: (0.0,1.0),
            x_unit: "X".to_string(),
            y_unit: "Y".to_string(),
        }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        if self.data.len() > 0 {
            let path = "./res/";
            if let Ok(_) = create_dir_all(path) {
                let _ = self.save("./res");
            }
        }
    }
}

use std::io::{Write,Result,Error,ErrorKind,BufWriter};
use std::fs::{create_dir_all, File};
use chrono::Local;
impl Data {
    pub fn push_state( self : &mut Self ) {
        if self.state.1 == 0 {
            self.state.1 = 1;
        } else {
            self.state.1 = 0;
            self.state.0 += 1;
            self.data.push( [ Vec::new(), Vec::new() ] )
        }
    }
    pub fn push( self : &mut Self, x : f32, y : f32 ) {
        self.data[self.state.0][self.state.1].push((x.into(),y.into()));
        self.x_lim.0 = self.x_lim.0.min(x.into());
        self.x_lim.1 = self.x_lim.1.max(x.into());
        self.y_lim.0 = self.y_lim.0.min(y.into());
        self.y_lim.1 = self.y_lim.1.max(y.into());
    }
    pub fn units( self : &mut Self, x : &str, y : &str ) {
        self.x_unit = x.to_string();
        self.y_unit = y.to_string();
    }
    pub fn save( self : &mut Self, path : &str ) -> Result<()> {
        create_dir_all(path)
            .map_err(|_| Error::new(ErrorKind::Other, "fail to create directory") )?;
        let file_id = format!( "{}/results_{}/", path, Local::now().format("%d-%m-%H-%M") );
        create_dir_all(file_id.clone())?;
        let mut c : u16 = 0;
        for [data0,data1] in &self.data {
            c += 1;
            let file_id = format!( "{}/cycle_{}.csv", &file_id, c );
            let file = File::create(file_id)?;
            let mut wr = BufWriter::new(file);
            writeln!(wr, "{},{}", self.x_unit,self.y_unit )?;
            for (x,y) in data0 {
                writeln!(wr, "{},{}", x,y )?;
            }
            writeln!(wr, "{},{}", self.x_unit,self.y_unit )?;
            for (x,y) in data1 {
                writeln!(wr, "{},{}", x,y )?;
            }
        }
        Ok(())
    }
}
