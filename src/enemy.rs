/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub struct Enemy {
    pub position: [f64; 2],
    pub is_alive: bool,
}

use piston_window::*;

impl Enemy {
    pub fn new(pos: [f64; 2]) -> Enemy {
        Enemy {
            position: pos,
            is_alive: true,
        }
    }
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


