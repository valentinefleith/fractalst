use crate::constants::{DOT_SIZE_IN_PXS, HEIGHT, WIDTH, X_MAX, X_MIN, Y_MAX, Y_MIN};
use crate::context::{ComplexNb, Context, Point};
use crate::fractal::mandelbrot::compute_mandelbrot;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;
        Ok(())
    }

    pub fn draw(&mut self, context: &Context) -> Result<(), String> {
        // self.draw_background(context);
        self.draw_mandelbrot(context)?;
        //self.draw_food(context)?;
        self.canvas.present();
        Ok(())
    }

    // fn draw_background(&mut self, context: &Context) {
    //     let color = match context.fractal_name {
    //         Fractal::Mandelbrot => Color::RGB(30, 30, 30),
    //         Fractal::Julia => Color::RGB(0, 0, 0),
    //     };
    //     self.canvas.set_draw_color(color);
    //     self.canvas.clear();
    // }
    //
    fn rescale_point(&mut self, point: Point) -> ComplexNb {
        ComplexNb {
            real: point.0 as f64 * (X_MAX - X_MIN) / WIDTH as f64 + X_MIN,
            imag: point.1 as f64 * (Y_MAX - Y_MIN) / HEIGHT as f64 + Y_MIN,
        }
    }

    fn draw_mandelbrot(&mut self, _context: &Context) -> Result<(), String> {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let current_px_color: Color =
                    compute_mandelbrot(self.rescale_point(Point(i as i32, j as i32)));
                self.canvas.set_draw_color(current_px_color);
                self.draw_dot(&Point(i as i32, j as i32))?;
            }
        }
        Ok(())
    }

    // fn draw_food(&mut self, context: &Context) -> Result<(), String> {
    //     self.canvas.set_draw_color(Color::RED);
    //     self.draw_dot(&context.food)?;
    //     Ok(())
    // }
}
