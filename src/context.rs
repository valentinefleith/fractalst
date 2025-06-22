use crate::colors;
use crate::constants::MAX_ITERATIONS;
use sdl2::pixels::Color;

pub enum Fractal {
    Mandelbrot,
    Julia,
}

pub struct Point(pub i32, pub i32);

pub struct ComplexNb {
    pub real: f64,
    pub imag: f64,
}

pub struct Context {
    pub fractal_name: Fractal,
    pub zoom: f64,
    pub shift_x: f64,
    pub shift_y: f64,
    pub colors: Vec<Color>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            fractal_name: Fractal::Mandelbrot,
            zoom: 1.0,
            colors: colors::generate_random_colors(MAX_ITERATIONS),
            shift_x: 0.,
            shift_y: 0.,
        }
    }
}
