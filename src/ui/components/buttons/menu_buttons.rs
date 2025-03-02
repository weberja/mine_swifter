#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::{board::BoardSettings, game::events::CreateBoard, states::AppState};

#[derive(Component)]
pub enum MenuButton {
    S9x9,
    S16x16,
    S30x16,
    Challenge,
}

pub fn menu_interaction(
    mut interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButton::S9x9 => {
                    next_state.set(AppState::Game);
                    commands.insert_resource(BoardSettings {
                        size: (9, 9).into(),
                        solvable: false,
                        bomb_count: 10,
                    });
                }
                MenuButton::S16x16 => {
                    next_state.set(AppState::Game);
                    commands.insert_resource(BoardSettings {
                        size: (16, 16).into(),
                        solvable: false,
                        bomb_count: 40,
                    });
                }
                MenuButton::S30x16 => {
                    next_state.set(AppState::Game);
                    commands.insert_resource(BoardSettings {
                        size: (30, 16).into(),
                        solvable: false,
                        bomb_count: 99,
                    });
                }
                MenuButton::Challenge => todo!(),
            }
        }
    }
}
