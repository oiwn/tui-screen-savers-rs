use crate::buffer::{Buffer, Cell};
use crate::common::TerminalEffect;
use crossterm::style;
use derive_builder::Builder;
use rand::Rng;

#[derive(Builder, Default, Debug)]
#[builder(public, setter(into))]
pub struct ConwayLifeOptions {
    screen_size: (usize, usize),
    #[builder(default = "100")]
    initial_cells: u32,
}

pub struct LifeCell {
    character: char,
    coords: (usize, usize),
}

pub struct ConwayLife {
    options: ConwayLifeOptions,
    buffer: Buffer,
    cells: Vec<LifeCell>,
    rng: rand::prelude::ThreadRng,
}

impl LifeCell {
    pub fn new(
        options: &ConwayLifeOptions,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Self {
        let x = rng.gen_range(0..options.screen_size.0);
        let y = rng.gen_range(0..options.screen_size.1);
        Self {
            character: '*',
            coords: (x, y),
        }
    }
}

impl TerminalEffect for ConwayLife {
    fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        let mut curr_buffer = Buffer::new(
            self.options.screen_size.0 as usize,
            self.options.screen_size.1 as usize,
        );

        // fill current buffer
        // first draw drops with bigger fy
        // Self::fill_buffer(&mut self.rain_drops, &mut curr_buffer, &self.gradients);
        self.fill_buffer(&mut curr_buffer);

        let diff = self.buffer.diff(&curr_buffer);
        self.buffer = curr_buffer;
        diff
    }
    fn update(&mut self) {
        let to_delete_indexes: Vec<usize> = vec![];

        for (index, cell) in self.cells.iter().enumerate() {
            let mut surrounding: usize = 0;

            let mut row_top = cell.coords.1 as i32 - 1;
            let mut row_bottom = cell.coords.1 as i32 + 1;
            let mut col_left = cell.coords.0 as i32 - 1;
            let mut col_right = cell.coords.0 as i32 - 1;

            if row_top < 0 {
                row_top = self.options.screen_size.1 as i32 - 1;
            };

            if row_bottom > self.options.screen_size.1 as i32 - 1 {
                row_bottom = 0;
            };

            if col_left < 0 {
                col_left = self.options.screen_size.0 as i32 - 1;
            };

            if col_right > self.options.screen_size.0 as i32 - 1 {
                col_right = 0;
            };

            // top row
            if self.buffer.get(col_left as usize, row_top as usize).symbol != ' ' {
                surrounding += 1;
            };

            if self.buffer.get(cell.coords.0, row_top as usize).symbol != ' ' {
                surrounding += 1;
            };

            if self.buffer.get(col_right as usize, row_top as usize).symbol != ' ' {
                surrounding += 1;
            };

            // middle row
            if self.buffer.get(col_left as usize, cell.coords.1).symbol != ' ' {
                surrounding += 1;
            };

            if self.buffer.get(col_right as usize, cell.coords.1).symbol != ' ' {
                surrounding += 1;
            };

            // bottom row
            if self
                .buffer
                .get(col_right as usize, row_bottom as usize)
                .symbol
                != ' '
            {
                surrounding += 1;
            };

            if self.buffer.get(cell.coords.0, row_bottom as usize).symbol != ' ' {
                surrounding += 1;
            };

            if self
                .buffer
                .get(col_right as usize, row_bottom as usize)
                .symbol
                != ' '
            {
                surrounding += 1;
            };
        }
    }
}

impl ConwayLife {
    pub fn new(options: ConwayLifeOptions) -> Self {
        let mut rng = rand::thread_rng();
        let buffer = Buffer::new(options.screen_size.0, options.screen_size.1);

        let mut cells = vec![];
        for _ in 0..options.initial_cells {
            let lc = LifeCell::new(&options, &mut rng);
            cells.push(lc);
        }

        Self {
            options,
            buffer,
            cells,
            rng,
        }
    }

    pub fn fill_buffer(&mut self, buffer: &mut Buffer) {
        for cell in self.cells.iter() {
            buffer.set(
                cell.coords.0,
                cell.coords.1,
                Cell::new('*', style::Color::Green, style::Attribute::Bold),
            )
        }
        /*
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
        */
    }
}
