use crossterm::{cursor, execute, terminal, Result};
use std::io::stdout;

mod matrix;

fn main() -> Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    matrix::run_loop(&mut stdout)?;

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
