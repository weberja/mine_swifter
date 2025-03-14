pub mod assets;
pub mod board;
mod camera;
pub mod game;
mod key_controls;
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
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "MineSwifter".into(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            states,
            startup_assets,
            ui::init,
            game::game,
            materials::init,
            camera::camera,
            key_controls::init,
        ))
        .insert_resource(RandomSource::<ChaCha8Rng>::default())
        .add_systems(Startup, fit_to_screen)
        .run();
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[allow(dead_code)]
fn fit_to_screen(mut bevy_window: Single<&mut Window>) {
    debug!("No auto fullscreen in non wasm envitoments");
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn fit_to_screen(mut bevy_window: Single<&mut Window>) {
    use web_sys::wasm_bindgen::UnwrapThrowExt;
    let screen = web_sys::window()
        .unwrap_throw()
        .document()
        .expect("could not get document")
        .document_element()
        .unwrap_throw();
    let width = screen.client_width();
    let height = screen.client_height();
    info!("Screen: {width} x {height}",);

    bevy_window.resolution.set(width as f32, height as f32);
}
