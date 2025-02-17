use std::{fmt::Display, u8};

use bevy::{prelude::*, utils::HashMap};
use bevy_rand::prelude::Entropy;
use rand_core::SeedableRng;

use crate::GameState;

use super::{board_generator::BoardGenertor, OpenField, OpenNeighbors};

#[derive(Component)]
pub struct Field {
    pub pos: UVec2,
}

#[derive(Clone, Copy, Default)]
pub enum FieldStatus {
    #[default]
    Closed,
    Open,
    Flaged,
}

impl Display for FieldStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldStatus::Closed => f.write_str("Status: Closed"),
            FieldStatus::Open => f.write_str("Status: Opened"),
            FieldStatus::Flaged => f.write_str("Status: Flagged"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FieldData {
    pub entity: Entity,
    pub status: FieldStatus,
    pub bomb: bool,
}

impl Default for FieldData {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            status: FieldStatus::default(),
            bomb: false,
        }
    }
}

#[derive(Resource)]
pub struct Board {
    pub fields: HashMap<UVec2, FieldData>,
    pub size: UVec2,
    pub bomb_count: u32,
    pub generated: bool,
}

impl Board {
    pub fn new(x: u32, y: u32) -> Self {
        let size = UVec2 { x, y };
        Self {
            fields: HashMap::new(),
            size,
            bomb_count: 0,
            generated: false,
        }
    }

    pub fn generate_layout(&mut self, first_field: UVec2, settings: &BoardSettings) {
        if !settings.solvable {
            let bombs = BoardGenertor::random(
                first_field,
                settings.size,
                settings.bombs,
                &mut Entropy::from_entropy(),
            );

            for pos in bombs.bombs {
                let Some(field_data) = self.fields.get_mut(&pos) else {
                    continue;
                };

                info!("Set Bomb at {}", pos);

                field_data.bomb = true;
            }
        }
        self.size = settings.size;
        self.bomb_count = settings.bombs;
        self.generated = true;
    }

    #[inline]
    pub fn neighbours(&self, pos: UVec2) -> Vec<UVec2> {
        Self::neighbour_pos(pos, self.size)
    }

    #[inline]
    pub fn is_sloved(&self) -> bool {
        let mut coverd_fields = 0;
        for (_, field) in self.fields.iter() {
            if matches!(field.status, FieldStatus::Closed)
                || matches!(field.status, FieldStatus::Flaged)
            {
                coverd_fields += 1;
            }
        }

        return coverd_fields == self.bomb_count;
    }

    #[inline]
    pub fn neighbour_pos(pos: UVec2, size: UVec2) -> Vec<UVec2> {
        let mut res = Vec::new();
        for w in -1..=1 {
            if pos.x as i32 + w < 0 || pos.x as i32 + w >= size.x as i32 {
                continue;
            }
            for h in -1..=1 {
                if pos.y as i32 + h < 0 || pos.y as i32 + h >= size.y as i32 {
                    continue;
                }

                let x = (pos.x as i32 + w) as u32;
                let y = (pos.y as i32 + h) as u32;

                res.push(UVec2 { x, y });
            }
        }

        res
    }

    pub fn neighbour_bombs(&self, pos: UVec2) -> u8 {
        let mut res = 0;

        for n_pos in &self.neighbours(pos) {
            if self.fields.get(n_pos).is_some_and(|&val| val.bomb == true) {
                res += 1;
            }
        }

        res
    }

    pub fn neighbour_flags(&self, pos: UVec2) -> u8 {
        let mut res = 0;

        for n_pos in &self.neighbours(pos) {
            if self
                .fields
                .get(n_pos)
                .is_some_and(|val| matches!(val.status, FieldStatus::Flaged))
            {
                res += 1;
            }
        }

        res
    }

    pub fn open_field(trigger: Trigger<OpenField>, board: Res<Board>, mut commands: Commands) {
        let ev = trigger.event();
        let pos = ev.pos;

        info!("OpenField triggert");

        let Some(field_data) = board.fields.get(&pos) else {
            warn!("Could not get FieldData for the clicked pos");
            return;
        };

        if matches!(field_data.status, FieldStatus::Open) || 0 == board.neighbour_flags(pos) {
            info!("Field is allready open or has 0 neighbours");
            commands.trigger(OpenNeighbors {
                pos,
                open_more: true,
            });
        } else if matches!(field_data.status, FieldStatus::Closed) {
            info!("Field was closed");
            commands.trigger(OpenNeighbors {
                pos,
                open_more: false,
            });
        }
    }

    pub fn open_neighbors(
        trigger: Trigger<OpenNeighbors>,
        mut sprites: Query<&mut Sprite, With<Field>>,
        mut board: ResMut<Board>,
        settings: Res<BoardSettings>,
        mut next_game_state: ResMut<NextState<GameState>>,
        mut commands: Commands,
    ) {
        let ev = trigger.event();
        let pos = ev.pos;

        let Some(field_data) = board.fields.get(&pos) else {
            warn!("Could not get FieldData for the clicked pos");

            return;
        };

        if !matches!(field_data.status, FieldStatus::Closed) && !ev.open_more {
            return;
        }

        let Ok(mut sprite) = sprites.get_mut(field_data.entity) else {
            warn!("Could not find sprite for entity");
            return;
        };

        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            warn!("Could not get texture atlas for fields");
            return;
        };

        if !board.generated {
            info!("Generate Board");
            board.generate_layout(pos, &settings);
        }

        let Some(field_data) = board.fields.get_mut(&pos) else {
            warn!("Can not get mutabale field data for pos");
            return;
        };

        info!("{}", field_data.status);

        if field_data.bomb {
            info!("Set bomb at {},{}", pos.x, pos.y);
            texture_atlas.index = 2;
            next_game_state.set(GameState::Lost);
        } else {
            field_data.status = FieldStatus::Open;

            let neighbours = board.neighbour_bombs(pos);
            info!("Set emtpy with {neighbours} at {},{}", pos.x, pos.y);

            match neighbours {
                1 => texture_atlas.index = 8,
                2 => texture_atlas.index = 7,
                3 => texture_atlas.index = 6,
                4 => texture_atlas.index = 5,
                5 => texture_atlas.index = 4,
                _ => texture_atlas.index = 3,
            }

            if board.is_sloved() {
                next_game_state.set(GameState::Won);
                return;
            }

            // Wenn im Event open_more wahr ist oder diese Feld keine Bomben Nachbarn hat, sollen alle Felder
            // in der Umgebung auch geöffnet werden. Diese sollen sich nur öffnen wenn sie auch
            // keine Bomben als nachbaren haben!

            if ev.open_more || neighbours == 0 {
                if board.neighbour_bombs(pos) == board.neighbour_flags(pos) {
                    for n_pos in board.neighbours(pos) {
                        commands.trigger(OpenNeighbors {
                            pos: n_pos,
                            open_more: false,
                        });
                    }
                }
            }
        }
    }
}

#[derive(Resource, Copy, Clone)]
pub struct BoardSettings {
    pub size: UVec2,
    pub bombs: u32,
    pub solvable: bool,
}

#[derive(Resource, Clone)]
pub struct BoardAssets {
    pub field_texture: Handle<Image>,
    pub atlas_unpressed: TextureAtlas,
    pub default_scale: Vec3,
}
