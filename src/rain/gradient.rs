fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 * (1.0 - t) + b as f32 * t).round() as u8
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn two_step_color_gradient(
    start_color: Color,
    middle_color: Color,
    end_color: Color,
    middle_point: usize,
    length: usize,
) -> Vec<Color> {
    let mut gradient = vec![];
    for i in 1..=length {
        let (r, g, b) = if i <= middle_point {
            let t = i as f32 / middle_point as f32;
            (
                lerp(start_color.r, middle_color.r, t),
                lerp(start_color.g, middle_color.g, t),
                lerp(start_color.b, middle_color.b, t),
            )
        } else {
            let t = (i - middle_point) as f32 / middle_point as f32;
            (
                lerp(middle_color.r, end_color.r, t),
                lerp(middle_color.g, end_color.g, t),
                lerp(middle_color.b, end_color.b, t),
            )
        };
        gradient.push(Color { r, g, b });
    }
    gradient
}
