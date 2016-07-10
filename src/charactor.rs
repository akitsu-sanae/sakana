/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use piston_window::*;

pub trait Charactor {
    fn update(&mut self);
    fn draw(&self, c: &Context, g: &mut G2d);


    fn position(&self) -> [f64; 2];
    fn is_alive(&self) -> bool {
        if self.position()[0] < -32.0 || self.position()[0] > 640.0 + 32.0 {
            false
        } else if self.position()[1] < -32.0 || self.position()[1] > 480.0 + 32.0 {
            false
        } else {
            true
        }
    }

    fn on_collided(&mut self, other: &Box<Charactor>);
    fn is_collided(&self, other: &Box<Charactor>) -> bool;
}

