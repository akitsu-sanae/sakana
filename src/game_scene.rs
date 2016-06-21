
use shot::*;
use enemy_company::*;
use keyboard_::*;
use player::*;
use piston_window::*;

pub struct GameScene {
    shots: Vec<Shot>,
    enemy_company: EnemyCompany,
    player: Player,

    keyboard: Keyboard,
    counter: u32,
}

impl GameScene {

    pub fn new() -> GameScene {
        GameScene {
            shots: vec![],
            enemy_company : EnemyCompany::load("resource/enemy_data.dat".to_string()).unwrap(),
            player: Player {
                position: [300.0, 400.0],
            },
            keyboard: Keyboard::new(),
            counter: 0,
        }
    }

    pub fn update(&mut self, e: &Event) {
        self.keyboard.update(e);
        self.player.update(&self.keyboard);

        self.shots.retain(|ref s| (*s).is_alive);

        if self.keyboard.is_button1 && self.counter%6 == 0 {
            self.shots.push(Shot::new(self.player.position));
        }

        self.enemy_company.update(&self.keyboard);

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
        self.enemy_company.draw(c, g);
    }
}


