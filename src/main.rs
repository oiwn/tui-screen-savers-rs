//! # tarts
//!
//! `tarts` or TerminalArts is a collection of terminal-based screen savers written
//! in Rust. This crate provides a variety of screen savers like "Matrix Rain",
//! "Conway's Game of Life", and "Maze Generation" (not yet), all running directly
//! in your terminal.
//!
//! ## Features
//!
//! - Matrix Rain: Simulates the famous "Matrix" digital rain effect in your terminal.
//! - Conway's Game of Life: Implements the classic cellular automaton in the terminal.
//! - [not yet] Maze Generation: Generates and displays a random maze.
//!
//! ## Usage
//!
//! To use the screen savers, run the executable with the desired screen saver's
//! name as an argument:
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
//! The screen savers can be configured via command line arguments
//! (planning to add configuration file).
//!
//! ## Contributing
//!
//! Contributions are welcome! Please feel free to submit pull requests,
//! report bugs, and suggest features.
//!
//! ## License
//!
//! This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
//!
#![cfg(not(test))]
use crossterm::{self, cursor, execute, terminal};
use tarts::{config, rain};
// use log::info;
use std::{io, process};

mod blank;
mod buffer;
mod check;
mod common;
mod config;
mod error;
mod life;
mod maze;
mod rain;

use crate::common::DefaultOptions;

const HELP: &str = "Terminal screensavers, run with arg: matrix, life, maze";

#[derive(Debug)]
struct AppArgs {
    screen_saver: String,
    check: bool,
    effect: Option<String>,
    frames: Option<usize>,
}

fn main() -> Result<(), error::TartsError> {
    env_logger::init();

    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing args: {}", e);
            process::exit(1);
        }
    };

    if args.check {
        let effect = args.effect.unwrap_or_else(|| "matrix".to_string());
        let frames = args.frames.unwrap_or(1);
        return check::run_test_for_effect(&effect, frames);
    }

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
            // info!("Initializing DigitalRain effect...");
            let options =
                rain::digital_rain::DigitalRain::default_options(width, height);
            let mut digital_rain = rain::digital_rain::DigitalRain::new(options);
            // info!("Running DigitalRain effect main loop...");
            common::run_loop(&mut stdout, &mut digital_rain, None)?
        }
        "life" => {
            // info!("Initializing Life effect...");
            let options = life::ConwayLife::default_options(width, height);
            let mut conway_life = life::ConwayLife::new(options);
            // info!("Running Life effect main loop...");
            common::run_loop(&mut stdout, &mut conway_life, None)?
        }
        "maze" => {
            // info!("Initializing Maze effect...");
            let options = maze::Maze::default_options(width, height);
            let mut maze = maze::Maze::new(options);
            // info!("Running Maze effect main loop...");
            common::run_loop(&mut stdout, &mut maze, None)?
        }
        "blank" => {
            // info!("Initializing Blank effect...");
            let options = blank::BlankOptionsBuilder::default()
                .screen_size((width, height))
                .build()
                .unwrap();
            let mut check = blank::Blank::new(options);
            // info!("Running Blank effect main loop...");
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

    let check = pargs.contains("--check");
    let effect = pargs.opt_value_from_str("--effect")?;
    let frames = pargs.opt_value_from_str("--frames")?;

    let args = AppArgs {
        screen_saver: pargs.free_from_str().map_or("matrix".into(), |arg| arg),
        check,
        effect,
        frames,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}", remaining);
    }

    Ok(args)
}
