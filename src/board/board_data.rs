use bevy::{prelude::*, utils::HashMap};

use super::field::FieldData;

#[derive(Resource)]
pub struct Board {
    pub fields: HashMap<UVec2, FieldData>,
    pub size: UVec2,
    pub bomb_count: u32,
    pub generated: bool,
}

impl Board {
    pub fn new(size: UVec2, bomb_count: u32) -> Self {
        let mut fields = HashMap::new();
        for x in 0..size.x {
            for y in 0..size.y {
                fields.insert((x, y).into(), FieldData::default());
            }
        }

        Self {
            fields,
            size,
            bomb_count,
            generated: false,
        }
    }
}
