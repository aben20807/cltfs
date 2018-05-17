extern crate piston_window;

use piston_window::*;

fn main() {
    let (width, height) = (512, 512);
    let mut window: PistonWindow =
        WindowSettings::new("Title", [width, height]).exit_on_esc(true).build().unwrap();
    let mut rotation: f64 = 0.0;
    while let Some(event) = window.next() {
        if let Some(UpdateArgs { dt }) = event.update_args() {
            rotation += 6.0 * dt;
        }
        if let Some(Button::Mouse(mouse_button)) = event.press_args() {
            println!("Click!, {:?}", mouse_button);
        }
        window.draw_2d(&event, |c, g| {
            clear([1.0, 1.0, 1.0, 0.0], g);
            let center = c.transform.trans(256.0, 256.0);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center.rot_rad(rotation).trans(-50.0, -50.0), g);
        });
    }
}
