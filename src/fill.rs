use crossterm::{
    cursor, event,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::{io::Write, time::Duration};

static INITIAL_WORMS: usize = 100;

pub fn process_input() -> Result<bool> {
    if event::poll(Duration::from_millis(10))? {
        match event::read()? {
            event::Event::Key(keyevent) => {
                if keyevent
                    == event::KeyEvent::new(
                        event::KeyCode::Char('q'),
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

pub fn run_loop<W>(stdout: &mut W, iterations: Option<usize>) -> Result<f64>
where
    W: Write,
{
    let mut is_running = true;
    let mut frames_per_second = 0.0;
    let (width, height) = terminal::size()?;

    // #[cfg(test)]
    let mut iters: usize = 0;

    // main loop
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    while is_running {
        let started_at: std::time::SystemTime = std::time::SystemTime::now();
        is_running = process_input()?;
        std::thread::sleep(Duration::from_millis(5));

        stdout.flush()?;
        let ended_at = std::time::SystemTime::now();
        let delta = ended_at.duration_since(started_at).unwrap();
        frames_per_second = (frames_per_second + (1.0 / delta.as_secs_f64())) / 2.0;

        // #[cfg(test)]
        if let Some(iterations) = iterations {
            iters += 1;
            if iters > iterations {
                is_running = false;
            }
        };
    }
    Ok(frames_per_second)
}
