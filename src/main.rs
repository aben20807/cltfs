extern crate piston_window;

use piston_window::*;

fn main() {
    let (width, height) = (512, 512);
    let mut window: PistonWindow =
        WindowSettings::new("Title", [width, height]).exit_on_esc(true).build().unwrap();
    let mut game = Game::new();
    while let Some(event) = window.next() {
        if let Some(upd_args) = event.update_args() {
            game.on_update(upd_args);
        } else if let Some(Button::Mouse(mouse_button)) = event.press_args() {
            println!("Click!, {:?}", mouse_button);
        } else if let Some(Button::Keyboard(key)) = event.press_args() {
            println!("Press!, {:?}", key);
        }
        game.on_draw(&event , &mut window);
    }
}

struct Game {
    rotation: f64
}

impl Game {
    fn new() -> Game {
        Game { rotation : 0.0 }
    }
    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;
    }
    fn on_draw(&mut self,
               event: &piston_window::Event,
               window: &mut PistonWindow) {
        window.draw_2d(event, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans(256.0, 256.0);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center.rot_rad(self.rotation).trans(-50.0, -50.0), g); // We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered
        });
    }
}
