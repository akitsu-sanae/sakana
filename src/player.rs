
pub struct Player {
    pub position: [f64; 2],
}
use keyboard_::*;
use piston_window::*;

impl Player {

    pub fn update(&mut self, keyboard: &Keyboard) {
        if keyboard.is_up {
            self.position[1] -= 1.0;
        }
        if keyboard.is_down {
            self.position[1] += 1.0;
        }
        if keyboard.is_left {
            self.position[0] -= 1.0;
        }
        if keyboard.is_right {
            self.position[0] += 1.0;
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([0.0, 0.5, 0.8, 0.5],
                  [self.position[0] - 16.0, self.position[1] - 16.0,
                  32.0 , 32.0],
                  c.transform, g);
    }
}

