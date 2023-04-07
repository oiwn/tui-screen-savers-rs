use crossterm::style;

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub symbol: char,
    pub color: style::Color,
    pub attr: style::Attribute,
}

/// Buffer implementation, coordinates unlike in crossterm started from [0, 0]
pub struct Buffer {
    width: usize,
    #[allow(dead_code)]
    height: usize,
    buffer: Vec<Cell>,
}

impl Cell {
    pub fn new(symbol: char, color: style::Color, attr: style::Attribute) -> Self {
        Self {
            symbol,
            color,
            attr,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            color: style::Color::Black,
            attr: style::Attribute::Reset,
        }
    }
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        // fill buffer with dafault values
        Self {
            width,
            height,
            buffer: vec![Cell::default(); width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let index = self.index_of(x, y);
        self.buffer[index] = cell;
    }

    pub fn set_from_coords(&mut self, x: usize, y: usize, cell: Cell) {
        let index = self.index_of(x - 1, y - 1);
        self.buffer[index] = cell;
    }

    #[allow(dead_code)]
    pub fn get_rect(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn index_of(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn pos_of(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    pub fn diff(&self, other: &Buffer) -> Vec<(usize, usize, Cell)> {
        let prev_buffer = &self.buffer;
        let next_buffer = &other.buffer;

        let mut updates: Vec<(usize, usize, Cell)> = vec![];

        for (i, (curr, prev)) in
            next_buffer.iter().zip(prev_buffer.iter()).enumerate()
        {
            if curr != prev {
                let (x, y) = self.pos_of(i);
                debug_assert!(x < self.width && y < self.height);
                updates.push((x, y, next_buffer[i]));
            }
        }

        updates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let buf = Buffer::new(20, 10);
        assert_eq!(buf.width, 20);
        assert_eq!(buf.height, 10);
        assert_eq!(buf.get_rect(), (20, 10));
    }

    #[test]
    fn diff() {
        let mut buf = Buffer::new(3, 3);
        let point = buf.index_of(0, 0);
        buf.buffer[point] =
            Cell::new('b', style::Color::Green, style::Attribute::NormalIntensity);
        let point = buf.index_of(0, 1);
        buf.buffer[point] =
            Cell::new('a', style::Color::Green, style::Attribute::NormalIntensity);

        let mut next_buf = Buffer::new(3, 3);
        let point = buf.index_of(0, 0);
        next_buf.buffer[point] = Cell::new(
            'c',
            style::Color::DarkGreen,
            style::Attribute::NormalIntensity,
        );
        let point = buf.index_of(0, 1);
        next_buf.buffer[point] =
            Cell::new('b', style::Color::Green, style::Attribute::NormalIntensity);
        let point = buf.index_of(0, 2);
        next_buf.buffer[point] =
            Cell::new('a', style::Color::Green, style::Attribute::NormalIntensity);

        let diff = buf.diff(&next_buf);
        assert_eq!(diff.len(), 3);
    }
}
