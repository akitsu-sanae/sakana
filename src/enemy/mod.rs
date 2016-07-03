/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub mod company;
pub mod kfa;
pub mod ogm;

use piston_window::*;
use bullet_company::*;

pub trait Enemy {
    fn new(pos: [f64; 2]) -> Box<Self> where Self: Sized;
    fn update(&mut self, bullets: &mut BulletCompany);
    fn is_alive(&self) -> bool;
    fn draw(&self, c: &Context, g: &mut G2d);
}

