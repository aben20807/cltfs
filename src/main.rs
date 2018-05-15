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
        window.draw_2d(&event, |c, g| {
            clear([1.0, 1.0, 1.0, 0.0], g);
            draw(&c, g);
        });
    }
}

fn draw(c: &Context, g: &mut G2d) {
    rectangle([1.0, 0.0, 0.0, 1.0],
              [50.0, 0.0, 50.0, 100.0], // x, y, w, h
              c.transform,
              g);
}
