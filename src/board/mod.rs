use bevy::{ecs::system::Resource, math::UVec2};

pub mod board_data;
pub mod board_solver;
mod events;
pub mod field;

#[derive(Resource)]
pub struct BoardSettings {
    pub size: UVec2,
    pub solvable: bool,
    pub bomb_count: u32,
}
