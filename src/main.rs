use std::char;
// Rust implementation of classic Matrix screensaver
use std::collections::HashMap;
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

use crossterm::{
    cursor, event, execute,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use once_cell::sync::Lazy;
use rand;
use rand::seq::SliceRandom;
use rand::Rng;

static CHARS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("digits", "012345789");
    m.insert("punctuation", r#":・."=*+-<>"#);
    m.insert("kanji", "日");
    m.insert("katakana", "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ");
    m.insert("other", "¦çﾘｸ");
    m
});

static CHARACTERS: Lazy<Vec<char>> = Lazy::new(|| {
    let mut v = Vec::new();
    for (_, chars) in CHARS.iter() {
        v.append(&mut chars.chars().collect());
    }
    v
});

static MAX_WORMS: u16 = 10;

#[derive(Clone)]
struct VerticalWorm {
    chars: Vec<char>,
    hx: u16,
    hy: u16,
    max_length: u8,
    finish: bool,
    speed: u8,
    rng: rand::prelude::ThreadRng,
}

impl VerticalWorm {
    fn new(w: u16, _h: u16) -> Self {
        let mut rng = rand::thread_rng();
        let max_length = rng.gen_range(4..10);
        let mut chars: Vec<char> = vec![];
        chars.push(CHARACTERS.choose(&mut rng).unwrap().clone());

        Self {
            chars,
            hx: rng.gen_range(0..w),
            hy: 0,
            max_length,
            speed: 9,
            finish: false,
            rng,
        }
    }
    fn update(&mut self, w: u16, h: u16) {
        // change character
        if self.chars.len() < self.max_length as usize {
            self.chars
                .insert(0, CHARACTERS.choose(&mut self.rng).unwrap().clone());
        }
        // shift one line down
        self.hy += 1;
        if self.hy >= h {
            self.hy = 0;
            self.hx = self.rng.gen_range(0..w);
            self.finish = true;
        }
    }
}

#[derive(Clone)]
struct Matrix {
    screen_width: u16,
    screen_height: u16,
    worms: Vec<VerticalWorm>,
}

impl Matrix {
    fn new(width: u16, heigth: u16) -> Self {
        let mut worms: Vec<VerticalWorm> = vec![];
        for _ in 1..MAX_WORMS {
            worms.push(VerticalWorm::new(width as u16, heigth as u16));
        }
        Self {
            screen_width: width,
            screen_height: heigth,
            worms,
        }
    }

    fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        for worm in self.worms.iter() {
            for (pos, wc) in worm.chars.iter().enumerate() {
                if worm.hy < pos as u16 {
                    continue;
                }
                let hy = worm.hy - pos as u16;
                if (hy > 1) && (hy < self.screen_height) {
                    stdout
                        .queue(cursor::MoveTo(worm.hx, hy))?
                        .queue(style::PrintStyledContent(wc.green()))?;
                }
            }
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        for worm in self.worms.iter_mut() {
            worm.update(self.screen_width, self.screen_height);
        }
        Ok(())
    }

    fn process_input() -> Result<bool> {
        if event::poll(Duration::from_millis(50))? {
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
}

fn main() -> Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut is_running = true;
    let (width, height) = terminal::size()?;
    let mut matrix = Matrix::new(width, height);
    // main loop
    while is_running {
        is_running = Matrix::process_input()?;
        matrix.draw(&mut stdout)?;
        stdout.flush()?;
        matrix.update()?;
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
