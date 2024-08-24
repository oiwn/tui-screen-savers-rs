use crate::common::TerminalEffect;
use crossterm::{
    cursor,
    event::{self, Event},
    execute,
    style::Stylize,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;

pub fn check<T: TerminalEffect>(effect: &mut T, frames: usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    for frame in 1..=frames {
        // Clear the screen
        execute!(stdout, Clear(ClearType::All))?;

        // Get the diff for the current frame
        let diff = effect.get_diff();

        // Render the frame
        for (x, y, cell) in diff {
            execute!(
                stdout,
                cursor::MoveTo(x as u16, y as u16),
                crossterm::style::PrintStyledContent(
                    cell.symbol.with(cell.color).attribute(cell.attr)
                )
            )?;
        }

        // Print frame number
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            crossterm::style::Print(format!("Frame: {}", frame))
        )?;

        stdout.flush()?;

        // Update the effect for the next frame
        effect.update();
    }

    // Wait for any key press
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn run_check(effect_name: &str, frames: usize) -> io::Result<()> {
    match effect_name {
        "matrix" => {
            let options =
                crate::rain::digital_rain::DigitalRainOptionsBuilder::default()
                    .screen_size(terminal::size()?)
                    .drops_range((120, 240))
                    .speed_range((2, 16))
                    .build()
                    .unwrap();
            let mut digital_rain =
                crate::rain::digital_rain::DigitalRain::new(options);
            check(&mut digital_rain, frames)
        }
        "life" => {
            let options = crate::life::ConwayLifeOptionsBuilder::default()
                .screen_size(terminal::size()?)
                .build()
                .unwrap();
            let mut conway_life = crate::life::ConwayLife::new(options);
            check(&mut conway_life, frames)
        }
        "maze" => {
            let options = crate::maze::MazeOptionsBuilder::default()
                .screen_size(terminal::size()?)
                .build()
                .unwrap();
            let mut maze = crate::maze::Maze::new(options);
            check(&mut maze, frames)
        }
        _ => {
            println!(
                "Unknown effect: {}. Available effects are: matrix, life, maze",
                effect_name
            );
            Ok(())
        }
    }
}
