use crate::context::Fractal;
use crate::context::{ComplexNb, Context, Point};
use crate::renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState, MouseWheelDirection};
use sdl2::Sdl;
use std::time::Duration;

pub struct GraphicWindow {
    sdl_context: Sdl,
    //video_subsystem: VideoSubsystem,
    pub renderer: Renderer,
    pub context: Context,
}

impl GraphicWindow {
    pub fn new(window_title: &str, height: u32, width: u32) -> Result<GraphicWindow, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(window_title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let renderer = Renderer::new(window)?;
        let context = Context::new();
        Ok(GraphicWindow {
            sdl_context,
            renderer,
            context,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.renderer.draw(&self.context)?;
        let mut event_pump = self.sdl_context.event_pump()?;
        'running: loop {
            let mouse_position = event_pump.mouse_state();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::MouseWheel { y, direction, .. } => {
                        self.mouse_events_handler(y, direction, mouse_position)
                    }
                    _ => self.key_event_handler(event),
                }
            }
            ::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    pub fn mouse_events_handler(
        &mut self,
        y: i32,
        direction: MouseWheelDirection,
        mouse_position: MouseState,
    ) {
        let mouse_pos_before_zoom: ComplexNb = self
            .renderer
            .rescale_point(Point(mouse_position.x(), mouse_position.y()), &self.context);
        self.context.zoom *= 0.95;
        let mouse_pos_after_zoom: ComplexNb = self
            .renderer
            .rescale_point(Point(mouse_position.x(), mouse_position.y()), &self.context);

        self.context.shift_x += (mouse_pos_before_zoom.real - mouse_pos_after_zoom.real);
        self.context.shift_y += (mouse_pos_before_zoom.imag - mouse_pos_after_zoom.imag);
        self.renderer.draw(&self.context).unwrap();
    }

    pub fn key_event_handler(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Space => {
                    self.context.fractal_name = match self.context.fractal_name {
                        Fractal::Mandelbrot => Fractal::Julia,
                        Fractal::Julia => Fractal::Mandelbrot,
                    };
                    self.renderer.draw(&self.context).unwrap();
                }
                Keycode::P => {
                    self.context.zoom /= 1.2;
                    self.renderer.draw(&self.context).unwrap();
                }
                Keycode::M => {
                    self.context.zoom *= 1.2;
                    self.renderer.draw(&self.context).unwrap();
                }
                Keycode::RIGHT => {
                    self.context.shift_x += (self.context.zoom * 0.2);
                    self.renderer.draw(&self.context).unwrap();
                }
                Keycode::LEFT => {
                    self.context.shift_x -= (self.context.zoom * 0.2);
                    self.renderer.draw(&self.context).unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
