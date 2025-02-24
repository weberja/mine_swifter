use bevy::{prelude::*, utils::HashMap};

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

    pub fn generate(&mut self) {}

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
