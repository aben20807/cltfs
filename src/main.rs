extern crate piston_window;

use piston_window::*;

fn main() {
    let (width, height) = (512, 512);
    let mut window: PistonWindow =
        WindowSettings::new("Title", [width, height]).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        if let Some(Button::Mouse(mouse_button)) = event.press_args() {
            println!("Click!, {:?}", mouse_button);
        }
    }
}
