/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(box_syntax)]

extern crate piston_window;

use piston_window::*;

mod keyboard_;
mod player;
mod enemy;
mod bullet;
mod bullet_company;
mod shot;
mod scene;

use scene::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("sakana", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut events = window.events();
    let mut scene: Box<Scene> = scene::title::Title::new();

    while let Some(e) = events.next(&mut window) {
        let next_scene = scene.update(&e);
        if let Some(s) = next_scene {
            scene = s;
        }

        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            scene.draw(&c, g);
        });
    }
}

