use charworm::{VerticalWorm, VerticalWormStyle};
use crossterm::{
    cursor, event,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use rand;
use rand::Rng;
use std::{
    io::{Stdout, Write},
    time::Duration,
};

mod charworm;

static INITIAL_WORMS: usize = 100;
static MAX_WORMS: usize = 300;

pub struct Matrix {
    screen_width: u16,
    screen_height: u16,
    worms: Vec<VerticalWorm>,
    map: ndarray::Array2<usize>,
    rng: rand::prelude::ThreadRng,
}

impl Matrix {
    // Initialize screensaver
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        let mut worms: Vec<VerticalWorm> = vec![];
        for worm_id in 1..=INITIAL_WORMS {
            worms.push(VerticalWorm::new(width, height, worm_id, &mut rng));
        }

        let mut map: ndarray::Array2<usize> =
            ndarray::Array::zeros((width as usize, height as usize));

        // fill current buffer
        // worm with lower y coordinate have priority
        worms.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for worm in worms.iter() {
            let (x, y) = worm.to_points();
            for pos in 0..worms.len() {
                let yy = y as i16 - pos as i16;
                if yy >= 0 {
                    map[[x as usize, yy as usize]] = worm.worm_id;
                }
            }
        }

        Self {
            screen_width: width,
            screen_height: height,
            worms,
            map,
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
        };
        worm_style
        // style::PrintStyledContent(ch.green())
    }

    pub fn draw(&mut self, stdout: &mut Stdout) -> Result<()> {
        // queue all space without worm to delete
        for (x, row) in self.map.outer_iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                if *val == 0 {
                    stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
                    stdout.queue(style::Print(' '))?;
                }
            }
        }

        self.worms.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for worm in self.worms.iter() {
            let (x, y) = worm.to_points();
            if y < self.screen_height {
                for (pos, ch) in worm.body.iter().enumerate() {
                    let yy = y as i16 - pos as i16;
                    if yy >= 0 {
                        if self.map[[x as usize, (y - pos as u16) as usize]]
                            == worm.worm_id
                        {
                            stdout.queue(cursor::MoveTo(x, yy as u16))?;
                            stdout.queue(self.pick_style(
                                &worm.vw_style,
                                pos,
                                // &worm.worm_id.to_string().chars().next().unwrap(),
                                ch,
                            ))?;
                            // self.map[[x as usize, yy as usize]] = worm.worm_id;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Add one more worm with decent chance
    pub fn add_one(&mut self) {
        if self.worms.len() >= MAX_WORMS {
            return;
        };
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..=1.0) <= 0.1 {
            self.worms.push(VerticalWorm::new(
                self.screen_width,
                self.screen_height,
                self.worms.len() + 1,
                &mut rng,
            ));
        }
    }

    pub fn update(&mut self) -> Result<()> {
        // start updating/drawing from lower worms
        // self.worms.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        // self.worms.reverse();
        for worm in self.worms.iter_mut() {
            worm.update(
                self.screen_width,
                self.screen_height,
                Duration::from_millis(50),
                &mut self.rng,
            );
        }

        self.add_one();

        // fill current buffer
        // worm with lower y coordinate have priority
        self.map.fill(0);
        self.worms.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for worm in self.worms.iter() {
            let (x, y) = worm.to_points();
            for pos in 0..worm.body.len() {
                let yy = y as i16 - pos as i16;
                if yy >= 0 {
                    self.map[[x as usize, yy as usize]] = worm.worm_id;
                }
            }
        }

        Ok(())
    }

    pub fn process_input() -> Result<bool> {
        if event::poll(Duration::from_millis(10))? {
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
        std::thread::sleep(Duration::from_millis(10));

        matrix.draw(stdout)?;
        stdout.flush()?;
        matrix.update()?;
    }
    Ok(())
}
