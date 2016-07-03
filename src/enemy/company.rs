/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/sakana
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use piston_window::*;
use enemy::*;
use super::kfa::*;
use super::ogm::*;
use bullet_company::*;

pub enum EnemyType {
    Kfa,
    Ogm
}

pub struct EnemyEmergeData {
    pub counter: i32,
    pub enemy_type: EnemyType,
    pub position: [f64; 2],
}

pub struct EnemyCompany {
    pub enemies: Vec<Box<Enemy>>,
    pub emerge_data: Vec<EnemyEmergeData>,
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

impl EnemyCompany {
    pub fn load(filename: &str) -> EnemyCompany {
        let path = Path::new(filename);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("can not open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("can not read {}: {}", display, why.description()),
            Ok(_) => EnemyCompany {
                enemies: vec![],
                emerge_data: EnemyCompany::read_emerge_data(&s),
            }
        }
    }

    pub fn update(&mut self, bullet_company: &mut BulletCompany) {
        self.enemies.retain(|ref e| (*e).is_alive());
        for ref mut e in &mut self.enemies {
            (*e).update(bullet_company);
        }

        if self.emerge_data.is_empty() {
            return;
        }

        if self.emerge_data[0].counter == 0 {
            self.enemies.push(match self.emerge_data[0].enemy_type {
                EnemyType::Kfa => Kfa::new(self.emerge_data[0].position),
                EnemyType::Ogm => Ogm::new(self.emerge_data[0].position),
            });
            self.emerge_data.remove(0);
        } else {
            self.emerge_data[0].counter -= 1;
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        for ref e in &self.enemies {
            (*e).draw(c, g);
        }
    }


    fn read_emerge_data(data: &String) -> Vec<EnemyEmergeData> {
        let mut result = vec![];
        for line in data.split("\n").collect::<Vec<_>>() {
            let infos = line.split(" ").collect::<Vec<_>>();
            if infos.len() != 4 {
                break;
            }
            result.push(EnemyEmergeData {
                counter: infos[0].parse().unwrap(),
                enemy_type: match infos[1] {
                    "Kfa" => EnemyType::Kfa,
                    "Ogm" => EnemyType::Ogm,
                    _ => panic!("invalid enemy type: {}", infos[1]),
                },
                position: [infos[2].parse::<f64>().unwrap(), infos[3].parse::<f64>().unwrap()],
            });
        }
        result
    }
}


