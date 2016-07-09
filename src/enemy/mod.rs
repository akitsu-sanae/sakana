/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub mod company;
pub mod kfa;
pub mod ogm;
pub mod pfm;

use piston_window::*;
use bullet_company::*;

pub trait Enemy {
    fn new(pos: [f64; 2]) -> Box<Self> where Self: Sized;
    fn update(&mut self, bullets: &mut BulletCompany);
    fn draw(&self, c: &Context, g: &mut G2d);

    // getter
    fn position(&self) -> [f64; 2];

    fn is_alive(&self) -> bool {
        if self.position()[0] < -32.0 {
            return false;
        }
        if self.position()[0] > 640.0 + 32.0 {
            return false;
        }
        if self.position()[1] < -32.0 {
            return false;
        }
        if self.position()[1] > 480.0 + 32.0 {
            return false;
        }
        return true;
    }
}

