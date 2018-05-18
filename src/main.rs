extern crate piston_window;

use piston_window::*;

mod game;
use game::Game;

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
        } else if let Some(btn_args) = event.button_args() {
            game.on_move(btn_args);
        }
        game.on_draw(&event, &mut window);
    }
}
