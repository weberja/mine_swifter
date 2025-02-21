pub mod assets;
pub mod board;
pub mod game;
pub mod materials;
pub mod states;
pub mod ui;

use assets::startup_assets;
use bevy::prelude::*;
use states::states;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, states, startup_assets, ui::init, game::game))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
