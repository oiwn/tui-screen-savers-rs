use crate::buffer::{Buffer, Cell};
use crate::common::TerminalEffect;
use crossterm::style;
use derive_builder::Builder;
use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, Rng};
use std::collections::{HashMap, HashSet, VecDeque};

/// Characters in form of hashmap with label as key
static CHARACTERS_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("punctuation", r#":."=*+-<>"#);
    m.insert("katakana", "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ");
    m.insert("other", "¦çﾘｸ");
    m
});

/// Characters to draw more interesing view
static CHARACTERS: Lazy<Vec<char>> = Lazy::new(|| {
    let mut v = Vec::new();
    for (_, chars) in CHARACTERS_MAP.iter() {
        v.append(&mut chars.chars().collect());
    }
    v
});

#[derive(Builder, Default, Debug, Clone)]
#[builder(public, setter(into))]
pub struct MazeOptions {
    screen_size: (u16, u16),
}

pub struct Maze {
    options: MazeOptions,
    buffer: Buffer,
    initial_walls: Buffer,
    paths: HashSet<(usize, usize)>,
    stack: VecDeque<(isize, isize)>,
    maze_complete: bool,
    pub rng: rand::prelude::ThreadRng,
}

impl TerminalEffect for Maze {
    fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        if self.maze_complete {
            self.reset();
            return Vec::new();
        }
        let mut curr_buffer = self.initial_walls.clone();
        let mut modified_cells = HashSet::new();
        // Randomly change 5 distinct cells
        while modified_cells.len() < 3 {
            let x = self.rng.gen_range(0..curr_buffer.width);
            let y = self.rng.gen_range(0..curr_buffer.height);

            if modified_cells.insert((x, y)) {
                let random_char =
                    CHARACTERS[self.rng.gen_range(0..CHARACTERS.len())];
                let random_color = style::Color::Rgb {
                    r: self.rng.gen_range(0..200) as u8,
                    g: self.rng.gen_range(0..256) as u8,
                    b: self.rng.gen_range(0..200) as u8,
                };
                self.initial_walls.set(
                    x,
                    y,
                    Cell::new(random_char, random_color, style::Attribute::Bold),
                );
            }
        }

        for (x, y) in self.paths.iter() {
            curr_buffer.set(
                *x,
                *y,
                Cell::new('█', style::Color::White, style::Attribute::Reset),
            )
        }

        let diff = self.buffer.diff(&curr_buffer);
        self.buffer = curr_buffer;
        diff
    }

    fn update(&mut self) {
        if self.maze_complete {
            return;
        }

        if let Some((x, y)) = self.stack.pop_back() {
            let directions = [(2, 0), (0, 2), (-2, 0), (0, -2)]; // Skip one cell to maintain walls
            let mut shuffled_directions = directions;
            shuffled_directions.shuffle(&mut self.rng);

            let mut moved = false;
            for &(dx, dy) in &shuffled_directions {
                let new_x = x + dx;
                let new_y = y + dy;

                // Check the cell to be carved and the wall between the current and new cell
                if self.is_valid_cell(new_x, new_y)
                    && self.is_valid_cell(x + dx / 2, y + dy / 2)
                    && !self.paths.contains(&(new_x as usize, new_y as usize))
                {
                    // Carve path for both the new cell and the wall between
                    self.carve_path(new_x, new_y);
                    self.carve_path(x + dx / 2, y + dy / 2);
                    // Push the current position back for backtracking
                    self.stack.push_back((x, y));
                    self.stack.push_back((new_x, new_y)); // Push the new position
                    moved = true;
                    break;
                }
            }

            if !moved {
                // If we didn't move, it means we're at a dead-end and need to backtrack
                self.stack.pop_back();
            }
        } else {
            // If the stack is empty, the maze is complete
            self.maze_complete = true;
        }
    }

    fn update_size(&mut self, width: u16, height: u16) {
        self.options.screen_size = (width, height);
    }

    fn reset(&mut self) {
        let mut new_effect = Self::new(self.options.clone());
        fill_initial_walls(&mut new_effect.initial_walls);
        new_effect.maze_complete = false;
        new_effect.paths.clear();
        new_effect.stack.clear();
        new_effect.rng = rand::thread_rng();

        let start_x = new_effect
            .rng
            .gen_range(0..self.options.screen_size.0 as isize);
        let start_y = new_effect
            .rng
            .gen_range(0..self.options.screen_size.1 as isize);
        new_effect.stack.push_back((start_x, start_y));
        *self = new_effect;
    }
}

impl Maze {
    pub fn new(options: MazeOptions) -> Self {
        let mut rng = rand::thread_rng();
        let buffer = Buffer::new(
            options.screen_size.0 as usize,
            options.screen_size.1 as usize,
        );

        let paths = HashSet::new();
        let start_x = rng.gen_range(0..options.screen_size.0 as isize);
        let start_y = rng.gen_range(0..options.screen_size.1 as isize);
        let mut stack = VecDeque::new();
        stack.push_back((start_x, start_y));

        let mut initial_walls = buffer.clone();
        fill_initial_walls(&mut initial_walls);

        Self {
            options,
            buffer,
            initial_walls,
            paths,
            stack,
            maze_complete: false,
            rng,
        }
    }

    fn is_valid_cell(&self, x: isize, y: isize) -> bool {
        x >= 0
            && y >= 0
            && (x as usize) < (self.options.screen_size.0 as usize)
            && (y as usize) < (self.options.screen_size.1 as usize)
    }

    fn carve_path(&mut self, x: isize, y: isize) {
        self.paths.insert((x as usize, y as usize));
    }
}

fn fill_initial_walls(buffer: &mut Buffer) {
    let mut rng = rand::thread_rng();
    for y in 0..buffer.height {
        for x in 0..buffer.width {
            let random_char = CHARACTERS[rng.gen_range(0..CHARACTERS.len())];
            let random_color = style::Color::Rgb {
                r: rng.gen_range(0..120) as u8,
                g: rng.gen_range(0..256) as u8,
                b: rng.gen_range(0..120) as u8,
            };
            buffer.set(
                x,
                y,
                Cell::new(random_char, random_color, style::Attribute::Bold),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_initial_state() {
        let options = MazeOptionsBuilder::default()
            .screen_size((3, 3))
            .build()
            .unwrap();
        let maze = Maze::new(options);

        // buffer correctly initialized
        let mut initialized_cells = 0;
        for cell in maze.buffer.iter() {
            if cell.symbol != ' ' {
                initialized_cells += 1;
            }
        }
        assert_eq!(initialized_cells, 0);
        assert_eq!(maze.initial_walls.buffer.len(), 9);

        // path and stack are empty, and maze is not completed
        assert!(maze.paths.is_empty());
        assert!(maze.stack.len() == 1);
        assert!(!maze.maze_complete);
    }

    #[test]
    fn check_flow() {
        let options = MazeOptionsBuilder::default()
            .screen_size((5, 5))
            .build()
            .unwrap();
        let mut maze = Maze::new(options);
        maze.update();
        let diff = maze.get_diff();
        assert_eq!(diff.len(), 25);

        // /* NOTE: why no path_cells set in buffer after update?
        // maze.update();
        // let _ = maze.get_diff();
        // buffer correctly processed
        let mut path_cells = 0;
        for cell in maze.buffer.iter() {
            if cell.symbol != '█' {
                path_cells += 1;
            }
        }
        assert_eq!(path_cells, 23);
        // */
    }
}
