use bevy::prelude::*;
pub mod components;
pub mod screens;

pub fn init(app: &mut App) {
    app.add_plugins((components::buttons, screens::screens))
        .add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
        .add_systems(Update, toggle_overlay);
}

fn toggle_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<bevy::dev_tools::ui_debug_overlay::UiDebugOptions>,
) {
    info_once!("The debug outlines are enabled, press Space to turn them on/off");
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug_overlay if disabled and disable if enabled
        options.toggle();
    }
}
