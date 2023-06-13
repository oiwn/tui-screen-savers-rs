/// The state of each cell in the next generation is determined by the states
/// of that cell and its eight neighbors in the current generation:
///
/// Overpopulation:
///     If a living cell is surrounded by more than three living cells, it dies.
/// Underpopulation:
///     If a living cell is surrounded by fewer than two living cells, it dies.
/// Survival:
///     If a living cell is surrounded by two or three living cells, it survives.
/// Birth:
///     If a dead cell is surrounded by exactly three living cells,
///     it becomes a living cell.
use crate::buffer::{Buffer, Cell};
use crate::common::TerminalEffect;
use crossterm::style;
use derive_builder::Builder;
use rand::Rng;
use std::collections::HashMap;

#[derive(Builder, Default, Debug)]
#[builder(public, setter(into))]
pub struct ConwayLifeOptions {
    screen_size: (usize, usize),
    #[builder(default = "2000")]
    initial_cells: u32,
}

#[derive(Clone)]
pub struct LifeCell {
    pub character: char,
    // coords: (usize, usize),
}

pub struct ConwayLife {
    options: ConwayLifeOptions,
    buffer: Buffer,
    cells: HashMap<(usize, usize), LifeCell>,
    pub rng: rand::prelude::ThreadRng,
}

impl LifeCell {
    pub fn new(
        _options: &ConwayLifeOptions,
        _rng: &mut rand::prelude::ThreadRng,
    ) -> Self {
        Self { character: '*' }
    }
}

impl TerminalEffect for ConwayLife {
    fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        let mut curr_buffer = Buffer::new(
            self.options.screen_size.0 as usize,
            self.options.screen_size.1 as usize,
        );

        // fill current buffer
        self.fill_buffer(&mut curr_buffer);

        let diff = self.buffer.diff(&curr_buffer);
        self.buffer = curr_buffer;
        diff
    }

    fn update(&mut self) {
        // let mut new_buffer: Buffer =
        //     Buffer::new(self.options.screen_size.0, self.options.screen_size.1);

        // let mut next_cells = HashMap::new();

        // for ((x, y), cell) in self.cells.iter() {
        //     next_cells.insert((*x, *y), cell.clone());
        // }

        // for (index, _) in self.buffer.iter().enumerate() {
        //     let neighbors = get_neighbors_by_index(&self.buffer, index);
        //     if neighbors.is_empty() {
        //         continue;
        //     };
        //     let (nx, ny) = self.buffer.pos_of(index);
        //     let alive_neighbors = neighbors.len();

        //     if let Some(cell) = self.cells.get(&(nx, ny)) {
        //         // Survival: an alive cell with 2 or 3 alive neighbors stays alive
        //         if alive_neighbors == 2 || alive_neighbors == 3 {
        //             next_cells.insert((nx, ny), cell.clone());
        //         }
        //     } else {
        //         // Birth: a dead cell with exactly 3 alive neighbors becomes alive
        //         if alive_neighbors == 3 {
        //             next_cells.insert((nx, ny), LifeCell { character: 'X' });
        //             // Replace 'X' with the desired initial state
        //         }
        //     };
        // }

        // for ((x, y), _) in next_cells.iter() {
        //     new_buffer.set(
        //         *x,
        //         *y,
        //         Cell {
        //             symbol: '*',
        //             color: style::Color::Green,
        //             attr: style::Attribute::Bold,
        //         },
        //     );
        // }

        // self.cells = next_cells;
        // self.buffer = new_buffer;
    }
}

impl ConwayLife {
    pub fn new(options: ConwayLifeOptions) -> Self {
        let mut rng = rand::thread_rng();
        let mut buffer = Buffer::new(options.screen_size.0, options.screen_size.1);

        let mut cells = HashMap::new();
        for _ in 0..options.initial_cells {
            let lc = LifeCell::new(&options, &mut rng);
            let x = rng.gen_range(0..options.screen_size.0);
            let y = rng.gen_range(0..options.screen_size.1);

            cells.insert((x, y), lc);
            buffer.set(
                x, // cell.coords.0,
                y, // cell.coords.1,
                Cell::new('*', style::Color::Green, style::Attribute::Bold),
            )
        }

        Self {
            options,
            buffer,
            cells,
            rng,
        }
    }

    pub fn fill_buffer(&mut self, buffer: &mut Buffer) {
        for ((x, y), _cell) in self.cells.iter() {
            buffer.set(
                *x,
                *y,
                Cell::new('*', style::Color::Green, style::Attribute::Bold),
            )
        }
    }
}

pub fn get_neighbors_by_index(buf: &Buffer, index: usize) -> Vec<(usize, &Cell)> {
    let mut neighbors = Vec::new();
    let (x, y) = buf.pos_of(index);
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue; // Skip the cell itself
            }
            let nx = x as i32 + i;
            let ny = y as i32 + j;
            // Check if the coordinates are within the buffer bounds
            if nx >= 0 && nx < buf.width as i32 && ny >= 0 && ny < buf.height as i32
            {
                let idx = nx as usize + ny as usize * buf.width;
                neighbors.push((idx, &buf.buffer[idx]));
            }
        }
    }
    neighbors
}

pub fn get_neighbors_by_coords(
    buf: &Buffer,
    x: usize,
    y: usize,
) -> Vec<(usize, usize, &Cell)> {
    let mut neighbors = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue; // Skip the cell itself
            }
            let nx = x as i32 + i;
            let ny = y as i32 + j;
            // Check if the coordinates are within the buffer bounds
            if nx >= 0 && nx < buf.width as i32 && ny >= 0 && ny < buf.height as i32
            {
                neighbors.push((
                    nx as usize,
                    ny as usize,
                    &buf.buffer[nx as usize + ny as usize * buf.width],
                ));
            }
        }
    }
    neighbors
}
