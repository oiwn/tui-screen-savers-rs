use crossterm::{event, Result};
use std::time::Duration;

pub fn process_input() -> Result<bool> {
    if event::poll(Duration::from_millis(10))? {
        if let event::Event::Key(keyevent) = event::read()? {
            if keyevent
                == event::KeyEvent::new(
                    event::KeyCode::Char('q'),
                    event::KeyModifiers::NONE,
                )
                || keyevent
                    == event::KeyEvent::new(
                        event::KeyCode::Esc,
                        event::KeyModifiers::NONE,
                    )
            {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
