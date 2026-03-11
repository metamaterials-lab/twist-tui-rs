pub mod app;
pub mod events;
pub mod configs;
pub mod quit;
pub mod status;

use crate::{model::{State, status::ComState}, serial::DeviceAction};
pub use self::events::events_handler;

#[derive(Debug)]
pub enum Keys {
    Up, Down, Left, Right,
    Enter, Tab,
    Q,
    Other,
}

#[derive(Debug)]
pub enum StateAction {
    Quit,
    ChangeState(State),
    ChangeComState(ComState),
    SerialData(f32,f32),
    SerialDataUnits(f32,String,f32,String),
    Msg(String)
}

#[derive(Debug)]
pub enum Action {
    None,
    Key(Keys),
    State(StateAction),
    Com(DeviceAction),
}

pub trait Update<T> {
    fn update( self : &mut Self, key : T ) -> std::io::Result<Action>;
    fn focus( self : &mut Self );
    fn unfocus( self : &mut Self );
}

