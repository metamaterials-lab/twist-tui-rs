#[derive(Debug)]
pub struct Data {
    pub data : Vec<(f64,f64)>,
    pub x_lim : (f64,f64),
    pub y_lim : (f64,f64),
    pub x_unit : String,
    pub y_unit : String,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            data : Vec::new(),
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
    pub fn push( self : &mut Self, x : f32, y : f32 ) {
        self.data.push((x.into(),y.into()));
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
        let file_id = format!( "{}/results_{}", path, Local::now().format("%d-%m-%H-%M") );
        let file = File::create(file_id)?;
        let mut wr = BufWriter::new(file);
        writeln!(wr, "{},{}", self.x_unit,self.y_unit )?;
        for (x,y) in &self.data {
            writeln!(wr, "{},{}", x,y )?;
        }
        Ok(())
    }
}
