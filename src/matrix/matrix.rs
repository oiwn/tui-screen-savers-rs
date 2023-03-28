use crate::matrix::charworm::VerticalWormStyle;
use crate::matrix::{Matrix, QueueItems};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::{
    io::{Stdout, Write},
    time::Duration,
};

static INITIAL_WORMS: usize = 80;
static MAX_WORMS: usize = 300;

pub fn draw(queue: &Vec<QueueItems>) -> Result<()> {
    Ok(())
}

pub fn run_loop(stdout: &mut Stdout) -> Result<()> {
    let mut is_running = true;
    let (width, height) = terminal::size()?;
    let mut matrix = Matrix::new(width, height, INITIAL_WORMS);

    // main loop
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    while is_running {
        is_running = Matrix::process_input()?;
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
        matrix.update()?;
    }
    Ok(())
}

pub fn pick_style(
    vw_style: &VerticalWormStyle,
    pos: usize,
    ch: &char,
) -> style::PrintStyledContent<char> {
    // let gradient = two_step_color_gradient(10);
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
    // style::PrintStyledContent(ch.green())
}
