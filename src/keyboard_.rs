/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub struct Keyboard {
    pub is_left: bool,
    pub is_right: bool,
    pub is_up: bool,
    pub is_down: bool,
    pub is_button1: bool,
    pub is_button2: bool,
}

use piston_window::*;

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            is_left: false,
            is_right: false,
            is_up: false,
            is_down: false,
            is_button1: false,
            is_button2: false,
        }
    }
    pub fn update(&mut self, e: &Event) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => self.is_left = true,
                Key::Right => self.is_right = true,
                Key::Up => self.is_up = true,
                Key::Down => self.is_down = true,
                Key::Z => self.is_button1 = true,
                Key::X => self.is_button2 = true,
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Left => self.is_left = false,
                Key::Right => self.is_right = false,
                Key::Up => self.is_up = false,
                Key::Down => self.is_down = false,
                Key::Z => self.is_button1 = false,
                Key::X => self.is_button2 = false,
                _ => {}
            }
        }
    }
}

