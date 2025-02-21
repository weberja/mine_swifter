use bevy::prelude::*;
pub mod components;
pub mod screens;

pub fn init(app: &mut App) {
    app.add_plugins((components::buttons, screens::screens));
}
