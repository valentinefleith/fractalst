extern crate sdl2;

use fractalst::constants::HEIGHT;
use fractalst::constants::WIDTH;
use fractalst::graphic_window::GraphicWindow;

fn main() -> Result<(), String> {
    let mut graphic_window = GraphicWindow::new("hello-rustsdl2", HEIGHT, WIDTH)?;
    graphic_window.run()?;
    Ok(())
}
