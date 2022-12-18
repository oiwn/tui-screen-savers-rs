use crossterm::{
    cursor, event,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use once_cell::sync::Lazy;
use rand::{
    self,
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};
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

static INITIAL_WORMS: u16 = 30;
static LENGTH_RANGE: (u8, u8) = (10, 20);
static SPEED_RANGE: (u8, u8) = (2, 8);

#[derive(Clone, Debug)]
enum VerticalWormStyle {
    Front,
    Middle,
    Back,
}

impl Distribution<VerticalWormStyle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VerticalWormStyle {
        match rng.gen_range(0..=2) {
            0 => VerticalWormStyle::Front,
            1 => VerticalWormStyle::Middle,
            _ => VerticalWormStyle::Back,
        }
    }
}

#[derive(Clone, Debug)]
struct VerticalWorm {
    body: Vec<char>,
    vw_style: VerticalWormStyle,
    fx: f32,
    fy: f32,
    offset: u16,
    max_length: u8,
    finish: bool,
    speed: u8,
}

impl VerticalWorm {
    fn new(w: u16, h: u16, rng: &mut rand::prelude::ThreadRng) -> Self {
        let body: Vec<char> = vec![*CHARACTERS.choose(rng).unwrap()];

        Self {
            body,
            vw_style: rand::random(),
            fx: rng.gen_range(0..w) as f32,
            fy: rng.gen_range(0..h / 2) as f32,
            offset: 0,
            max_length: rng.gen_range(LENGTH_RANGE.0..=LENGTH_RANGE.1),
            speed: rng.gen_range(SPEED_RANGE.0..SPEED_RANGE.1),
            finish: false,
        }
    }

    fn to_points(&self) -> (u16, u16) {
        let x = self.fx.round() as u16;
        let y = self.fy.round() as u16;
        (x, y)
    }

    fn reset(&mut self, w: u16, _h: u16, rng: &mut rand::prelude::ThreadRng) -> Result<()> {
        self.body.clear();
        self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone());
        // self.body.truncate(self.max_length as usize);
        self.fy = 0.0;
        self.fx = rng.gen_range(0..=w) as f32;
        self.speed = rng.gen_range(2..20);
        self.finish = false;
        self.max_length = rng.gen_range(4..10);
        Ok(())
    }

    fn grow(&mut self, head: u16, rng: &mut rand::prelude::ThreadRng) {
        let delta: i16 = head as i16 - self.fy.round() as i16;
        if delta > 0 {
            for _ in 0..=delta {
                self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone());
            }
            self.body.truncate(self.max_length as usize);
        }
    }

    fn update(
        &mut self,
        w: u16,
        h: u16,
        dt: Duration,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Result<()> {
        // there can be 3 cases:
        // worm vector not yet fully come from top
        // worm vector somewhere in the middle of the scren
        // worm vector reach bottom and need to fade out

        if (self.body.len() == 0) || (self.finish == true) {
            self.reset(w, h, rng)?;
        }

        let fy = self.fy + (self.speed as f32 * dt.as_millis() as f32) / 1000.0;
        let head = fy.round() as u16;
        let tail = fy.round() as i16 - self.body.len() as i16;

        // store previous tail to cleanup
        self.offset = head as u16 - self.fy.round() as u16;

        if tail <= 0 {
            // not fully come from top

            self.grow(head, rng);
            self.fy = fy;
            return Ok(());
        }

        if (head < h) && (tail > 0) {
            // somewhere in the middle

            self.grow(head, rng);
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
    buffer_prev: ndarray::Array2<u8>,
    buffer_curr: ndarray::Array2<u8>,
    rng: rand::prelude::ThreadRng,
}

impl Matrix {
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        let mut worms: Vec<VerticalWorm> = vec![];
        for _ in 1..=INITIAL_WORMS {
            worms.push(VerticalWorm::new(width as u16, height as u16, &mut rng));
        }
        let buffer_prev: ndarray::Array2<u8> =
            ndarray::Array::zeros((width as usize, height as usize));
        let mut buffer_curr = buffer_prev.clone();

        // fill current buffer
        for worm in worms.iter() {
            for pos in 0..worms.len() {
                let x = worm.fy.round() as i16 - pos as i16;
                if x >= 0 {
                    let y = worm.fx.round() as u16;
                    buffer_curr[[y as usize, x as usize]] = 1;
                }
            }
        }

        Self {
            screen_width: width,
            screen_height: height,
            worms,
            buffer_prev,
            buffer_curr,
            rng,
        }
    }

    fn pick_style(
        &self,
        vw_style: &VerticalWormStyle,
        pos: usize,
        ch: &char,
    ) -> style::PrintStyledContent<char> {
        let worm_style = match vw_style {
            VerticalWormStyle::Front => match pos {
                0 => style::PrintStyledContent(ch.white().bold()),
                1 => style::PrintStyledContent(ch.white()),
                2..=4 => style::PrintStyledContent(ch.green()),
                5..=7 => style::PrintStyledContent(ch.dark_green()),
                8..=12 => style::PrintStyledContent(ch.grey()),
                13..=20 => style::PrintStyledContent(ch.dark_grey()),
                _ => style::PrintStyledContent(ch.black()),
            },
            VerticalWormStyle::Middle => match pos {
                0 => style::PrintStyledContent(ch.white()),
                1..=3 => style::PrintStyledContent(ch.green()),
                4..=5 => style::PrintStyledContent(ch.dark_green()),
                6..=10 => style::PrintStyledContent(ch.grey()),
                11..=20 => style::PrintStyledContent(ch.dark_grey()),
                _ => style::PrintStyledContent(ch.black()),
            },
            VerticalWormStyle::Back => match pos {
                0 => style::PrintStyledContent(ch.green()),
                1..=3 => style::PrintStyledContent(ch.dark_green()),
                4..=5 => style::PrintStyledContent(ch.grey()),
                6..=20 => style::PrintStyledContent(ch.dark_grey()),
                _ => style::PrintStyledContent(ch.black()),
            },
        };
        worm_style
    }

    pub fn draw(&mut self, stdout: &mut Stdout) -> Result<()> {
        // delete current buffer and copy it to previous
        for (x, row) in self.buffer_curr.outer_iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                if self.buffer_curr[[x, y]] == 1 {
                    stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
                    stdout.queue(style::PrintStyledContent(' '.black()))?;
                }
            }
        }

        self.buffer_curr.fill(0);

        for worm in self.worms.iter() {
            let (x, y) = worm.to_points();
            if y <= self.screen_height {
                for (pos, ch) in worm.body.iter().enumerate() {
                    if (y as i16 - pos as i16) >= 0 {
                        stdout.queue(cursor::MoveTo(x, y - pos as u16))?;
                        stdout.queue(self.pick_style(&worm.vw_style, pos, ch))?;
                        self.buffer_curr[[x as usize, (y - pos as u16) as usize]] = 1;
                    }
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
                &mut self.rng,
            )?;
        }
        Ok(())
    }

    pub fn process_input() -> Result<bool> {
        if event::poll(Duration::from_millis(30))? {
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
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    while is_running {
        is_running = Matrix::process_input()?;

        matrix.draw(stdout)?;
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(20));
        matrix.update()?;
    }
    Ok(())
}
