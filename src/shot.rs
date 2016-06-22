/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub struct Shot {
    pub position: [f64; 2],
    pub is_alive: bool,
}

use piston_window::*;

impl Shot {

    pub fn new(pos: [f64; 2]) -> Shot {
        Shot {
            position: pos,
            is_alive: true,
        }
    }

    pub fn update(&mut self) {
        self.position[1] -= 3.0;

        if self.position[0] < 0.0
            || self.position[0] > 640.0
            || self.position[1] < 0.0
            || self.position[1] > 480.0 {
                self.is_alive = false;
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([0.0, 0.7, 0.4, 0.1],
                  [self.position[0] - 2.0, self.position[1] - 2.0,
                  4.0, 12.0],
                  c.transform, g);
    }
}

