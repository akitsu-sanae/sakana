/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub struct Shot {
    pub position: [f64; 2],
}

use piston_window::*;

impl Shot {

    pub fn new(pos: [f64; 2]) -> Shot {
        Shot {
            position: pos,
        }
    }

    pub fn update(&mut self) {
        self.position[1] -= 3.0;
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([0.0, 0.7, 0.4, 0.1],
                  [self.position[0] - 2.0, self.position[1] - 2.0,
                  4.0, 12.0],
                  c.transform, g);
    }

    pub fn is_alive(&self) -> bool {
        if self.position[0] < -32.0 || self.position[0] > 640.0 + 32.0 {
            false
        } else if self.position[1] < -32.0 || self.position[1] > 480.0 + 32.0 {
            false
        } else {
            true
        }
    }
}

