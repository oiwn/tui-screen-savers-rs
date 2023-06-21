use super::draw::{pick_color, pick_style};
use super::gradient;
use super::rain_drop::RainDrop;
use crate::buffer::{Buffer, Cell};
use crate::common::TerminalEffect;

use derive_builder::Builder;
use rand::{self, Rng};
use std::time::Duration;

#[derive(Builder, Default, Debug, PartialEq)]
pub struct DigitalRainOptions {
    pub size: (u16, u16),
    pub drops_range: (u16, u16),
    pub speed_range: (u16, u16),
}

pub struct DigitalRain {
    options: DigitalRainOptions,
    gradients: Vec<Vec<gradient::Color>>,
    rain_drops: Vec<RainDrop>,
    buffer: Buffer,
    rng: rand::prelude::ThreadRng,
}

impl TerminalEffect for DigitalRain {
    /// Calculate difference between current frame and previous frame
    fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        let mut curr_buffer = Buffer::new(
            self.options.get_width() as usize,
            self.options.get_height() as usize,
        );

        // fill current buffer
        // first draw drops with bigger fy
        Self::fill_buffer(&mut self.rain_drops, &mut curr_buffer, &self.gradients);

        let diff = self.buffer.diff(&curr_buffer);
        self.buffer = curr_buffer;
        diff
    }

    /// Update each rain drop position
    fn update(&mut self) {
        for rain_drop in self.rain_drops.iter_mut() {
            rain_drop.update(
                &self.options,
                Duration::from_millis(50),
                &mut self.rng,
            );
        }

        self.add_one();
    }
}

/// Process digital rain effect.
/// Noice that all processing done implying coordinates started from 0, 0
/// and width / height is actual number of columnts and rows
impl DigitalRain {
    // Initialize screensaver
    pub fn new(options: DigitalRainOptions) -> Self {
        let mut rng = rand::thread_rng();
        let mut rain_drops: Vec<RainDrop> = vec![];
        let mut buffer: Buffer = Buffer::new(
            options.get_width() as usize,
            options.get_height() as usize,
        );
        for rain_drop_id in 1..=options.get_min_drops_number() {
            rain_drops.push(RainDrop::new(
                &options,
                rain_drop_id as usize,
                &mut rng,
            ));
        }

        // fill gradients
        let gradients = vec![
            gradient::two_step_color_gradient(
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
                3 * options.get_height() as usize / 2,
            ),
            gradient::two_step_color_gradient(
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
                3 * options.get_height() as usize / 2,
            ),
            gradient::two_step_color_gradient(
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
                options.get_height() as usize / 2,
                3 * options.get_height() as usize / 2,
            ),
        ];

        Self::fill_buffer(&mut rain_drops, &mut buffer, &gradients);

        Self {
            options,
            gradients,
            rain_drops,
            buffer,
            rng,
        }
    }

    pub fn fill_buffer(
        rain_drops: &mut [RainDrop],
        buffer: &mut Buffer,
        gradients: &[Vec<gradient::Color>],
    ) {
        rain_drops.sort_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap());
        for rain_drop in rain_drops.iter().rev() {
            let points = rain_drop.to_points_vec();
            for (index, (x, y, character)) in points.iter().enumerate() {
                let (width, height) = buffer.get_size();
                if *x < width as u16 && *y < height as u16 {
                    buffer.set(
                        *x as usize,
                        *y as usize,
                        Cell::new(
                            *character,
                            pick_color(&rain_drop.style, index, gradients),
                            pick_style(&rain_drop.style, index),
                        ),
                    );
                };
            }
        }
    }

    /// Add one more worm with decent chance
    pub fn add_one(&mut self) {
        if self.rain_drops.len() >= self.options.get_max_drops_number() as usize {
            return;
        };
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..=1.0) <= 0.3 {
            self.rain_drops.push(RainDrop::new(
                &self.options,
                self.rain_drops.len() + 1,
                &mut rng,
            ));
        };
    }
}

impl DigitalRainOptions {
    #[inline]
    pub fn get_width(&self) -> u16 {
        self.size.0
    }

    #[inline]
    pub fn get_height(&self) -> u16 {
        self.size.1
    }

    #[inline]
    pub fn get_min_drops_number(&self) -> u16 {
        self.drops_range.0
    }

    #[inline]
    pub fn get_max_drops_number(&self) -> u16 {
        self.drops_range.1
    }

    #[inline]
    pub fn get_min_speed(&self) -> u16 {
        self.speed_range.0
    }

    #[inline]
    pub fn get_max_speed(&self) -> u16 {
        self.speed_range.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sane_options() -> DigitalRainOptions {
        DigitalRainOptionsBuilder::default()
            .size((100, 100))
            .drops_range((20, 30))
            .speed_range((10, 20))
            .build()
            .unwrap()
    }

    #[test]
    fn create_new() {
        let foo = DigitalRain::new(get_sane_options());
        assert_eq!(foo.rain_drops.len(), 20);
    }

    #[test]
    fn no_diff() {
        let mut foo = DigitalRain::new(get_sane_options());
        let q = foo.get_diff();
        assert!(q.len() == 0);
    }

    #[test]
    fn some_diff_and_update() {
        let mut foo = DigitalRain::new(get_sane_options());
        foo.update();
        let q = foo.get_diff();
        assert!(q.len() > 0)
    }
}
