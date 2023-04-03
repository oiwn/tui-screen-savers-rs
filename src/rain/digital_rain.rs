use crate::buffer::{Buffer, Cell};
use crate::rain::draw::{pick_color, pick_style};
use crate::rain::gradient;
use crate::rain::rain_drop::RainDrop;

use rand::{self, Rng};
use std::time::Duration;

static MAX_WORMS: usize = 200;

pub struct DigitalRain {
    screen_width: u16,
    screen_height: u16,
    gradients: Vec<Vec<gradient::Color>>,
    rain_drops: Vec<RainDrop>,
    buffer: Buffer,
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

        // fill gradients
        let mut gradients = vec![];
        gradients.push(gradient::two_step_color_gradient(
            gradient::Color {
                r: 255,
                g: 255,
                b: 255,
            },
            gradient::Color { r: 0, g: 255, b: 0 },
            gradient::Color {
                r: 10,
                g: 10,
                b: 10,
            },
            4,
            height as usize / 2,
        ));
        gradients.push(gradient::two_step_color_gradient(
            gradient::Color {
                r: 200,
                g: 200,
                b: 200,
            },
            gradient::Color { r: 0, g: 250, b: 0 },
            gradient::Color {
                r: 10,
                g: 10,
                b: 10,
            },
            6,
            height as usize / 2,
        ));
        gradients.push(gradient::two_step_color_gradient(
            gradient::Color {
                r: 200,
                g: 200,
                b: 200,
            },
            gradient::Color { r: 0, g: 200, b: 0 },
            gradient::Color {
                r: 10,
                g: 10,
                b: 10,
            },
            4,
            height as usize / 2,
        ));

        Self {
            screen_width: width,
            screen_height: height,
            gradients,
            rain_drops,
            buffer,
            rng,
        }
    }

    /// Calculate difference between current frame and previous frame
    pub fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        let mut curr_buffer =
            Buffer::new(self.screen_width as usize, self.screen_height as usize);

        // fill current buffer
        // first draw drops with bigger fy
        self.rain_drops
            .sort_by(|a, b| a.fy.partial_cmp(&b.fy).unwrap());
        for rain_drop in self.rain_drops.iter().rev() {
            let points = rain_drop.to_points_vec();
            for (index, (x, y, character)) in points.iter().enumerate() {
                if *x < self.screen_width && *y < self.screen_height {
                    curr_buffer.set(
                        *x as usize,
                        *y as usize,
                        Cell::new(
                            *character,
                            pick_color(&rain_drop.style, index, &self.gradients),
                            pick_style(&rain_drop.style, index),
                        ),
                    );
                }
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

    /// Update each rain drop position
    pub fn update(&mut self) {
        for rain_drop in self.rain_drops.iter_mut() {
            rain_drop.update(
                self.screen_width,
                self.screen_height,
                Duration::from_millis(50),
                &mut self.rng,
            );
        }

        self.add_one();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let m = DigitalRain::new(30, 30, 20);
        assert_eq!(m.rain_drops.len(), 20);
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
