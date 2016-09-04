/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub struct Shot {
    pub position: [f64; 2],
    pub angle: f64,
}

use charactor::*;
use piston_window::*;
use std::f64;

impl Shot {
    pub fn new(pos: [f64; 2], angle: f64) -> Box<Shot> {
        box Shot {
            position: pos,
            angle: angle,
        }
    }
}

impl Charactor for Shot {

    fn update(&mut self) {
        self.position[0] += 2.0 * self.angle.cos();
        self.position[1] += 2.0 * self.angle.sin();
    }

    fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([0.0, 0.7, 0.4, 0.1],
                  [self.position[0] - 2.0, self.position[1] - 2.0,
                  4.0, 12.0],
                  c.transform, g);
    }

    fn position(&self) -> [f64; 2] { self.position }

    fn on_collided(&mut self, _other: &Box<Charactor>) {}
    fn is_collided(&self, _other: &Box<Charactor>) -> bool { false }
}

