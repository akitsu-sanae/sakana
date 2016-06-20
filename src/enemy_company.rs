
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
            self.enemies.push(Enemy {
                position: [32.0, 32.0],
                speed: 0.0,
                angle: 0.0,
                is_alive: true,
            });
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        for ref e in &self.enemies {
            (*e).draw(c, g);
        }
    }
}


