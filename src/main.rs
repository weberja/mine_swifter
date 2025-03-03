pub mod assets;
pub mod board;
mod camera;
pub mod game;
pub mod materials;
pub mod states;
pub mod ui;
pub mod utils;

use assets::startup_assets;
use bevy::{asset::AssetMetaCheck, prelude::*};
use rand_chacha::ChaCha8Rng;
use states::states;

use utils::random::RandomSource;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            states,
            startup_assets,
            ui::init,
            game::game,
            materials::init,
            camera::camera,
        ))
        .insert_resource(RandomSource::<ChaCha8Rng>::default())
        .run();
}
