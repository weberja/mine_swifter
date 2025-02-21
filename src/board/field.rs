use bevy::prelude::*;

#[derive(Component)]
pub struct Field(pub UVec2);

#[derive(Clone, Copy, Default)]
pub enum FieldStatus {
    #[default]
    Closed,
    Open,
    Flaged,
}

#[derive(Resource)]
pub struct FieldData {
    pub status: FieldStatus,
    pub bomb: bool,
}

impl Default for FieldData {
    fn default() -> Self {
        Self {
            bomb: false,
            ..default()
        }
    }
}
