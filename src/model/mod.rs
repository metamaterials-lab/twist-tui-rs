pub mod status;
pub mod configs;
pub mod data;
pub mod commands;
pub mod quit;


pub use self::status::Status;
pub use self::configs::Configs;
pub use self::data::Data;
pub use self::commands::Commands;
pub use self::quit::Quit;
use crate::serial::{Com, Twister};


#[derive(Debug)]
pub struct App {
    pub lock : bool,
    pub state : State,
    pub prev_state : State,
    pub status : Status,
    pub configs : Configs,
    pub data : Data,
    pub commands : Commands,
    pub quit : Quit,
    pub com : Com
}

#[derive(Default,Debug,Copy,Clone)]
pub enum State {
    #[default]
    Config,
    Status,
    Back,
    Quit,
}

impl App {
    pub fn new( port : &str, baudrate : u32 ) -> std::io::Result<Self> {
        Ok( App { 
            lock: false,
            state: State::default(),
            prev_state: State::default(),
            status: Status::default(),
            configs: Configs::default(),
            data: Data::default(),
            commands: Commands::default(),
            quit: Quit::default(),
            com: Com::new::<Twister>(port, baudrate )?
        } )
    }
}
