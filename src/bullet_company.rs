/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use piston_window::*;
use bullet::*;

pub struct BulletCompany {
    pub bullets: Vec<Bullet>,
}

impl BulletCompany {
    pub fn new() -> BulletCompany {
        BulletCompany {
            bullets: vec![],
        }
    }

    pub fn add(&mut self, pos: [f64; 2], angle: f64) {
        self.bullets.push(Bullet::new(pos, angle));
    }
    pub fn update(&mut self) {
        self.bullets.retain(|ref e| (*e).is_alive());
        for ref mut e in &mut self.bullets {
            (*e).update();
        }
    }
    pub fn draw(&self, c: &Context, g: &mut G2d) {
        for ref e in &self.bullets {
            (*e).draw(c, g);
        }
    }
}

