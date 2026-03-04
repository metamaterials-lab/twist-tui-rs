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

#[derive(Default,Debug)]
pub struct App {
    pub state : State,
    pub prev_state : State,
    pub status : Status,
    pub configs : Configs,
    pub data : Data,
    pub commands : Commands,
    pub quit : Quit
}

#[derive(Default,Debug,Copy,Clone)]
pub enum State {
    #[default]
    Config,
    Status,
    Back,
    Quit,
}
