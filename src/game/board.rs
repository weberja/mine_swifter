use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;
use rand_core::{RngCore, SeedableRng};

use crate::helper::NDim;

use super::OpenNeighbors;

#[derive(Component)]
pub struct Field {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Default)]
pub enum FieldStatus {
    #[default]
    Closed,
    Open,
    Flaged,
}

#[derive(Clone, Copy)]
pub struct FieldData {
    pub entity: Entity,
    pub status: FieldStatus,
}

impl Default for FieldData {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            status: FieldStatus::default(),
        }
    }
}

#[derive(Resource)]
pub struct Board {
    pub fields: NDim<FieldData>,
    pub flags: NDim<bool>,
    pub bombs: NDim<bool>,
    pub generated: bool,
}

impl Board {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            fields: NDim::new(UVec2 { x, y }, FieldData::default()),
            flags: NDim::new(UVec2 { x, y }, false),
            bombs: NDim::new(UVec2 { x, y }, false),
            generated: false,
        }
    }

    pub fn generate_layout(&mut self, first_field: UVec2, settings: &BoardSettings) {
        self.generated = true;

        if !settings.solvable {
            let res = Self::random_board(
                first_field,
                settings.size.x,
                settings.size.y,
                settings.bombs,
            );

            self.bombs = res;
            return;
        }
    }

    pub fn random_board(first_field: UVec2, w: u32, h: u32, bomb_count: u32) -> NDim<bool> {
        let mut rng = Entropy::<WyRand>::new(WyRand::seed_from_u64(1234));

        let mut bombs = NDim::new(UVec2 { x: w, y: h }, false);

        for _ in 0..bomb_count {
            Self::set_bomb(&mut bombs, first_field, w, h, &mut rng);
        }

        bombs
    }

    pub fn neighbour_pos(&self, pos: UVec2) -> Vec<UVec2> {
        let mut res = Vec::new();
        for w in -1..=1 {
            if pos.x as i32 + w < 0 || pos.x as i32 + w >= self.bombs.dims().x as i32 {
                continue;
            }
            for h in -1..=1 {
                if pos.y as i32 + h < 0 || pos.y as i32 + h >= self.bombs.dims().y as i32 {
                    continue;
                }

                let x = (pos.x as i32 + w) as u32;
                let y = (pos.y as i32 + h) as u32;

                res.push(UVec2 { x, y });
            }
        }

        res
    }

    pub fn neighbour_bombs(&self, pos: UVec2) -> u32 {
        let mut res = 0;

        for n_pos in self.neighbour_pos(pos) {
            if self
                .bombs
                .get(n_pos.x as usize, n_pos.y as usize)
                .is_some_and(|&val| val == true)
            {
                res += 1;
            }
        }

        res
    }

    pub fn neighbour_flags(&self, pos: UVec2) -> u32 {
        let mut res = 0;

        for n_pos in self.neighbour_pos(pos) {
            if self
                .flags
                .get(n_pos.x as usize, n_pos.y as usize)
                .is_some_and(|&val| val == true)
            {
                res += 1;
            }
        }

        res
    }

    pub fn open_neighbors(
        trigger: Trigger<OpenNeighbors>,
        mut sprites: Query<&mut Sprite, With<Field>>,
        mut board: ResMut<Board>,
        settings: Res<BoardSettings>,
        mut commands: Commands,
    ) {
        let ev = trigger.event();
        let pos = ev.pos;

        let Some(field_data) = board.fields.get(pos.x as usize, pos.y as usize) else {
            return;
        };

        if !matches!(field_data.status, FieldStatus::Closed) {
            return;
        }

        let Ok(mut sprite) = sprites.get_mut(field_data.entity) else {
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

        let Some(&is_bomb) = board.bombs.get(pos.x as usize, pos.y as usize) else {
            warn!("Could not get board data");
            return;
        };

        let Some(field_data) = board.fields.get_mut(pos.x as usize, pos.y as usize) else {
            return;
        };

        if is_bomb {
            info!("Set bomb at {},{}", pos.x, pos.y);
            texture_atlas.index = 2;
        } else {
            field_data.status = FieldStatus::Open;

            let neighbours = board.neighbour_bombs(UVec2 { x: pos.x, y: pos.y });
            info!("Set emtpy with {neighbours} at {},{}", pos.x, pos.y);

            match neighbours {
                1 => texture_atlas.index = 8,
                2 => texture_atlas.index = 7,
                3 => texture_atlas.index = 6,
                4 => texture_atlas.index = 5,
                5 => texture_atlas.index = 4,
                _ => texture_atlas.index = 3,
            }

            let surrounding_bombs = board.neighbour_bombs(pos);
            let surrounding_flags = board.neighbour_flags(pos);

            if surrounding_bombs == surrounding_flags {
                let surrounding_pos = board.neighbour_pos(pos);
                for n_pos in surrounding_pos {
                    commands.trigger(OpenNeighbors { pos: n_pos });
                }
            }
        }
    }

    fn set_bomb(
        bombs: &mut NDim<bool>,
        first_field: UVec2,
        w: u32,
        h: u32,
        rng: &mut Entropy<WyRand>,
    ) {
        let random = rng.next_u32() % (w * h);
        let x = random / w;
        let y = random % h;

        if bombs
            .get(x as usize, y as usize)
            .is_some_and(|&val| val == false)
            && first_field.x != x
            && first_field.y != y
        {
            info!("Set bomb at {x}, {y}");
            bombs.set(x as usize, y as usize, true);
        } else {
            Self::set_bomb(bombs, first_field, w, h, rng);
        }
    }
}

#[derive(Component)]
struct BoardSolver;

impl BoardSolver {
    fn solve(board: Board) -> Result {
        let solvable = false;
        Result { solvable }
    }
}

pub struct Result {
    pub solvable: bool,
}

#[derive(Resource, Copy, Clone)]
pub struct BoardSettings {
    pub size: UVec3,
    pub bombs: u32,
    pub solvable: bool,
}

#[derive(Resource, Clone)]
pub struct BoardAssets {
    pub field_texture: Handle<Image>,
    pub atlas_unpressed: TextureAtlas,
    pub default_scale: Vec3,
}
