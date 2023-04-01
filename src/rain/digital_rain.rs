use crate::buffer::{Buffer, Cell};
use crate::rain::draw::pick_color;
use crate::rain::rain_drop::{RainDrop, RainDropStyle};
// use crossterm::style;

use rand::{self, Rng};
use std::time::Duration;

static MAX_WORMS: usize = 200;

pub struct DigitalRain {
    screen_width: u16,
    screen_height: u16,
    pub rain_drops: Vec<RainDrop>,
    pub buffer: Buffer,
    map: ndarray::Array2<usize>,
    rng: rand::prelude::ThreadRng,
}

impl DigitalRain {
    // Initialize screensaver
    pub fn new(width: u16, height: u16, number_of_worms: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut rain_drops: Vec<RainDrop> = vec![];
        let buffer: Buffer = Buffer::new(width as usize, height as usize);
        for rain_drop_id in 1..=number_of_worms {
            rain_drops.push(RainDrop::new(width, height, rain_drop_id, &mut rng));
        }

        let prev_map: ndarray::Array2<usize> =
            ndarray::Array::zeros((width as usize, height as usize));
        let mut map: ndarray::Array2<usize> =
            ndarray::Array::zeros((width as usize, height as usize));

        // fill current buffer
        // worm with lower y coordinate have priority
        rain_drops.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for rain_drop in rain_drops.iter() {
            let (x, y) = rain_drop.to_point();
            for pos in 0..rain_drop.body.len() {
                let yy = y as i16 - pos as i16;
                if yy >= 0 {
                    map[[x as usize, yy as usize]] = rain_drop.worm_id;
                }
            }
        }

        Self {
            screen_width: width,
            screen_height: height,
            rain_drops,
            buffer,
            map,
            rng,
        }
    }

    pub fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        let mut curr_buffer =
            Buffer::new(self.screen_width as usize, self.screen_height as usize);

        // fill current buffer
        self.rain_drops
            .sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for rain_drop in self.rain_drops.iter() {
            let points = rain_drop.to_points_vec();
            for (index, (x, y, character)) in points.iter().enumerate() {
                curr_buffer.set(
                    *x as usize,
                    *y as usize,
                    Cell::new(character.clone(), pick_color(index)),
                );
            }
        }

        let diff = self.buffer.diff(&curr_buffer);
        self.buffer = curr_buffer;
        diff
    }

    /// Add one more worm with decent chance
    pub fn add_one(&mut self) {
        if self.rain_drops.len() >= MAX_WORMS {
            return;
        };
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..=1.0) <= 0.3 {
            self.rain_drops.push(RainDrop::new(
                self.screen_width,
                self.screen_height,
                self.rain_drops.len() + 1,
                &mut rng,
            ));
        }
    }

    pub fn update(&mut self) {
        // start updating/drawing from lower worms
        // self.worms.sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        // self.worms.reverse();
        for rain_drop in self.rain_drops.iter_mut() {
            rain_drop.update(
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
        self.rain_drops
            .sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for rain_drop in self.rain_drops.iter() {
            let (x, y) = rain_drop.to_point();
            for pos in 0..rain_drop.body.len() {
                let yy = y as i16 - pos as i16;
                if yy >= 0 {
                    self.map[[x as usize, yy as usize]] = rain_drop.worm_id;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let m = DigitalRain::new(30, 30, 20);
        assert_eq!(m.rain_drops.len(), 20);
        assert_eq!(m.map.shape(), &[30, 30]);
    }

    #[test]
    fn draw() {
        let mut m = DigitalRain::new(30, 30, 20);
        let q = m.get_diff();
        assert_eq!(q.len() > 0, true);
        // assert_eq!(q.len(), 1000);
        assert_eq!(q.len() < 1000, true);
    }

    #[test]
    fn update() {
        let mut m = DigitalRain::new(30, 30, 20);
        let q_1_len = m.get_diff().len();
        m.update();
        let q_2_len = m.get_diff().len();
        assert_eq!(q_2_len > q_1_len, true);
    }
}
