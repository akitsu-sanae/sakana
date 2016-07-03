/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use piston_window::*;
use keyboard_::*;
use scene::*;
use super::game::*;

pub struct Title {
    keyboard: Keyboard,
}

impl Scene for Title {

    fn new() -> Box<Title> {
        box Title {
            keyboard: Keyboard::new(),
        }
    }

    fn update(&mut self, e: &Event) -> Option<Box<Scene>> {
        self.keyboard.update(e);

        if self.keyboard.is_button1 {
            Some(Game::new())
        } else {
            None
        }
    }

    fn draw(&self, c: &Context, g: &mut G2d) {
    }
}

