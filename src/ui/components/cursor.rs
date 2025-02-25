use bevy::{
    prelude::*,
    winit::cursor::{CursorIcon, CustomCursor},
};

use crate::assets::UiAssets;

pub fn custom_cursor(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    assets: Res<UiAssets>,
) {
    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image {
            handle: assets.cursor.clone(),
            hotspot: (5, 5),
        }));
}
