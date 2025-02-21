use bevy::prelude::*;

pub mod background;
pub mod field;

pub fn init(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
        .add_plugins(Material2dPlugin::<FieldMaterial>::default());
}
