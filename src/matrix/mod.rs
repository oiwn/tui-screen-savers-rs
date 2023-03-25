use charworm::{VerticalWorm, VerticalWormStyle};
use crossterm::{
    cursor, event,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use rand;
use std::{
    io::{Stdout, Write},
    time::Duration,
};

mod charworm;

static INITIAL_WORMS: u16 = 120;

pub struct Matrix {
    screen_width: u16,
    screen_height: u16,
    worms: Vec<VerticalWorm>,
    buffer_prev: ndarray::Array2<i8>,
    buffer_curr: ndarray::Array2<i8>,
    rng: rand::prelude::ThreadRng,
}

impl Matrix {
    // Initialize screensaver
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        let mut worms: Vec<VerticalWorm> = vec![];
        for _ in 1..=INITIAL_WORMS {
            worms.push(VerticalWorm::new(width, height, &mut rng));
        }
        let buffer_prev: ndarray::Array2<i8> =
            ndarray::Array::zeros((width as usize, height as usize));
        let mut buffer_curr = buffer_prev.clone();

        // fill current buffer
        for worm in worms.iter() {
            for pos in 0..worms.len() {
                let y = worm.fy.round() as i16 - pos as i16;
                if y > 0 {
                    let x = worm.fx.round() as u16;
                    if y < height as i16 {
                        buffer_curr[[x as usize, y as usize]] = 1;
                    }
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
        // delete changes
        let diff = self.buffer_prev.clone() - self.buffer_curr.clone();
        for (x, row) in diff.outer_iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                if *val == 1 as i8 {
                    stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
                    // stdout.queue(style::PrintStyledContent(' '.black()))?;
                    stdout.queue(style::Print(' '))?;
                }
            }
        }

        self.buffer_prev = self.buffer_curr.clone();
        self.buffer_curr.fill(0);

        let mut buffer_collision: ndarray::Array2<u8> = ndarray::Array::zeros((
            self.screen_width as usize,
            self.screen_height as usize,
        ));

        self.worms.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for worm in self.worms.iter() {
            let (x, y) = worm.to_points();
            if y < self.screen_height {
                for (pos, ch) in worm.body.iter().enumerate() {
                    if (y as i16 - pos as i16) >= 0 {
                        if buffer_collision[[x as usize, (y - pos as u16) as usize]]
                            != 1
                        {
                            stdout.queue(cursor::MoveTo(x, y - pos as u16))?;
                            stdout.queue(self.pick_style(
                                &worm.vw_style,
                                pos,
                                ch,
                            ))?;
                            self.buffer_curr
                                [[x as usize, (y - pos as u16) as usize]] = 1;
                            buffer_collision[[x as usize, y as usize]] = 1;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        // start updating/drawing from lower worms
        // self.worms.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
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
        std::thread::sleep(Duration::from_millis(20));

        matrix.draw(stdout)?;
        stdout.flush()?;
        matrix.update()?;
    }
    Ok(())
}
