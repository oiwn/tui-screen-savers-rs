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

static INITIAL_WORMS: u16 = 60;
// static MAX_WORMS: u16 = 40;

#[derive(Clone)]
struct VerticalWorm {
    chars: Vec<char>,
    fx: f32,
    fy: f32,
    max_length: u8,
    // finish: bool,
    speed: u8,
    rng: rand::prelude::ThreadRng,
}

impl VerticalWorm {
    fn new(w: u16, _h: u16) -> Self {
        let mut rng = rand::thread_rng();
        let mut chars: Vec<char> = vec![];
        chars.push(CHARACTERS.choose(&mut rng).unwrap().clone());

        Self {
            chars,
            fx: rng.gen_range(0..w) as f32,
            fy: 0.0,
            max_length: rng.gen_range(4..10),
            speed: rng.gen_range(2..20),
            // finish: false,
            rng,
        }
    }

    fn to_points(&self) -> (u16, u16) {
        let x = self.fx.round() as u16;
        let y = self.fy.round() as u16;
        (x, y)
    }

    fn reset(&mut self, w: u16, _h: u16) -> Result<()> {
        self.chars
            .insert(0, CHARACTERS.choose(&mut self.rng).unwrap().clone());
        self.chars.truncate(self.max_length as usize);
        self.fy = 0.0;
        self.fx = self.rng.gen_range(0..w) as f32;
        self.speed = self.rng.gen_range(2..20);
        self.max_length = self.rng.gen_range(4..10);
        Ok(())
    }

    fn update(&mut self, w: u16, h: u16, dt: Duration) -> Result<()> {
        self.chars
            .insert(0, CHARACTERS.choose(&mut self.rng).unwrap().clone());
        self.chars.truncate(self.max_length as usize);
        self.fy = self.fy + ((self.speed as f32 * dt.as_millis() as f32) / 1000.0);

        if self.fy.round() as u16 >= h {
            self.reset(w, h)?;
        }

        Ok(())
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
        for _ in 1..INITIAL_WORMS {
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
            let (x, y) = worm.to_points();
            for (pos, char) in worm.chars.iter().enumerate() {
                if (y as i16 - pos as i16) > 0 {
                    stdout.queue(cursor::MoveTo(x, y - pos as u16))?;

                    match pos {
                        0 => stdout.queue(style::PrintStyledContent(char.white().bold()))?,
                        1 => stdout.queue(style::PrintStyledContent(char.white()))?,
                        2 => stdout.queue(style::PrintStyledContent(char.green()))?,
                        3 => stdout.queue(style::PrintStyledContent(char.dark_green()))?,
                        10..=20 => stdout.queue(style::PrintStyledContent(char.black()))?,
                        _ => stdout.queue(style::PrintStyledContent(char.dark_grey()))?,
                    };
                }
            }
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        for worm in self.worms.iter_mut() {
            worm.update(
                self.screen_width,
                self.screen_height,
                Duration::from_millis(50),
            )?;
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
        // std::thread::sleep(Duration::from_millis(16));
        matrix.update()?;
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
