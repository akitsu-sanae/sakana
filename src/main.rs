/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(box_syntax)]

extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod keyboard_;
mod charactor;
mod player;
mod enemy;
mod bullet;
mod bullet_company;
mod shot;
mod scene;
mod resource;

use scene::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("sakana", [640, 480])
        .exit_on_esc(true)
        .build().expect("fail to initialize window");

    let mut scene: Box<Scene> = scene::title::Title::new();
    let mut resource = resource::Resource::new(&window);

    while let Some(e) = window.next() {
        let next_scene = scene.update(&e);
        if let Some(s) = next_scene {
            scene = s;
        }

        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            scene.draw(&c, g, &mut resource);
        });
    }
}

