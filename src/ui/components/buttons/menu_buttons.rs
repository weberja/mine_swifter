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
                    commands.trigger(CreateBoard);
                }
                MenuButton::S16x16 => todo!(),
                MenuButton::S30x16 => todo!(),
                MenuButton::Challenge => todo!(),
            }
        }
    }
}
