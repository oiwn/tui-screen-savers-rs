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

static MIN_WORM_LENGTH: u16 = 2;
static SPEED_RANGE: (u16, u16) = (2, 12);

#[derive(Clone, Debug)]
pub enum VerticalWormStyle {
    Front,
    Middle,
    Back,
}

#[derive(Debug)]
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
        match rng.gen_range(0..=2) {
            0 => VerticalWormStyle::Front,
            1 => VerticalWormStyle::Middle,
            _ => VerticalWormStyle::Back,
        }
    }
}

/// Set of operations to make worm displyaing and moving
impl VerticalWorm {
    pub fn new(
        w: u16,
        h: u16,
        worm_id: usize,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Self {
        // pick random first character
        let body: Vec<char> = vec![*CHARACTERS.choose(rng).unwrap()];

        Self {
            worm_id,
            body,
            vw_style: rand::random(),
            fx: rng.gen_range(0..w) as f32,
            fy: rng.gen_range(0..h - 1) as f32,
            max_length: rng.gen_range(MIN_WORM_LENGTH..=(h / 2)),
            speed: rng.gen_range(SPEED_RANGE.0..=SPEED_RANGE.1),
            finish: false,
        }
    }

    pub fn to_points(&self) -> (u16, u16) {
        let x = self.fx.round() as u16;
        let y = self.fy.round() as u16;
        (x, y)
    }

    fn reset(&mut self, w: u16, h: u16, rng: &mut rand::prelude::ThreadRng) {
        self.body.clear();
        self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone());
        self.fy = 0.0;
        self.fx = rng.gen_range(0..w) as f32;
        self.speed = rng.gen_range(SPEED_RANGE.0..=SPEED_RANGE.1);
        self.finish = false;
        self.max_length = rng.gen_range(MIN_WORM_LENGTH..=(h / 2));
    }

    /// Grow up matrix worm characters array
    fn grow(&mut self, _head: u16, rng: &mut rand::prelude::ThreadRng) {
        // let delta: i16 = head as i16 - self.fy.round() as i16;
        self.body.insert(0, CHARACTERS.choose(rng).unwrap().clone());
        if self.body.len() >= self.max_length as usize {
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

        if (self.body.len() == 0) || (self.finish == true) {
            self.reset(w, h, rng);
        }

        // new fy coordinate
        let fy = self.fy + (self.speed as f32 * dt.as_millis() as f32) / 1000.0;

        // calculate head and tail y coordinate
        let head = fy.round() as u16;
        let tail = fy.round() as i16 - self.body.len() as i16;

        if tail <= 0 {
            // not fully come from top
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
            /*
            // truncate vector so head will remain the same but cut the tail
            let new_body_len = self.body.len() as i16 - 1;
            if new_body_len >= 1 {
                self.max_length -= 1;
                self.body.truncate(new_body_len as usize);
                self.fy = (h - 1) as f32;
            } else {
                self.reset(w, h, rng);
            }
            */
            self.reset(w, h, rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let mut rng = rand::thread_rng();
        let _new_worm = VerticalWorm::new(100, 100, 1 as usize, &mut rng);
    }
}
