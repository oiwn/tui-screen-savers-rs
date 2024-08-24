use crate::buffer::{Buffer, Cell};
use crate::common::TerminalEffect;
use crossterm::style;
use derive_builder::Builder;

#[derive(Builder, Default, Debug, Clone)]
#[builder(public, setter(into))]
pub struct BlankOptions {
    screen_size: (u16, u16),
}

#[allow(dead_code)]
pub struct Blank {
    options: BlankOptions,
    buffer: Buffer,
}

impl TerminalEffect for Blank {
    fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        let mut curr_buffer = Buffer::new(
            self.options.screen_size.0 as usize,
            self.options.screen_size.1 as usize,
        );

        curr_buffer.fill_with(&Cell {
            symbol: '#',
            color: style::Color::Green,
            attr: style::Attribute::Reset,
        });

        let diff = self.buffer.diff(&curr_buffer);
        self.buffer = curr_buffer;
        diff
    }

    fn update(&mut self) {}

    fn update_size(&mut self, width: u16, height: u16) {
        self.options.screen_size = (width, height)
    }

    fn reset(&mut self) {
        *self = Self::new(self.options.clone());
    }
}

impl Blank {
    pub fn new(options: BlankOptions) -> Self {
        let mut buffer = Buffer::new(
            options.screen_size.0 as usize,
            options.screen_size.1 as usize,
        );

        buffer.fill_with(&Cell {
            symbol: '#',
            color: style::Color::Green,
            attr: style::Attribute::Reset,
        });

        Self { options, buffer }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn some_test() {}
}
