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
fn fit_to_screen() {
    debug!("No automatic fit to screen");
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn fit_to_screen(
    mut bevy_window: Single<&mut Window>,
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    use bevy::window::WindowResolution;
    use web_sys::wasm_bindgen::UnwrapThrowExt;

    // It is not the device screen, but the screen to what can be written
    let screen = web_sys::window()
        .unwrap_throw()
        .document()
        .expect("could not get document")
        .document_element()
        .unwrap_throw();

    let width = screen.client_width() as f64;
    let height = screen.client_height() as f64;

    // Is needed as the normel widht is not scaled to the real screen size
    let pixel_ratio = web_sys::window().unwrap().device_pixel_ratio();

    debug!("{} x {} with pixel_ratio {}", width, height, pixel_ratio);

    bevy_window.resolution =
        WindowResolution::new((width * pixel_ratio) as f32, (height * pixel_ratio) as f32);
}
