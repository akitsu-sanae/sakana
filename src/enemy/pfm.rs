/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use enemy::*;
use piston_window::*;
use bullet_company::*;

use std::f64::consts::PI;


// big enemy
pub struct Pfm {
    pub position: [f64; 2],
    counter: i32,
}
impl Enemy for Pfm {
    fn new(pos: [f64; 2]) -> Box<Pfm> {
        box Pfm {
            position: pos,
            counter: 0,
        }
    }

    fn update(&mut self, bullets: &mut BulletCompany) {
        self.position[1] += 0.5;
        self.counter += 1;

        if self.counter%5 == 0 {
            let angle = 2.0 * PI * (self.counter as f64) / 360.0;
            bullets.add(self.position, angle);
        }

        if self.counter > 360 {
            self.counter = 0;
        }
    }

    fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([0.4, 0.4, 0.4, 0.5],
                 [self.position[0] - 32.0, self.position[1] - 32.0,
                64.0, 64.0],
                c.transform, g);
    }

    fn position(&self) -> [f64; 2] {
        self.position
    }
}


