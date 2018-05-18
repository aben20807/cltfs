use piston_window::*;

pub struct Tank {
    rotation: f64, x: f64, y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool,
}

impl Tank {
    pub fn new() -> Tank {
        Tank {
            rotation : 0.0, x : 0.0, y : 0.0,
            up_d: false, down_d: false, left_d: false, right_d: false,
        }
    }
    pub fn on_update(&mut self, u: UpdateArgs) {
        self.rotation += 3.0 * u.dt;
        if self.up_d {
            self.y += (-50.0) * u.dt;
        }
        if self.down_d {
            self.y += (50.0) * u.dt;
        }
        if self.left_d {
            self.x += (-50.0) * u.dt;
        }
        if self.right_d {
            self.x += (50.0) * u.dt;
        }
    }
    pub fn on_draw(&mut self, e: &Event, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans(256.0, 256.0);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g); // We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered
        });
    }
    pub fn on_move(&mut self, btn_args: ButtonArgs) {
        match btn_args.state {
            ButtonState::Press => {
                match btn_args.button {
                    Button::Keyboard(Key::Up) => {self.up_d = true;}
                    Button::Keyboard(Key::Down) => {self.down_d = true;}
                    Button::Keyboard(Key::Left) => {self.left_d = true;}
                    Button::Keyboard(Key::Right) => {self.right_d = true;}
                    _ => {}
                }
            }
            ButtonState::Release => {
                match btn_args.button {
                    Button::Keyboard(Key::Up) => {self.up_d = false;}
                    Button::Keyboard(Key::Down) => {self.down_d = false;}
                    Button::Keyboard(Key::Left) => {self.left_d = false;}
                    Button::Keyboard(Key::Right) => {self.right_d = false;}
                    _ => {}
                }
            }
        }
    }
}
