use crate::common::process_input;
use crate::rain::digital_rain::DigitalRain;
use crate::rain::gradient;
use crate::rain::rain_drop::RainDropStyle;
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::{io::Write, time::Duration};

static INITIAL_WORMS: usize = 100;

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
        std::thread::sleep(Duration::from_millis(5));

        let queue = matrix.get_diff();
        for item in queue.iter() {
            let (x, y, cell) = item;
            assert_eq!(
                *x < width as usize && *y < height as usize && *x >= 1 && *y >= 1,
                true
            );
            stdout.queue(cursor::MoveTo(*x as u16, *y as u16))?;
            stdout.queue(style::PrintStyledContent(
                cell.symbol.with(cell.color).attribute(cell.attr),
            ))?;
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

pub fn pick_style(vw_style: &RainDropStyle, pos: usize) -> style::Attribute {
    let drop_style = match vw_style {
        RainDropStyle::Front => style::Attribute::Bold,
        RainDropStyle::Middle => match pos {
            0..=4 => style::Attribute::Bold,
            _ => style::Attribute::NormalIntensity,
        },
        _ => style::Attribute::NormalIntensity,
    };
    // match pos {
    //     0 => style::Attribute::NormalIntensity,
    //     _ => style::Attribute::Bold,
    // }
    // style::Attribute::NormalIntensity
    drop_style
}

pub fn pick_color(
    vw_style: &RainDropStyle,
    pos: usize,
    gradients: &Vec<Vec<gradient::Color>>,
) -> style::Color {
    let drop_color = match vw_style {
        RainDropStyle::Gradient => match pos {
            0 => style::Color::White,
            _ => {
                let color = style::Color::Rgb {
                    r: 0,
                    g: 255 - (pos as u16 * 12).clamp(10, 256) as u8,
                    b: 0,
                };
                color
            }
        },
        RainDropStyle::Front => match pos {
            0 => style::Color::White,
            _ => {
                let color = style::Color::Rgb {
                    r: 0,
                    g: 255 - (pos.pow(2) as u16).clamp(10, 256) as u8,
                    b: 0,
                };
                color
            }
        },
        RainDropStyle::Back => {
            let color = gradients[2][pos];
            style::Color::Rgb {
                r: color.r,
                g: color.g,
                b: color.b,
            }
        }
        _ => style::Color::DarkGrey,
    };
    drop_color
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
