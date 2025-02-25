pub mod assets;
pub mod board;
pub mod game;
pub mod materials;
pub mod states;
pub mod ui;
pub mod utils;

use assets::startup_assets;
use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;
use states::states;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use utils::random::RandomSource;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            states,
            startup_assets,
            ui::init,
            game::game,
            materials::init,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .insert_resource(RandomSource::<ChaCha8Rng>::default())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
