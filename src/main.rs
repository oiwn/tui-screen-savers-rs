#![cfg(not(test))]
use crossterm::{self, cursor, execute, terminal};
use std::{io, process};

mod matrix;

const HELP: &str = "\
Terminal screensavers";

#[derive(Debug)]
struct AppArgs {
    screen_saver: String,
}

fn main() -> crossterm::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing args: {}", e);
            process::exit(1);
        }
    };
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let fps = match args.screen_saver.as_str() {
        "matrix" => matrix::draw::run_loop(&mut stdout, None)?,
        _ => matrix::draw::run_loop(&mut stdout, None)?,
    };

    execute!(stdout, cursor::Show)?;
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
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
