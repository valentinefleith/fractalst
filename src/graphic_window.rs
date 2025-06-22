use crate::context::Context;
use crate::context::Fractal;
use crate::renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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
            .window(window_title, height, width)
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
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => self.event_handler(event),
                }
            }
            ::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    pub fn event_handler(&mut self, event: Event) {
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
                _ => {}
            },
            _ => {}
        }
    }
}
