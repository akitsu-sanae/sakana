
pub struct Enemy {
    pub position: [f64; 2],
    pub speed: f64,
    pub angle: f64,
    pub is_alive: bool,
}

use piston_window::*;

impl Enemy {
    pub fn update(&mut self) {
        self.position[1] += 1.0;

        if self.position[0] < 0.0
            || self.position[0] > 640.0
            || self.position[1] < 0.0
            || self.position[1] > 480.0 {
                self.is_alive = false;
            }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([1.0, 0.5, 0.2, 0.5],
                 [self.position[0] - 16.0, self.position[1] - 16.0,
                32.0, 32.0],
                c.transform, g);
    }
}


