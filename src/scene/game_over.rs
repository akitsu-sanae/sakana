/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use piston_window::*;
use keyboard_::*;
use scene::*;
use resource::Resource;
use super::title::*;

pub struct GameOver {
    keyboard: Keyboard,
}

impl Scene for GameOver {
    fn new() -> Box<GameOver> {
        box GameOver {
            keyboard: Keyboard::new(),
        }
    }

    fn update(&mut self, e: &Event) -> Option<Box<Scene>> {
        self.keyboard.update(e);

        if self.keyboard.is_button1 {
            Some(Title::new())
        } else {
            None
        }
    }

    fn draw(&self, c: &Context, g: &mut G2d, resource: &mut Resource) {
        text::Text::new_color([1.0, 0.0, 0.0, 1.0], 32)
            .draw("Game Over", &mut resource.glyphs, &c.draw_state,
                  c.transform.trans(100.0, 100.0), g);
    }
}

