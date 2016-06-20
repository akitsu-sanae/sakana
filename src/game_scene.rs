
use shot::*;
use enemy::*;
use keyboard_::*;
use player::*;
use piston_window::*;

pub struct GameScene {
    shots: Vec<Shot>,
    enemies: Vec<Enemy>,
    player: Player,

    keyboard: Keyboard,
    counter: u32,
}

impl GameScene {

    pub fn new() -> GameScene {
        GameScene {
            shots: vec![],
            enemies: vec![],
            player: Player {
                name: "akitsu-sanae".to_string(),
                position: [300.0, 400.0],
            },
            keyboard: Keyboard {
                is_left: false,
                is_right: false,
                is_up: false,
                is_down: false,
                is_button1: false,
                is_button2: false,
            },
            counter: 0,
        }
    }

    pub fn update(&mut self, e: &Event) {
        self.keyboard.update(e);
        self.player.update(&self.keyboard);

        self.enemies.retain(|ref e| (*e).is_alive);
        self.shots.retain(|ref s| (*s).is_alive);

        if self.keyboard.is_button1 && self.counter%6 == 0 {
            self.shots.push(Shot{
                position: self.player.position.clone(),
                speed: 0.0,
                angle: 0.0,
                is_alive: true,
            });
        }

        if self.keyboard.is_button2 {
            self.enemies.push(Enemy {
                position: [32.0, 32.0],
                speed: 0.0,
                angle: 0.0,
                is_alive: true,
            });
        }

        for ref mut e in &mut self.enemies {
            (*e).update();
        }

        for ref mut s in &mut self.shots {
            (*s).update();
        }

        self.counter += 1;
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        self.player.draw(c, g);
        for ref s in &self.shots {
            (*s).draw(c, g);
        }
        for ref e in &self.enemies {
            (*e).draw(c, g);
        }
    }
}


