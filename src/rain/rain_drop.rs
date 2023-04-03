use once_cell::sync::Lazy;
use rand::{
    self,
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};
use std::{collections::HashMap, time::Duration};

/// Characters in form of hashmap with label as key
static CHARACTERS_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("digits", "012345789");
    m.insert("punctuation", r#":・."=*+-<>"#);
    // m.insert("kanji", "日"); // Somehow it causing blinks - too wide
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

pub enum RainDropStyle {
    Front,
    Middle,
    Back,
    Fading,
    Gradient,
}

pub struct RainDrop {
    pub worm_id: usize,
    pub body: Vec<char>,
    pub vw_style: RainDropStyle,
    pub fx: f32,
    pub fy: f32,
    pub max_length: usize,
    pub visible_pos: usize, // used to correctly track position for styling
    pub finish: bool,
    pub speed: u16,
}

impl Distribution<RainDropStyle> for Standard {
    /// Choose from range
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RainDropStyle {
        match rng.gen_range(0..=6) {
            0 => RainDropStyle::Front,
            1 => RainDropStyle::Middle,
            2 => RainDropStyle::Back,
            3 => RainDropStyle::Fading,
            _ => RainDropStyle::Gradient,
        }
    }
}

/// Set of operations to make worm displyaing and moving
impl RainDrop {
    /// Create new worm with sane random defaults
    pub fn new(
        w: u16,
        h: u16,
        worm_id: usize,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Self {
        // pick random first character
        let body: Vec<char> = vec![*CHARACTERS.choose(rng).unwrap()];
        let vw_style: RainDropStyle = rand::random();
        let fx: f32 = rng.gen_range(1..=w) as f32;
        let fy: f32 = rng.gen_range(1..=h / 2) as f32;
        let max_length: usize = rng.gen_range(MIN_WORM_LENGTH..=(h / 2)) as usize;

        let speed: u16 = rng.gen_range(SPEED_RANGE.0..=SPEED_RANGE.1);
        let visible_pos = 0;
        let finish = false;

        Self::from_values(
            worm_id,
            body,
            vw_style,
            fx,
            fy,
            max_length,
            speed,
            visible_pos,
            finish,
        )
    }

    /// Create new worm from values
    #[inline]
    pub fn from_values(
        worm_id: usize,
        body: Vec<char>,
        vw_style: RainDropStyle,
        fx: f32,
        fy: f32,
        max_length: usize,
        speed: u16,
        visible_pos: usize,
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
            visible_pos,
            finish,
        }
    }

    /// Convert float into screen coordinates
    #[inline]
    pub fn to_point(&self) -> (u16, u16) {
        let x = self.fx.round() as u16;
        let y = self.fy.round() as u16;
        (x, y)
    }

    /// Receive vector of coordinates of RainDrop body
    pub fn to_points_vec(&self) -> Vec<(u16, u16, char)> {
        let mut points = vec![];
        let (head_x, head_y) = self.to_point();
        for (index, character) in self.body.iter().enumerate() {
            let yy = head_y as i16 - index as i16;
            if yy >= 1 {
                points.push((head_x as u16, yy as u16, character.clone()));
            } else {
                break;
            };
        }
        points
    }

    /// Reset worm to the sane defaults
    fn reset(&mut self, w: u16, h: u16, rng: &mut rand::prelude::ThreadRng) {
        self.body.clear();
        self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone());
        self.vw_style = rand::random();
        self.fy = 1.0;
        self.fx = rng.gen_range(1..=w) as f32;
        self.speed = rng.gen_range(SPEED_RANGE.0..=SPEED_RANGE.1);
        self.finish = false;
        self.visible_pos = 0;
        self.max_length = rng.gen_range(MIN_WORM_LENGTH..=(h / 2)) as usize;
    }

    /// Grow condition
    fn grow_condition(&self) -> bool {
        self.speed > 8
    }

    /// Grow up matrix worm characters array
    fn grow(&mut self, head_y: u16, rng: &mut rand::prelude::ThreadRng) {
        match self.grow_condition() {
            true => self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone()),
            false => {
                // if position on screen not changed, do not grow body vector
                let delta: i16 = head_y as i16 - self.fy.round() as i16;
                if delta > 0 {
                    self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone());
                }
            }
        };

        if self.body.len() > self.max_length as usize {
            self.body.truncate(self.max_length as usize);
        }
    }

    /// Update rain drops to change position/grow etc
    /// there can be 4 cases:
    /// rain drop vector not yet fully come from top
    /// rain drop vector somewhere in the middle of the scren
    /// rain drop vector reach bottom and need to fade out
    /// raid drop vector tail out of screen rect visibility
    ///
    /// Note that rain drop coordiantes can be outside bounds defined
    /// by screen width and height, this should be handled during draw process
    pub fn update(
        &mut self,
        w: u16,
        h: u16,
        dt: Duration,
        rng: &mut rand::prelude::ThreadRng,
    ) {
        // NOTE: looks like guard, but why i even need it here?
        if self.body.len() == 0 {
            self.reset(w, h, rng);
            return;
        }

        // new fy coordinate
        let fy = self.fy + (self.speed as f32 * dt.as_millis() as f32) / 1000.0;

        // calculate head and tail y coordinate
        let head_y = fy.round() as u16;
        let tail_y = fy.round() as i16 - self.body.len() as i16;

        if tail_y <= 1 {
            // not fully come out from top
            self.grow(head_y, rng);
            self.fy = fy;
            return;
        };

        if (head_y < h) && (tail_y > 1) {
            // somewhere in the middle
            self.grow(head_y, rng);
            self.fy = fy;
            return;
        };

        if head_y >= h {
            // come to bottom
            self.finish = true;
            // self.vw_style = RainDropStyle::Fading;
            self.visible_pos += (fy - self.fy).round() as usize;
            self.fy = fy;
        };

        // NOTE: need this to reset
        if tail_y as u16 >= h {
            self.reset(w, h, rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_and_reset() {
        let mut rng = rand::thread_rng();
        let mut new_worm = RainDrop::new(100, 100, 1 as usize, &mut rng);
        assert_eq!(new_worm.body.len(), 1);
        assert_eq!(new_worm.finish, false);

        new_worm.reset(100, 100, &mut rng);
        assert_eq!(new_worm.fy, 1.0);
        assert_eq!(new_worm.body.len(), 1);
    }

    #[test]
    fn generate_a_lot_of_worms() {
        let mut rng = rand::thread_rng();
        let mut worms = vec![];
        for index in 1..=1000 {
            worms.push(RainDrop::new(100, 100, index, &mut rng));
        }
        assert_eq!(worms.len(), 1000);
    }

    #[test]
    fn to_point() {
        let new_worm = RainDrop::from_values(
            1,
            vec!['a'],
            RainDropStyle::Gradient,
            10.3,
            10.8,
            20,
            10,
            0,
            false,
        );
        let (x, y) = new_worm.to_point();
        assert_eq!(x, 10);
        assert_eq!(y, 11);
    }

    #[test]
    fn to_point_vec() {
        let new_worm = RainDrop::from_values(
            1,
            vec!['a', 'b', 'c'],
            RainDropStyle::Fading,
            10.0,
            10.0,
            10,
            8,
            0,
            false,
        );
        let points = new_worm.to_points_vec();
        assert_eq!(points.len(), 3);
    }

    #[test]
    fn grow() {
        let mut rng = rand::thread_rng();
        let mut new_worm = RainDrop::from_values(
            1,
            vec!['a'],
            RainDropStyle::Front,
            10.3,
            10.8,
            20,
            10,
            0,
            false,
        );
        new_worm.grow(10, &mut rng);
        assert_eq!(new_worm.body.len(), 2);
        assert_eq!(new_worm.body.get(1), Some(&'a'));

        let mut new_worm = RainDrop::from_values(
            1,
            vec!['b'],
            RainDropStyle::Middle,
            10.3,
            10.8,
            20,
            4,
            0,
            false,
        );
        new_worm.grow(12, &mut rng);
        assert_eq!(new_worm.body.len(), 2);
        assert_eq!(new_worm.body.get(1), Some(&'b'));
        new_worm.grow(11, &mut rng);
        assert_eq!(new_worm.body.len(), 2);

        let mut new_worm = RainDrop::from_values(
            1,
            vec!['c'],
            RainDropStyle::Back,
            10.3,
            10.8,
            3,
            4,
            0,
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
        let mut new_worm = RainDrop::from_values(
            1,
            vec!['c'],
            RainDropStyle::Back,
            10.3,
            10.8,
            3,
            8,
            0,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.fy.round() as u16, 19);
        assert_eq!(new_worm.body.len(), 2);

        // edge case when body len is 0 (why?)
        let mut new_worm = RainDrop::from_values(
            1,
            vec![],
            RainDropStyle::Middle,
            10.3,
            10.8,
            3,
            8,
            0,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.body.len(), 1);
        assert_eq!(new_worm.fy, 1.0); // should be reseted

        // when tail_y < 0
        let mut new_worm = RainDrop::from_values(
            1,
            vec!['a', 'b', 'c', 'd'],
            RainDropStyle::Fading,
            10.0,
            2.0,
            5,
            2,
            0,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.body.len(), 5);
        assert_eq!((new_worm.fy - new_worm.body.len() as f32) < 0.0, true);

        // when head_y > screen height
        let mut new_worm = RainDrop::from_values(
            1,
            vec!['a', 'b', 'c', 'd'],
            RainDropStyle::Fading,
            10.0,
            30.8,
            5,
            2,
            0,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.body.len(), 4);
        assert_eq!(new_worm.fy > 30.0, true);

        // when head_y > screen height and body len is 2
        let mut new_worm = RainDrop::from_values(
            1,
            vec!['a', 'b'],
            RainDropStyle::Fading,
            10.0,
            29.0,
            5,
            2,
            0,
            false,
        );
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.body.len(), 2);
        assert_eq!(new_worm.fy, 31.0);
        new_worm.update(30, 30, Duration::from_millis(1000), &mut rng);
        assert_eq!(new_worm.fy, 1.0); // should be reseted there
    }
}
