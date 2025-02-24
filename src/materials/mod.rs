use background::BackgroundMaterial;
use bevy::{prelude::*, sprite::Material2dPlugin};
use field::FieldMaterial;
use grid::GridMaterial;

pub mod background;
pub mod field;
pub mod grid;

pub fn init(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
        .add_plugins(Material2dPlugin::<FieldMaterial>::default())
        .add_plugins(Material2dPlugin::<GridMaterial>::default());
}
