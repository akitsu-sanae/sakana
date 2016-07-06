/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use shot::*;
use enemy::company::*;
use bullet_company::*;
use keyboard_::*;
use player::*;
use piston_window::*;
use scene::*;

pub struct Game {
    shots: Vec<Shot>,
    enemy_company: EnemyCompany,
    bullet_company: BulletCompany,
    player: Player,

    keyboard: Keyboard,
    counter: u32,
}

impl Scene for Game {

    fn new() -> Box<Game> {
        box Game {
            shots: vec![],
            enemy_company : EnemyCompany::load("resource/enemy_data.dat"),
            bullet_company: BulletCompany::new(),
            player: Player {
                position: [300.0, 400.0],
            },
            keyboard: Keyboard::new(),
            counter: 0,
        }
    }

    fn update(&mut self, e: &Event) -> Option<Box<Scene>> {
        self.keyboard.update(e);
        self.player.update(&self.keyboard);

        self.shots.retain(|ref s| (*s).is_alive);

        if self.keyboard.is_button1 && self.counter%6 == 0 {
            self.shots.push(Shot::new(self.player.position));
        }

        self.enemy_company.update(&mut self.bullet_company);
        self.bullet_company.update();

        for ref mut s in &mut self.shots {
            (*s).update();
        }

        self.counter += 1;
        None
    }

    fn draw(&self, c: &Context, g: &mut G2d) {
        self.player.draw(c, g);
        for ref s in &self.shots {
            (*s).draw(c, g);
        }
        self.enemy_company.draw(c, g);
        self.bullet_company.draw(c, g);
    }
}


