extern crate sdl2;

use fractalst::graphic_window::GraphicWindow;
use fractalst::renderer::HEIGHT;
use fractalst::renderer::WIDTH;

fn main() -> Result<(), String> {
    let mut graphic_window = GraphicWindow::new("hello-rustsdl2", HEIGHT, WIDTH)?;
    graphic_window.run()?;
    Ok(())
}
