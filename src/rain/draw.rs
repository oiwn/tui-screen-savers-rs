use crate::common::process_input;
use crate::rain::digital_rain::{DigitalRain, DigitalRainOptionsBuilder};
use crate::rain::gradient;
use crate::rain::rain_drop::RainDropStyle;
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::{io::Write, time::Duration};

pub fn run_loop<W>(stdout: &mut W, iterations: Option<usize>) -> Result<f64>
where
    W: Write,
{
    let (width, height) = terminal::size()?;

    // #[cfg(test)]
    let mut iters: usize = 0;

    let mut is_running = true;
    let mut frames_per_second = 0.0;
    let target_frame_duration = Duration::from_secs_f64(1.0 / 40.0_f64);

    let rain_options = DigitalRainOptionsBuilder::default()
        .size((width, height))
        .drops_range((100, 200))
        .speed_range((2, 15))
        .build()
        .unwrap();
    let mut digital_rain = DigitalRain::new(rain_options);

    // main loop
    while is_running {
        let started_at: std::time::SystemTime = std::time::SystemTime::now();
        is_running = process_input()?;

        // draw diff
        let queue = digital_rain.get_diff();
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
        digital_rain.update();

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

pub fn pick_style(vw_style: &RainDropStyle, pos: usize) -> style::Attribute {
    match vw_style {
        RainDropStyle::Front => style::Attribute::Bold,
        RainDropStyle::Middle => match pos {
            0..=4 => style::Attribute::Bold,
            _ => style::Attribute::NormalIntensity,
        },
        RainDropStyle::Back => style::Attribute::Bold,
        _ => style::Attribute::NormalIntensity,
    }
}

pub fn pick_color(
    vw_style: &RainDropStyle,
    pos: usize,
    gradients: &[Vec<gradient::Color>],
) -> style::Color {
    match vw_style {
        RainDropStyle::Gradient => match pos {
            0 => style::Color::White,
            _ => style::Color::Rgb {
                r: 0,
                g: 255 - (pos as u16 * 12).clamp(10, 256) as u8,
                b: 0,
            },
        },
        RainDropStyle::Front => match pos {
            0 => style::Color::White,
            _ => style::Color::Rgb {
                r: 0,
                g: 255 - (pos.pow(2) as u16).clamp(10, 256) as u8,
                b: 0,
            },
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_loop_10_iterations() {
        let mut stdout = Vec::new();
        let _ = run_loop(&mut stdout, Some(10));
    }

    // #[test]
    fn run_loop_fps_gte_20() {
        // NOTE: this test failed on github CI pipeline
        let mut stdout = Vec::new();
        let fps = run_loop(&mut stdout, Some(10)).unwrap();
        assert_eq!(fps > 20.0, true);
    }
}
