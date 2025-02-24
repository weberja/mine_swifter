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
    pub entity: Entity,
    pub status: FieldStatus,
    pub bomb: bool,
}

impl Default for FieldData {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            status: Default::default(),
            bomb: Default::default(),
        }
    }
}
