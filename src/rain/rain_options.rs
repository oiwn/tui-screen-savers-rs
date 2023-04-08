#[derive(Debug, PartialEq)]
pub struct DigitalRainOptions {
    pub size: (u16, u16),
    pub drops_range: (u16, u16),
    pub speed_range: (u16, u16),
}

#[derive(Default)]
pub struct DigitalRainOptionsBuilder {
    size: (u16, u16),
    drops_range: (u16, u16),
    speed_range: (u16, u16),
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

impl DigitalRainOptionsBuilder {
    pub fn new(size: (u16, u16)) -> DigitalRainOptionsBuilder {
        DigitalRainOptionsBuilder {
            size,
            drops_range: (10, 100),
            speed_range: (4, 16),
        }
    }

    pub fn drops_range(mut self, range: (u16, u16)) -> DigitalRainOptionsBuilder {
        self.drops_range.0 = range.0;
        self.drops_range.1 = range.1;
        self
    }

    pub fn speed_range(mut self, range: (u16, u16)) -> DigitalRainOptionsBuilder {
        self.speed_range.0 = range.0;
        self.speed_range.1 = range.1;
        self
    }

    pub fn build(self) -> DigitalRainOptions {
        DigitalRainOptions {
            size: self.size,
            drops_range: self.drops_range,
            speed_range: self.speed_range,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builder_test() {
        let foo = DigitalRainOptions {
            size: (100, 100),
            drops_range: (100, 200),
            speed_range: (5, 15),
        };
        let foo_from_builder = DigitalRainOptionsBuilder::new((100, 100))
            .drops_range((100, 200))
            .speed_range((5, 15))
            .build();

        assert_eq!(foo, foo_from_builder);
    }
}
