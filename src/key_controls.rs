use bevy::{prelude::*, window::WindowMode};

use crate::camera::events::ResetView;

pub fn init(app: &mut App) {
    app.add_systems(Update, key_controls);
}

fn key_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        if let WindowMode::BorderlessFullscreen(_) = window.mode {
            window.mode = WindowMode::Windowed
        } else {
            window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        }
    }

    if keys.just_pressed(KeyCode::KeyR) {
        commands.trigger(ResetView);
    }
}
