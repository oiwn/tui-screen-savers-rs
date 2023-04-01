use crate::rain::digital_rain::DigitalRain;
// use crate::rain::rain_drop::RainDropStyle;
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
    let mut matrix = DigitalRain::new(width, height, INITIAL_WORMS);

    // #[cfg(test)]
    let mut iters: usize = 0;

    // main loop
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    while is_running {
        let started_at: std::time::SystemTime = std::time::SystemTime::now();
        is_running = process_input()?;
        std::thread::sleep(Duration::from_millis(10));

        let queue = matrix.get_diff();
        for item in queue.iter() {
            let (x, y, cell) = item;
            stdout.queue(cursor::MoveTo(*x as u16, *y as u16))?;
            stdout
                .queue(style::PrintStyledContent(cell.symbol.with(cell.color)))?;
        }

        stdout.flush()?;
        matrix.update();
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

pub fn pick_color(pos: usize) -> style::Color {
    match pos {
        0 => style::Color::White,
        _ => {
            let color = style::Color::Rgb {
                r: 0,
                g: 255 - (pos as u16 * 12).clamp(0, 255) as u8,
                b: 0,
            };
            color
        }
    }
}

/* TODO: Style chars, need to add style into Cell along with color information
pub fn pick_style(
    vw_style: &RainDropStyle,
    pos: usize,
    ch: &char,
) -> style::PrintStyledContent<char> {
    let worm_style = match vw_style {
        RainDropStyle::Front => match pos {
            0 => style::PrintStyledContent(ch.white().bold()),
            1 => style::PrintStyledContent(ch.white()),
            2..=4 => style::PrintStyledContent(ch.green()),
            5..=7 => style::PrintStyledContent(ch.dark_green()),
            8..=12 => style::PrintStyledContent(ch.grey()),
            _ => style::PrintStyledContent(ch.dark_grey()),
        },
        RainDropStyle::Middle => match pos {
            0 => style::PrintStyledContent(ch.white()),
            1..=3 => style::PrintStyledContent(ch.green()),
            4..=5 => style::PrintStyledContent(ch.dark_green()),
            6..=10 => style::PrintStyledContent(ch.grey()),
            _ => style::PrintStyledContent(ch.dark_grey()),
        },
        RainDropStyle::Back => match pos {
            0 => style::PrintStyledContent(ch.green()),
            1..=3 => style::PrintStyledContent(ch.dark_green()),
            4..=5 => style::PrintStyledContent(ch.grey()),
            _ => style::PrintStyledContent(ch.dark_grey()),
        },
        RainDropStyle::Fading => match pos {
            0..=4 => style::PrintStyledContent(ch.grey()),
            _ => style::PrintStyledContent(ch.dark_grey()),
        },
        RainDropStyle::Gradient => match pos {
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
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_loop_10_iterations() {
        let mut stdout = Vec::new();
        let _ = run_loop(&mut stdout, Some(10));
    }

    #[test]
    fn run_loop_fps_gte_20() {
        let mut stdout = Vec::new();
        let fps = run_loop(&mut stdout, Some(10)).unwrap();
        assert_eq!(fps > 20.0, true);
    }
}
