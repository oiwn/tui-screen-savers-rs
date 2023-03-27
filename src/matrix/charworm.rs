use once_cell::sync::Lazy;
use rand::{
    self,
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};
use std::{collections::HashMap, time::Duration};

static CHARACTERS_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("digits", "012345789");
    m.insert("punctuation", r#":・."=*+-<>"#);
    // m.insert("kanji", "日"); // Somehow it causing blinks
    m.insert("katakana", "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ");
    m.insert("other", "¦çﾘｸ");
    m
});

/// Characters used to form kinda-canonical matrix effect
static CHARACTERS: Lazy<Vec<char>> = Lazy::new(|| {
    let mut v = Vec::new();
    for (_, chars) in CHARACTERS_MAP.iter() {
        v.append(&mut chars.chars().collect());
    }
    v
});

static MIN_WORM_LENGTH: u16 = 10;
static SPEED_RANGE: (u16, u16) = (2, 20);

pub enum VerticalWormStyle {
    Front,
    Middle,
    Back,
    Fading,
    Gradient,
}

pub struct VerticalWorm {
    pub worm_id: usize,
    pub body: Vec<char>,
    pub vw_style: VerticalWormStyle,
    pub fx: f32,
    pub fy: f32,
    pub max_length: u16,
    pub finish: bool,
    pub speed: u16,
}

impl Distribution<VerticalWormStyle> for Standard {
    /// Choose from range
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VerticalWormStyle {
        match rng.gen_range(0..=6) {
            0 => VerticalWormStyle::Front,
            1 => VerticalWormStyle::Middle,
            2 => VerticalWormStyle::Back,
            3 => VerticalWormStyle::Fading,
            _ => VerticalWormStyle::Gradient,
        }
    }
}

/// Set of operations to make worm displyaing and moving
impl VerticalWorm {
    /// Create new worm with sane random defaults
    pub fn new(
        w: u16,
        h: u16,
        worm_id: usize,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Self {
        // pick random first character
        let body: Vec<char> = vec![*CHARACTERS.choose(rng).unwrap()];
        let vw_style: VerticalWormStyle = rand::random();
        let fx: f32 = rng.gen_range(0..w) as f32;
        let fy: f32 = rng.gen_range(0..h / 2) as f32;
        let max_length: u16 = rng.gen_range(MIN_WORM_LENGTH..=(h / 2));
        let speed: u16 = rng.gen_range(SPEED_RANGE.0..=SPEED_RANGE.1);
        let finish = false;

        Self::from_values(
            worm_id, body, vw_style, fx, fy, max_length, speed, finish,
        )
    }

    /// Create new worm from values
    pub fn from_values(
        worm_id: usize,
        body: Vec<char>,
        vw_style: VerticalWormStyle,
        fx: f32,
        fy: f32,
        max_length: u16,
        speed: u16,
        finish: bool,
    ) -> Self {
        Self {
            worm_id,
            body,
            vw_style,
            fx,
            fy,
            max_length,
            speed,
            finish,
        }
    }

    /// Convert float points into screen coordinates
    pub fn to_points(&self) -> (u16, u16) {
        let x = self.fx.round() as u16;
        let y = self.fy.round() as u16;
        (x, y)
    }

    /// Reset worm to the sane defaults
    fn reset(&mut self, w: u16, h: u16, rng: &mut rand::prelude::ThreadRng) {
        self.body.clear();
        self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone());
        self.fy = 0.0;
        self.fx = rng.gen_range(0..w) as f32;
        self.speed = rng.gen_range(SPEED_RANGE.0..=SPEED_RANGE.1);
        self.finish = false;
        self.max_length = rng.gen_range(MIN_WORM_LENGTH..=(h / 2));
    }

    /// Grow condition
    fn grow_condition(&self) -> bool {
        self.speed > 8
    }

    /// Grow up matrix worm characters array
    fn grow(&mut self, head: u16, rng: &mut rand::prelude::ThreadRng) {
        match self.grow_condition() {
            true => self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone()),
            false => {
                // if position on screen not changed, do not grow body vector
                let delta: i16 = head as i16 - self.fy.round() as i16;
                if delta > 0 {
                    self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone());
                }
            }
        };

        if self.body.len() > self.max_length as usize {
            self.body.truncate(self.max_length as usize);
        }
    }

    pub fn update(
        &mut self,
        w: u16,
        h: u16,
        dt: Duration,
        rng: &mut rand::prelude::ThreadRng,
    ) {
        // there can be 3 cases:
        // worm vector not yet fully come from top
        // worm vector somewhere in the middle of the scren
        // worm vector reach bottom and need to fade out

        // NOTE: looks like guard, but why i even need it here?
        if self.body.len() == 0 {
            self.reset(w, h, rng);
            return;
        }

        // new fy coordinate
        let fy = self.fy + (self.speed as f32 * dt.as_millis() as f32) / 1000.0;

        // calculate head and tail y coordinate
        let head = fy.round() as u16;
        let tail = fy.round() as i16 - self.body.len() as i16;

        if tail <= 0 {
            // not fully come out from top
            self.grow(head, rng);
            self.fy = fy;
            return;
        }

        if (head < h) && (tail > 0) {
            // somewhere in the middle
            self.grow(head, rng);
            self.fy = fy;
            return;
        }

        if head >= h {
            // come to bottom
            self.finish = true;
            self.vw_style = VerticalWormStyle::Back;
            // truncate vector so head will remain the same but cut the tail
            let new_body_len = self.body.len() as i16 - 1;
            if new_body_len >= 1 {
                self.max_length -= 1;
                self.body.truncate(new_body_len as usize);
                self.fy = (h - 1) as f32;
            } else {
                self.reset(w, h, rng);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_and_reset() {
        let mut rng = rand::thread_rng();
        let mut new_worm = VerticalWorm::new(100, 100, 1 as usize, &mut rng);
        assert_eq!(new_worm.body.len(), 1);
        assert_eq!(new_worm.finish, false);

        new_worm.reset(100, 100, &mut rng);
        assert_eq!(new_worm.fy, 0.0);
        assert_eq!(new_worm.body.len(), 1);
    }

    #[test]
    fn generate_a_lot_of_worms() {
        let mut rng = rand::thread_rng();
        let mut worms = vec![];
        for index in 1..=1000 {
            worms.push(VerticalWorm::new(100, 100, index, &mut rng));
        }
        assert_eq!(worms.len(), 1000);
    }

    #[test]
    fn to_points() {
        let new_worm = VerticalWorm::from_values(
            1,
            vec!['a'],
            VerticalWormStyle::Gradient,
            10.3,
            10.8,
            20,
            10,
            false,
        );
        let (x, y) = new_worm.to_points();
        assert_eq!(x, 10);
        assert_eq!(y, 11);
    }

    #[test]
    fn grow() {
        let mut rng = rand::thread_rng();
        let mut new_worm = VerticalWorm::from_values(
            1,
            vec!['a'],
            VerticalWormStyle::Front,
            10.3,
            10.8,
            20,
            10,
            false,
        );
        new_worm.grow(10, &mut rng);
        assert_eq!(new_worm.body.len(), 2);
        assert_eq!(new_worm.body.get(1), Some(&'a'));

        let mut new_worm = VerticalWorm::from_values(
            1,
            vec!['b'],
            VerticalWormStyle::Middle,
            10.3,
            10.8,
            20,
            4,
            false,
        );
        new_worm.grow(12, &mut rng);
        assert_eq!(new_worm.body.len(), 2);
        assert_eq!(new_worm.body.get(1), Some(&'b'));
        new_worm.grow(11, &mut rng);
        assert_eq!(new_worm.body.len(), 2);

        let mut new_worm = VerticalWorm::from_values(
            1,
            vec!['c'],
            VerticalWormStyle::Back,
            10.3,
            10.8,
            3,
            4,
            false,
        );
        new_worm.grow(12, &mut rng);
        new_worm.grow(12, &mut rng);
        new_worm.grow(12, &mut rng);
        new_worm.grow(12, &mut rng);
        assert_eq!(new_worm.body.len(), 3);
    }

    #[test]
    fn update() {
        let mut rng = rand::thread_rng();

        // nothing special worm update
        let mut new_worm = VerticalWorm::from_values(
            1,
            vec!['c'],
            VerticalWormStyle::Back,
            10.3,
            10.8,
            3,
            8,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.fy.round() as u16, 19);
        assert_eq!(new_worm.body.len(), 2);

        // edge case when body len is 0 (why?)
        let mut new_worm = VerticalWorm::from_values(
            1,
            vec![],
            VerticalWormStyle::Middle,
            10.3,
            10.8,
            3,
            8,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.body.len(), 1);
        assert_eq!(new_worm.fy, 0.0); // should be reseted

        // when tail_y < 0
        let mut new_worm = VerticalWorm::from_values(
            1,
            vec!['a', 'b', 'c', 'd'],
            VerticalWormStyle::Fading,
            10.0,
            2.0,
            5,
            2,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.body.len(), 5);
        assert_eq!((new_worm.fy - new_worm.body.len() as f32) < 0.0, true);

        // when head_y > screen height
        let mut new_worm = VerticalWorm::from_values(
            1,
            vec!['a', 'b', 'c', 'd'],
            VerticalWormStyle::Fading,
            10.0,
            30.8,
            5,
            2,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.body.len(), 3);
        assert_eq!(new_worm.fy < 30.0, true);

        // when head_y > screen height and body len is 2
        let mut new_worm = VerticalWorm::from_values(
            1,
            vec!['a', 'b'],
            VerticalWormStyle::Fading,
            10.0,
            30.8,
            5,
            2,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.body.len(), 1);
        assert_eq!(new_worm.fy, 29.0);
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.fy, 0.0); // should be reseted there
    }
}
