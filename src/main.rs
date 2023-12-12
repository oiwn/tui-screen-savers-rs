//! # tarts
//!
//! `tarts` or TerminalArts is a collection of terminal-based screen savers written in Rust.
//! This crate provides a variety of screen savers like "Matrix Rain", "Conway's Game of Life",
//! and "Maze Generation" (not yet), all running directly in your terminal.
//!
//! ## Features
//!
//! - Matrix Rain: Simulates the famous "Matrix" digital rain effect in your terminal.
//! - Conway's Game of Life: Implements the classic cellular automaton in the terminal.
//! - [not yet] Maze Generation: Generates and displays a random maze.
//!
//! ## Usage
//!
//! To use the screen savers, run the executable with the desired screen saver's name as an argument:
//!
//! ```bash
//! tarts matrix
//! tarts life
//! tarts maze
//! ```
//!
//! ## Installation
//!
//! Install directly using cargo:
//!
//! ```bash
//! cargo install tarts
//! ```
//!
//! ## Configuration
//!
//! The screen savers can be configured via command line arguments (planning to add configuration file).
//!
//! ## Contributing
//!
//! Contributions are welcome! Please feel free to submit pull requests, report bugs, and suggest features.
//!
//! ## License
//!
//! This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
//!
#![cfg(not(test))]
use crossterm::{self, cursor, execute, terminal};
use std::{io, process};

mod buffer;
mod check;
mod common;
mod life;
mod maze;
mod rain;

const HELP: &str = "Terminal screensavers, run with arg: matrix, life, maze";

#[derive(Debug)]
struct AppArgs {
    screen_saver: String,
}

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing args: {}", e);
            process::exit(1);
        }
    };
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;

    let (width, height) = terminal::size()?;

    let fps = match args.screen_saver.as_str() {
        "matrix" => {
            let options = rain::digital_rain::DigitalRainOptionsBuilder::default()
                .size((width, height))
                .drops_range((120, 240))
                .speed_range((2, 16))
                .build()
                .unwrap();
            let mut digital_rain = rain::digital_rain::DigitalRain::new(options);
            common::run_loop(&mut stdout, &mut digital_rain, None)?
        }
        "life" => {
            let options = life::ConwayLifeOptionsBuilder::default()
                .screen_size((width as usize, height as usize))
                .build()
                .unwrap();
            let mut conway_life = life::ConwayLife::new(options);
            common::run_loop(&mut stdout, &mut conway_life, None)?
        }
        "maze" => {
            let options = maze::MazeOptionsBuilder::default()
                .screen_size((width as usize, height as usize))
                .build()
                .unwrap();
            let mut wilson_maze = maze::Maze::new(options);
            common::run_loop(&mut stdout, &mut wilson_maze, None)?
        }
        "check" => {
            let options = check::CheckOptionsBuilder::default()
                .screen_size((width as usize, height as usize))
                .build()
                .unwrap();
            let mut check = check::Check::new(options);
            common::run_loop(&mut stdout, &mut check, None)?
        }
        _ => {
            println!("Pick screensaver: [matrix, life, maze]");
            0.0
        }
    };

    execute!(
        stdout,
        cursor::Show,
        terminal::Clear(terminal::ClearType::All),
        terminal::LeaveAlternateScreen,
    )?;
    terminal::disable_raw_mode()?;

    println!("Frames per second: {}", fps);
    Ok(())
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        process::exit(0);
    }

    let args = AppArgs {
        // default screensaver is "matrix"
        screen_saver: pargs.free_from_str().map_or("matrix".into(), |arg| arg),
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}", remaining);
    }

    Ok(args)
}
