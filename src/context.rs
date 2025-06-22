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
    pub food: Point,
    pub color: Color,
}

impl Context {
    pub fn new() -> Context {
        Context {
            fractal_name: Fractal::Mandelbrot,
            food: Point(3, 3),
            color: Color::RED,
        }
    }
}
