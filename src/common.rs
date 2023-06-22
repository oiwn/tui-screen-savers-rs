use crate::buffer::Cell;
use crossterm::{
    cursor, event,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::{io::Write, time::Duration};

pub trait TerminalEffect {
    fn get_diff(&mut self) -> Vec<(usize, usize, Cell)>;
    fn update(&mut self);
}

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

pub fn run_loop<W, TE>(
    stdout: &mut W,
    effect: &mut TE,
    iterations: Option<usize>,
) -> Result<f64>
where
    W: Write,
    TE: TerminalEffect,
{
    let (width, height) = terminal::size()?;

    // #[cfg(test)]
    let mut iters: usize = 0;

    let mut is_running = true;
    let mut frames_per_second = 0.0;
    let target_frame_duration = Duration::from_secs_f64(1.0 / 30.0_f64);

    // main loop
    while is_running {
        let started_at: std::time::SystemTime = std::time::SystemTime::now();
        is_running = process_input()?;

        // draw diff
        let queue = effect.get_diff();
        for item in queue.iter() {
            let (x, y, cell) = item;
            let actual_x = x + 1;
            let actual_y = y + 1;
            debug_assert!(
                actual_x <= width as usize
                    && actual_y <= height as usize
                    && actual_x >= 1
                    && actual_y >= 1
            );
            stdout.queue(cursor::MoveTo(actual_x as u16, actual_y as u16))?;
            stdout.queue(style::PrintStyledContent(
                cell.symbol.with(cell.color).attribute(cell.attr),
            ))?;
        }
        stdout.flush()?;
        effect.update();

        // stabilize fps if requred
        let ended_at = std::time::SystemTime::now();
        let delta = ended_at.duration_since(started_at).unwrap();
        if delta < target_frame_duration {
            std::thread::sleep(target_frame_duration - delta);
        };

        // calculate actual frame rate
        let ended_at = std::time::SystemTime::now();
        let delta = ended_at.duration_since(started_at).unwrap();
        frames_per_second = (frames_per_second + (1.0 / delta.as_secs_f64())) / 2.0;

        if delta < target_frame_duration {
            std::thread::sleep(target_frame_duration - delta);
        }

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
