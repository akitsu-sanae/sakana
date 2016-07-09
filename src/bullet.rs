/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub struct Bullet {
    pub position: [f64;2],
    pub angle: f64,
    pub is_alive: bool,
}

use piston_window::*;
use std::f64;

impl Bullet {
    pub fn new(pos: [f64; 2], angle: f64) -> Bullet {
        Bullet {
            position: pos,
            angle: angle,
            is_alive: true,
        }
    }

    pub fn update(&mut self) {
        self.position[0] += 2.0 * self.angle.cos();
        self.position[1] += 2.0 * self.angle.sin();

        if self.position[0] < 0.0
            || self.position[0] > 640.0
            || self.position[1] < 0.0
            || self.position[1] > 480.0 {
                self.is_alive = false;
            }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([1.0, 0.2, 0.2, 0.7],
                  [self.position[0] - 4.0, self.position[1] - 8.0,
                  8.0, 16.0],
                  c.transform, g);
    }
}

