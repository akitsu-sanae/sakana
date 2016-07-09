/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use enemy::*;
use piston_window::*;
use bullet_company::*;


// small enemy
pub struct Kfa {
    pub position: [f64; 2],
    counter: i32,
}
impl Enemy for Kfa {
    fn new(pos: [f64; 2]) -> Box<Kfa> {
        box Kfa {
            position: pos,
            counter: 0,
        }
    }

    fn update(&mut self, bullets: &mut BulletCompany) {
        self.position[1] += 1.0;
        self.counter += 1;

        if self.counter >= 60 {
            bullets.add(self.position);
            self.counter -= 60;
        }
    }

    fn draw(&self, c: &Context, g: &mut G2d) {
        rectangle([1.0, 0.5, 0.2, 0.5],
                 [self.position[0] - 16.0, self.position[1] - 16.0,
                32.0, 32.0],
                c.transform, g);
    }

    fn position(&self) -> [f64; 2] {
        return self.position;
    }
}


