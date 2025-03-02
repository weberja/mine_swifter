use bevy::prelude::*;
use buttons::{
    button_color_system, game_buttons::game_button_interaction, menu_buttons::menu_interaction,
};

use crate::states::AppState;

pub mod buttons;
pub mod cursor;

pub fn buttons(app: &mut App) {
    app.add_systems(
        Update,
        (
            menu_interaction,
            game_button_interaction,
            button_color_system,
        ),
    )
    .add_systems(OnExit(AppState::LoadingAssets), cursor::custom_cursor);
}
