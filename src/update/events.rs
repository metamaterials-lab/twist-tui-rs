use super::*;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
pub fn events_handler() -> std::io::Result<Action> {
    let res = match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => key_handler(key)?,
        _ => Action::None
    };
    Ok(res)
}

fn key_handler( key : KeyEvent ) -> std::io::Result<Action> {
    let res = match key.code {
        KeyCode::Up => Action::Key(Keys::Up),
        KeyCode::Down => Action::Key(Keys::Down),
        KeyCode::Left => Action::Key(Keys::Left),
        KeyCode::Right => Action::Key(Keys::Right),
        KeyCode::Enter => Action::Key(Keys::Enter),
        KeyCode::Tab => Action::Key(Keys::Tab),
        KeyCode::Char('q') => Action::Key(Keys::Q),
        _ => Action::Key(Keys::Other),
    };
    Ok(res)
}
