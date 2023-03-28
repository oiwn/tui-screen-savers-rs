use crate::matrix::charworm::VerticalWormStyle;
use crate::matrix::{Matrix, QueueItems};
use crossterm::{
    cursor, event,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::{
    io::{Stdout, Write},
    time::Duration,
};

static INITIAL_WORMS: usize = 80;

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

pub fn run_loop<W>(stdout: &mut W) -> Result<f64>
where
    W: Write,
{
    let mut is_running = true;
    let mut frames_per_second = 0.0;
    let (width, height) = terminal::size()?;
    let mut matrix = Matrix::new(width, height, INITIAL_WORMS);

    #[cfg(test)]
    let mut iterations: u32 = 0;

    // main loop
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    while is_running {
        let started_at: std::time::SystemTime = std::time::SystemTime::now();
        is_running = process_input()?;
        std::thread::sleep(Duration::from_millis(10));

        let queue = matrix.draw();

        for draw_command in queue.iter() {
            match draw_command {
                QueueItems::MoveTo(x, y) => {
                    stdout.queue(cursor::MoveTo(x.clone(), y.clone()))?
                }
                QueueItems::PrintChar(s, p, c) => {
                    stdout.queue(pick_style(&s, p.clone() as usize, c))?
                }
                QueueItems::ClearChar => stdout.queue(style::Print(' '))?,
            };
        }
        stdout.flush()?;
        matrix.update();
        let ended_at = std::time::SystemTime::now();
        let delta = ended_at.duration_since(started_at).unwrap();
        frames_per_second = 1.0 / delta.as_secs_f64();

        #[cfg(test)]
        {
            iterations += 1;
            if iterations > 10 {
                is_running = false;
            }
        }
    }
    Ok(frames_per_second)
}

pub fn pick_style(
    vw_style: &VerticalWormStyle,
    pos: usize,
    ch: &char,
) -> style::PrintStyledContent<char> {
    let worm_style = match vw_style {
        VerticalWormStyle::Front => match pos {
            0 => style::PrintStyledContent(ch.white().bold()),
            1 => style::PrintStyledContent(ch.white()),
            2..=4 => style::PrintStyledContent(ch.green()),
            5..=7 => style::PrintStyledContent(ch.dark_green()),
            8..=12 => style::PrintStyledContent(ch.grey()),
            _ => style::PrintStyledContent(ch.dark_grey()),
        },
        VerticalWormStyle::Middle => match pos {
            0 => style::PrintStyledContent(ch.white()),
            1..=3 => style::PrintStyledContent(ch.green()),
            4..=5 => style::PrintStyledContent(ch.dark_green()),
            6..=10 => style::PrintStyledContent(ch.grey()),
            _ => style::PrintStyledContent(ch.dark_grey()),
        },
        VerticalWormStyle::Back => match pos {
            0 => style::PrintStyledContent(ch.green()),
            1..=3 => style::PrintStyledContent(ch.dark_green()),
            4..=5 => style::PrintStyledContent(ch.grey()),
            _ => style::PrintStyledContent(ch.dark_grey()),
        },
        VerticalWormStyle::Fading => match pos {
            0..=4 => style::PrintStyledContent(ch.grey()),
            _ => style::PrintStyledContent(ch.dark_grey()),
        },
        VerticalWormStyle::Gradient => match pos {
            0 => style::PrintStyledContent(ch.white().bold()),
            _ => {
                let color = style::Color::Rgb {
                    r: 0,
                    g: 255 - (pos as u16 * 12).clamp(0, 255) as u8,
                    b: 0,
                };
                style::PrintStyledContent(ch.with(color))
            }
        },
    };
    worm_style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_loop_10_iterations() {
        let mut stdout = Vec::new();
        let _ = run_loop(&mut stdout);
    }

    #[test]
    fn run_loop_fps_gte_20() {
        let mut stdout = Vec::new();
        let fps = run_loop(&mut stdout).unwrap();
        assert_eq!(fps > 20.0, true);
    }
}
