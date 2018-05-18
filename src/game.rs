use piston_window::*;

use tank::Tank;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLU: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct Game {
    player1: Tank,
    player2: Tank,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Tank::new(BLU),
            player2: Tank::new(BLU),
        }
    }
    pub fn on_update(&mut self, u: UpdateArgs) {
        self.player1.on_update(u);
        self.player2.on_update(u);
    }
    pub fn on_draw(&mut self, e: &Event, w: &mut PistonWindow) {
        self.player1.on_draw(&e, w);
        self.player2.on_draw(&e, w);
    }
    pub fn on_move(&mut self, btn_args: ButtonArgs) {
        self.player1.on_move(btn_args);
        self.player2.on_move(btn_args);
    }
}
