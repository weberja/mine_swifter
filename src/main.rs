pub mod assets;
pub mod states;
pub mod ui;

use assets::startup_assets;
use bevy::prelude::*;
use states::states;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, states, startup_assets, ui::init))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
