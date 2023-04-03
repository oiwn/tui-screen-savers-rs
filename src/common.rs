use crossterm::{event, Result};
use std::time::Duration;

pub fn process_input() -> Result<bool> {
    if event::poll(Duration::from_millis(10))? {
        match event::read()? {
            event::Event::Key(keyevent) => {
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
            _ => {}
        }
    }
    Ok(true)
}