use rand::Rng;
use sdl2::pixels::Color;
pub fn generate_random_colors(n: u32) -> Vec<Color> {
    let mut rng = rand::rng();
    let mut colors: Vec<Color> = (0..n + 1)
        .map(|_| Color::RGB(rng.random(), rng.random(), rng.random()))
        .collect();

    colors.sort_by(|a, b| brightness(a).partial_cmp(&brightness(b)).unwrap());
    colors.push(Color::BLACK);
    colors
}

fn brightness(color: &Color) -> f32 {
    0.2126 * color.r as f32 + 0.7152 * color.g as f32 + 0.0722 * color.b as f32
}
