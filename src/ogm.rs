/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use enemy::*;
use piston_window::*;
use bullet_company::*;


// normal enemy
pub struct Ogm {
    pub position: [f64; 2],
    counter: i32,
}

impl Enemy for Ogm {
    fn new(pos: [f64; 2]) -> Box<Ogm> {
        box Ogm {
            position: pos,
            counter: 0,
        }
    }

    fn update(&mut self, bullets: &mut BulletCompany) {
        self.position[1] += 1.0;
        self.counter += 1;

        if self.counter < 60 {
            self.position[0] += 1.0;
        } else if self.counter < 120 {
            self.position[0] -= 1.0;
        } else {
            self.counter = 0;
        }
    }

    fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([1.0, 0.5, 0.2, 0.5],
                 [self.position[0] - 16.0, self.position[1] - 16.0,
                32.0, 32.0],
                c.transform, g);
    }

    fn is_alive(&self) -> bool {
        if self.position[0] < -32.0 {
            return false;
        }
        if self.position[0] > 640.0 + 32.0 {
            return false;
        }
        if self.position[1] < -32.0 {
            return false;
        }
        if self.position[1] > 480.0 + 32.0 {
            return false;
        }
        return true;
    }

}


