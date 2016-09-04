/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use piston_window::*;
use find_folder::Search;

pub struct Resource {
    pub glyphs: Glyphs,
}

impl Resource {
    pub fn new(window: &PistonWindow) -> Resource {
        let resource = Search::ParentsThenKids(3, 3)
            .for_folder("resource")
            .expect("resource folder not found");

        let ref font = resource.join("PixelMPlus10-Regular.ttf");
        let glyph = Glyphs::new(font, window.factory.clone())
            .expect("font error");

        Resource {
            glyphs: glyph,
        }
    }
}

