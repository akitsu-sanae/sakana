/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

extern crate piston_window;

use piston_window::*;

mod keyboard_;
mod player;
mod enemy;
mod enemy_company;
mod shot;
mod game_scene;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("sakana", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut events = window.events();
    let mut scene = game_scene::GameScene::new();

    while let Some(e) = events.next(&mut window) {
        scene.update(&e);

        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            scene.draw(&c, g);
        });
    }
}

