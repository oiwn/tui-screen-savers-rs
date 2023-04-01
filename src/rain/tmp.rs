fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 * (1.0 - t) + b as f32 * t).round() as u8
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn two_step_color_gradient(length: usize) -> Vec<Color> {
    let start_color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    let middle_color = Color { r: 0, g: 255, b: 0 };
    let end_color = Color {
        r: 10,
        g: 10,
        b: 10,
    };

    let half_length = length / 2;

    let mut gradient = vec![];
    for i in 1..=length {
        let (r, g, b) = if i <= half_length {
            let t = i as f32 / half_length as f32;
            (
                lerp(start_color.r, middle_color.r, t),
                lerp(start_color.g, middle_color.g, t),
                lerp(start_color.b, middle_color.b, t),
            )
        } else {
            let t = (i - half_length) as f32 / half_length as f32;
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
