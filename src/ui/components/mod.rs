use bevy::prelude::*;

use crate::states::AppState;

pub mod buttons;
pub mod cursor;

pub fn buttons(app: &mut App) {
    app.add_systems(OnExit(AppState::LoadingAssets), cursor::custom_cursor);
}
