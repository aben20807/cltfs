use piston_window::*;

use tank::Tank;

pub struct Game {
    player1: Tank,
}

impl Game {
    pub fn new() -> Game {
        Game { player1: Tank::new() }
    }
    pub fn on_update(&mut self, u: UpdateArgs) {
        self.player1.on_update(u);
    }
    pub fn on_draw(&mut self, e: &Event, w: &mut PistonWindow) {
        self.player1.on_draw(&e, w);
    }
    pub fn on_move(&mut self, btn_args: ButtonArgs) {
        self.player1.on_move(btn_args);
    }
}
