use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use rand::distr::{Distribution, Uniform};
use rand_chacha::ChaCha8Rng;

use crate::utils::random::RandomSource;

use super::field::{FieldData, FieldStatus};

#[derive(Resource)]
pub struct Board {
    pub fields: HashMap<UVec2, FieldData>,
    pub size: UVec2,
    pub bomb_count: u32,
    pub generated: bool,
}

impl Board {
    pub fn new(size: UVec2, bomb_count: u32) -> Self {
        let fields = HashMap::new();

        Self {
            fields,
            size,
            bomb_count,
            generated: false,
        }
    }

    pub fn generate(&mut self, mut rng: ResMut<RandomSource<ChaCha8Rng>>, start_field: UVec2) {
        let mut bombs: HashSet<UVec2> = HashSet::new();
        let mut counter = 0;
        self.generated = true;

        let distribution_x = Uniform::new(0, self.size.x).expect("Could not create Distribution");
        let distribution_y = Uniform::new(0, self.size.y).expect("Could not create Distribution");

        while counter < self.bomb_count {
            let pos: UVec2 = (
                distribution_x.sample(&mut rng),
                distribution_y.sample(&mut rng),
            )
                .into();

            if !bombs.contains(&pos) && pos != start_field {
                let Some(field) = self.fields.get_mut(&pos) else {
                    warn!("Could not set bomb for field {}", pos);
                    self.generated = false;
                    return;
                };
                field.bomb = true;
                bombs.insert(pos);
                counter += 1;
            }
        }
    }

    pub fn is_bomb(&self, pos: UVec2) -> bool {
        self.fields.get(&pos).unwrap().bomb
    }

    pub fn bombs(&self, pos: UVec2) -> u32 {
        let mut res = 0;
        for pos in self.neighbors(pos) {
            if self.fields.get(&pos).unwrap().bomb {
                res += 1;
            }
        }
        res
    }

    pub fn flags(&self, pos: UVec2) -> u32 {
        let mut res = 0;
        for pos in self.neighbors(pos) {
            if matches!(self.fields.get(&pos).unwrap().status, FieldStatus::Flaged) {
                res += 1;
            }
        }
        res
    }

    pub fn fullfilled(&self, pos: UVec2) -> bool {
        let mut bombs = 0;
        let mut flags = 0;

        for pos in self.neighbors(pos) {
            if matches!(self.fields.get(&pos).unwrap().status, FieldStatus::Flaged) {
                flags += 1;
            }
            if self.fields.get(&pos).unwrap().bomb {
                bombs += 1;
            }
        }

        bombs == flags
    }

    pub fn neighbors(&self, pos: UVec2) -> Vec<UVec2> {
        let mut res = Vec::new();
        for w in -1..=1 {
            if pos.x as i32 + w < 0 || pos.x as i32 + w >= self.size.x as i32 {
                continue;
            }
            for h in -1..=1 {
                if pos.y as i32 + h < 0 || pos.y as i32 + h >= self.size.y as i32 {
                    continue;
                }

                let x = (pos.x as i32 + w) as u32;
                let y = (pos.y as i32 + h) as u32;

                res.push(UVec2 { x, y });
            }
        }

        res
    }

    pub fn add_field(&mut self, pos: UVec2, id: Entity) {
        self.fields.insert(
            pos,
            FieldData {
                entity: id,
                ..default()
            },
        );
    }
}
