use piston_window::*;

pub struct Game {
    rotation: f64, x: f64, y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool,
    rotationb: f64, xb: f64, yb: f64,
    up_db: bool, down_db: bool, left_db: bool, right_db: bool
}

impl Game {
    pub fn new() -> Game {
        Game {
            rotation : 0.0, x : 0.0, y : 0.0,
            up_d: false, down_d: false, left_d: false, right_d: false,
            rotationb : 0.0, xb : 0.0, yb : 0.0,
            up_db: false, down_db: false, left_db: false, right_db: false
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
        self.rotationb += 3.0 * u.dt;
        if self.up_db {
            self.yb += (-50.0) * u.dt;
        }
        if self.down_db {
            self.yb += (50.0) * u.dt;
        }
        if self.left_db {
            self.xb += (-50.0) * u.dt;
        }
        if self.right_db {
            self.xb += (50.0) * u.dt;
        }
    }
    pub fn on_draw(&mut self, e: &Event, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans(256.0, 256.0);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            let blue = [0.0, 0.0, 1.0, 1.0];
            rectangle(red, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g); // We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered
            rectangle(blue, square, center.trans(self.xb, self.yb).rot_rad(self.rotation).trans(-50.0, -50.0), g); // We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered
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
                    Button::Keyboard(Key::W) => {self.up_db = true;}
                    Button::Keyboard(Key::S) => {self.down_db = true;}
                    Button::Keyboard(Key::A) => {self.left_db = true;}
                    Button::Keyboard(Key::D) => {self.right_db = true;}
                    _ => {}
                }
            }
            ButtonState::Release => {
                match btn_args.button {
                    Button::Keyboard(Key::Up) => {self.up_d = false;}
                    Button::Keyboard(Key::Down) => {self.down_d = false;}
                    Button::Keyboard(Key::Left) => {self.left_d = false;}
                    Button::Keyboard(Key::Right) => {self.right_d = false;}
                    Button::Keyboard(Key::W) => {self.up_db = false;}
                    Button::Keyboard(Key::S) => {self.down_db = false;}
                    Button::Keyboard(Key::A) => {self.left_db = false;}
                    Button::Keyboard(Key::D) => {self.right_db = false;}
                    _ => {}
                }
            }
        }
    }
}
