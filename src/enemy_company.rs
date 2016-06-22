/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use piston_window::*;
use keyboard_::*;
use enemy::*;

pub struct EnemyCompany {
    pub enemies: Vec<Enemy>,
}

use std::option::Option;
use std::fs::File;
use std::io::Read;
use std::path::Path;

impl EnemyCompany {
    pub fn load(filename: String) -> Option<EnemyCompany> {
        Some(EnemyCompany {
            enemies: vec![],
        })
    }
    pub fn update(&mut self, keyboard: &Keyboard) {
        self.enemies.retain(|ref e| (*e).is_alive);
        for ref mut e in &mut self.enemies {
            (*e).update();
        }
        if keyboard.is_button2 {
            self.enemies.push(Enemy::new([32.0, 32.0]));
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        for ref e in &self.enemies {
            (*e).draw(c, g);
        }
    }
}


