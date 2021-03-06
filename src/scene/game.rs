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
use super::game_over::*;
use resource::Resource;
use charactor::*;

pub struct Game {
    shots: Vec<Box<Shot>>,
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
                hp: 3,
            },
            keyboard: Keyboard::new(),
            counter: 0,
        }
    }

    fn update(&mut self, e: &Event) -> Option<Box<Scene>> {
        self.keyboard.update(e);
        self.player.update(&self.keyboard);

        self.shots.retain(|s| (*s).is_alive());

        if self.keyboard.is_button1 && self.counter%6 == 0 {
            use std::f64::consts::PI;

            self.shots.push(Shot::new(self.player.position, -PI/3.0));
            self.shots.push(Shot::new(self.player.position, -PI/2.0));
            self.shots.push(Shot::new(self.player.position, -PI*2.0/3.0));
        }

        self.enemy_company.update(&mut self.bullet_company);
        self.bullet_company.update(&mut self.player);

        for ref mut s in &mut self.shots {
            (*s).update();
        }

        self.counter += 1;

        if self.player.hp < 0 {
            Some(GameOver::new())
        } else {
            None
        }
    }

    fn draw(&self, c: &Context, g: &mut G2d, resource: &mut Resource) {
        self.player.draw(c, g);
        for ref s in &self.shots {
            (*s).draw(c, g);
        }
        self.enemy_company.draw(c, g);
        self.bullet_company.draw(c, g);

        text::Text::new_color([1.0, 0.0, 0.0, 1.0], 32)
            .draw(format!("HP: {}", self.player.hp).as_str(),
                  &mut resource.glyphs, &c.draw_state,
                  c.transform.trans(16.0, 32.0), g);
    }
}


