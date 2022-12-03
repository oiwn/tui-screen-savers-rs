use crossterm::{
    cursor, event,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use once_cell::sync::Lazy;
use rand::{self, seq::SliceRandom, Rng};
// use rayon::prelude::*;
use std::{
    collections::HashMap,
    io::{Stdout, Write},
    time::Duration,
};

static CHARACTERS_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
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
    for (_, chars) in CHARACTERS_MAP.iter() {
        v.append(&mut chars.chars().collect());
    }
    v
});

static INITIAL_WORMS: u16 = 180;
// static MAX_WORMS: u16 = 40;
static MIN_LENGTH: u8 = 6;
static MAX_LENGTH: u8 = 20;

#[derive(Clone, Debug)]
struct VerticalWorm {
    body: Vec<char>,
    fx: f32,
    fy: f32,
    max_length: u8,
    finish: bool,
    speed: u8,
    rng: rand::prelude::ThreadRng,
}

impl VerticalWorm {
    fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::thread_rng();
        let body: Vec<char> = vec![*CHARACTERS.choose(&mut rng).unwrap()];

        Self {
            body,
            fx: rng.gen_range(0..w) as f32,
            fy: rng.gen_range(0..h / 2) as f32,
            max_length: rng.gen_range(MIN_LENGTH..=MAX_LENGTH),
            speed: rng.gen_range(2..20),
            finish: false,
            rng,
        }
    }

    fn to_points(&self) -> (u16, u16) {
        let x = self.fx.round() as u16;
        let y = self.fy.round() as u16;
        (x, y)
    }

    fn reset(&mut self, w: u16, _h: u16) -> Result<()> {
        self.body.clear();
        self.body
            .insert(0, CHARACTERS.choose(&mut self.rng).unwrap().clone());
        // self.body.truncate(self.max_length as usize);
        self.fy = 0.0;
        self.fx = self.rng.gen_range(0..=w) as f32;
        self.speed = self.rng.gen_range(2..20);
        self.finish = false;
        self.max_length = self.rng.gen_range(4..10);
        Ok(())
    }

    fn grow(&mut self, head: u16) {
        let delta: i16 = head as i16 - self.fy.round() as i16;
        if delta > 0 {
            for _ in 0..=delta {
                self.body
                    .insert(0, CHARACTERS.choose(&mut self.rng).unwrap().clone());
            }
            self.body.truncate(self.max_length as usize);
        }
    }

    fn update(&mut self, w: u16, h: u16, dt: Duration) -> Result<()> {
        // there can be 3 cases:
        // worm vector not yet fully come from top
        // worm vector somewhere in the middle of the scren
        // worm vector reach bottom and need to fade out

        if (self.body.len() == 0) || (self.finish == true) {
            self.reset(w, h)?;
        }

        let fy = self.fy + (self.speed as f32 * dt.as_millis() as f32) / 1000.0;
        let head = fy.round() as u16;
        let tail = fy.round() as i16 - self.body.len() as i16;

        if tail <= 0 {
            // not fully come from top

            self.grow(head);
            self.fy = fy;
            return Ok(());
        }

        if (head < h) && (tail > 0) {
            // somewhere in the middle

            self.grow(head);
            self.fy = fy;
            return Ok(());
        }

        if (head >= h) && (tail > 0) {
            // come to bottom
            self.finish = true;
            // self.reset(w, h)?;
            return Ok(());
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Matrix {
    screen_width: u16,
    screen_height: u16,
    worms: Vec<VerticalWorm>,
}

impl Matrix {
    pub fn new(width: u16, heigth: u16) -> Self {
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

    fn pick_style(&self, pos: usize, ch: &char) -> style::PrintStyledContent<char> {
        match pos {
            0 => style::PrintStyledContent(ch.white().bold()),
            1 => style::PrintStyledContent(ch.white()),
            2..=4 => style::PrintStyledContent(ch.green()),
            5..=7 => style::PrintStyledContent(ch.dark_green()),
            8..=10 => style::PrintStyledContent(ch.dark_grey()),
            _ => style::PrintStyledContent(ch.black()),
        }
    }

    pub fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        stdout.queue(terminal::Clear(terminal::ClearType::Purge))?;
        for worm in self.worms.iter() {
            let (x, y) = worm.to_points();
            if worm.finish == true {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::Print(' '))?;
                return Ok(());
            }
            for (pos, ch) in worm.body.iter().enumerate() {
                if (y as i16 - pos as i16) >= 0 {
                    stdout.queue(cursor::MoveTo(x, y - pos as u16))?;
                    stdout.queue(self.pick_style(pos, ch))?;

                    // match pos {
                    //     0 => stdout.queue(style::PrintStyledContent(char.white().bold()))?,
                    //     1 => stdout.queue(style::PrintStyledContent(char.white()))?,
                    //     2..=4 => stdout.queue(style::PrintStyledContent(char.green()))?,
                    //     5..=8 => stdout.queue(style::PrintStyledContent(char.dark_green()))?,
                    //     8..=10 => stdout.queue(style::PrintStyledContent(char.dark_grey()))?,
                    //     _ => stdout.queue(style::PrintStyledContent(char.black()))?,
                    // };
                }
            }
        }
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        // start updating/drawing from lower worms
        self.worms.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for worm in self.worms.iter_mut() {
            worm.update(
                self.screen_width,
                self.screen_height,
                Duration::from_millis(50),
            )?;
        }
        Ok(())
    }

    pub fn process_input() -> Result<bool> {
        if event::poll(Duration::from_millis(20))? {
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

pub fn run_loop(stdout: &mut Stdout) -> Result<()> {
    let mut is_running = true;
    let (width, height) = terminal::size()?;
    let mut matrix = Matrix::new(width, height);
    // main loop
    while is_running {
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        is_running = Matrix::process_input()?;
        matrix.draw(stdout)?;
        stdout.flush()?;
        // std::thread::sleep(Duration::from_millis(20));
        matrix.update()?;
    }
    Ok(())
}
