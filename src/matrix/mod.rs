use charworm::{VerticalWorm, VerticalWormStyle};

use rand;
use rand::Rng;
use std::time::Duration;

mod charworm;
pub mod matrix;

static MAX_WORMS: usize = 600;

pub enum QueueItems<'a> {
    MoveTo(u16, u16),
    PrintChar(&'a VerticalWormStyle, u16, char),
    ClearChar,
}

pub struct Matrix {
    screen_width: u16,
    screen_height: u16,
    worms: Vec<VerticalWorm>,
    map: ndarray::Array2<usize>,
    rng: rand::prelude::ThreadRng,
}

impl Matrix {
    // Initialize screensaver
    pub fn new(width: u16, height: u16, number_of_worms: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut worms: Vec<VerticalWorm> = vec![];
        for worm_id in 1..=number_of_worms {
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

    pub fn draw(&mut self) -> Vec<QueueItems> {
        let mut queue: Vec<QueueItems> = vec![];
        // queue all space without worm to delete
        for (x, row) in self.map.outer_iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                if *val == 0 {
                    queue.push(QueueItems::MoveTo(x as u16, y as u16));
                    queue.push(QueueItems::ClearChar);
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
                            queue.push(QueueItems::MoveTo(x as u16, yy as u16));
                            queue.push(QueueItems::PrintChar(
                                &worm.vw_style,
                                pos as u16,
                                ch.clone(),
                            ));
                        }
                    }
                }
            }
        }
        queue
    }

    /// Add one more worm with decent chance
    pub fn add_one(&mut self) {
        if self.worms.len() >= MAX_WORMS {
            return;
        };
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..=1.0) <= 0.3 {
            self.worms.push(VerticalWorm::new(
                self.screen_width,
                self.screen_height,
                self.worms.len() + 1,
                &mut rng,
            ));
        }
    }

    pub fn update(&mut self) {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {}

    #[test]
    fn draw() {}

    #[test]
    fn update() {}

    #[test]
    fn run_loop() {}
}
