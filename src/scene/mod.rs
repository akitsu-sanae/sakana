/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/
pub mod game;
pub mod title;
pub mod game_over;

use piston_window::*;
use resource::Resource;

pub trait Scene {
    fn new() -> Box<Self> where Self: Sized;

    fn update(&mut self, e: &Event) -> Option<Box<Scene>>;

    fn draw(&self, &Context, g: &mut G2d, resource: &mut Resource);
}

