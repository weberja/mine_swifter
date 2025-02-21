use bevy::prelude::*;
use buttons::{game_buttons::game_button_interaction, menu_buttons::menu_interaction};

use crate::states::AppState;

pub mod buttons;

pub fn buttons(app: &mut App) {
    app.add_systems(
        Update,
        menu_interaction.run_if(in_state(AppState::MainMenu)),
    )
    .add_systems(
        Update,
        game_button_interaction.run_if(in_state(AppState::Game)),
    );
}
