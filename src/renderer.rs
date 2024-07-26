use crate::context::{Context, Fractal, Point};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub const HEIGHT: u32 = 1000;
pub const WIDTH: u32 = 800;
pub const DOT_SIZE_IN_PXS: u32 = 1;

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
        self.draw_background(context);
        self.draw_mandelbrot(context)?; //self.draw_food(context)?;
        self.canvas.present();
        Ok(())
    }

    fn draw_background(&mut self, context: &Context) {
        let color = match context.fractal_name {
            Fractal::Mandelbrot => Color::RGB(30, 30, 30),
            Fractal::Julia => Color::RGB(0, 0, 0),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_mandelbrot(&mut self, context: &Context) -> Result<(), String> {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.canvas.set_draw_color(Color::WHITE);
                self.draw_dot(&Point(i as i32, j as i32))?;
            }
        }
        Ok(())
    }

    fn draw_food(&mut self, context: &Context) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;
        Ok(())
    }
}
