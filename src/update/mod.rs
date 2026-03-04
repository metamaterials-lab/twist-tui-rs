pub mod app;
pub mod events;
pub mod configs;
pub mod quit;
pub mod status;

use crate::model::State;
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
}

#[derive(Debug)]
pub enum Action {
    None,
    Key(Keys),
    State(StateAction),
}

pub trait Update<T> {
    fn update( self : &mut Self, key : T ) -> std::io::Result<Action>;
}

