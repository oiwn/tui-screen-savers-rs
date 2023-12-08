use crate::buffer::{Buffer, Cell};
use crate::common::TerminalEffect;
use crossterm::style;
use derive_builder::Builder;
use rand::{seq::SliceRandom, Rng};
use std::collections::{HashMap, HashSet};

#[derive(Builder, Default, Debug)]
#[builder(public, setter(into))]
pub struct MazeOptions {
    screen_size: (usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

pub struct Maze {
    options: MazeOptions,
    buffer: Buffer,
    cells: HashMap<(usize, usize), MazeCell>,
    pub rng: rand::prelude::ThreadRng,
}

#[derive(Clone)]
pub struct MazeCell {
    pub character: char,
    // coords: (usize, usize),
}

impl MazeCell {
    pub fn new(
        _options: &MazeOptions,
        _rng: &mut rand::prelude::ThreadRng,
    ) -> Self {
        Self { character: '#' }
    }
}

impl TerminalEffect for Maze {
    fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        let mut curr_buffer =
            Buffer::new(self.options.screen_size.0, self.options.screen_size.1);

        // fill current buffer
        self.fill_buffer(&mut curr_buffer);

        let diff = self.buffer.diff(&curr_buffer);
        self.buffer = curr_buffer;
        diff
    }

    fn update(&mut self) {
        #[allow(unused_assignments)]
        let mut next_cells = HashMap::new();

        next_cells = self.cells.clone();
        self.cells = next_cells;
    }
}

impl Maze {
    pub fn new(options: MazeOptions) -> Self {
        let mut rng = rand::thread_rng();
        let mut buffer = Buffer::new(options.screen_size.0, options.screen_size.1);

        buffer.fill_with(&Cell {
            symbol: ' ',
            color: style::Color::Green,
            attr: style::Attribute::Reset,
        });

        let maze = wilsons_algorithm(
            options.screen_size.0 as isize,
            options.screen_size.1 as isize,
        );

        let mut cells = HashMap::new();
        for (point_a, point_b) in maze {
            let mc = MazeCell::new(&options, &mut rng);
            cells.insert((point_a.x as usize, point_a.y as usize), mc);
            let mc = MazeCell::new(&options, &mut rng);
            cells.insert((point_b.x as usize, point_b.y as usize), mc);
        }

        Self {
            options,
            buffer,
            cells,
            rng,
        }
    }

    pub fn fill_buffer(&mut self, buffer: &mut Buffer) {
        for ((x, y), cell) in self.cells.iter() {
            buffer.set(
                *x,
                *y,
                Cell::new(
                    cell.character,
                    style::Color::Green,
                    style::Attribute::Bold,
                ),
            )
        }
    }
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

fn wilsons_algorithm(width: isize, height: isize) -> HashSet<(Point, Point)> {
    let mut rng = rand::thread_rng();
    let mut maze: HashSet<(Point, Point)> = HashSet::new();
    let mut visited = HashSet::new();

    // Start with a random cell
    let start = Point {
        x: rng.gen_range(0..width),
        y: rng.gen_range(0..height),
    };
    visited.insert(start);

    for _ in 0..width * height {
        let mut current = Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        };
        if visited.contains(&current) {
            continue;
        }

        let mut path = vec![current];
        while !visited.contains(&current) {
            let neighbors = current
                .neighbors()
                .into_iter()
                .filter(|p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height)
                .collect::<Vec<_>>();
            current = *neighbors.choose(&mut rng).unwrap();
            if let Some(pos) = path.iter().position(|&p| p == current) {
                path.truncate(pos + 1);
            } else {
                path.push(current);
            }
        }

        for window in path.windows(2) {
            if let [a, b] = *window {
                maze.insert((a, b));
                maze.insert((b, a));
                visited.insert(a);
                visited.insert(b);
            }
        }
    }

    maze
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_neighbors_by_index() {
        let _buf = Buffer::new(3, 3);

        for _x in 0..3 {
            for _y in 0..3 {
                // let res = get_neighbors_by_index(&buf, buf.index_of(x, y));
                // assert!(res.is_empty());
            }
        }
    }
}
