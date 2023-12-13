use crate::buffer::{Buffer, Cell};
use crate::common::TerminalEffect;
use crossterm::style;
use derive_builder::Builder;

#[derive(Builder, Default, Debug)]
#[builder(public, setter(into))]
pub struct CheckOptions {
    screen_size: (usize, usize),
}

pub struct Check {
    options: CheckOptions,
    buffer: Buffer,
}

impl TerminalEffect for Check {
    fn get_diff(&mut self) -> Vec<(usize, usize, Cell)> {
        let mut curr_buffer =
            Buffer::new(self.options.screen_size.0, self.options.screen_size.1);

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
}

impl Check {
    pub fn new(options: CheckOptions) -> Self {
        let mut buffer = Buffer::new(options.screen_size.0, options.screen_size.1);

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
