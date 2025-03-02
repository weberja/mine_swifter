#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::board::events::RestartGame;

#[derive(Component)]
pub enum GameButton {
    ResetGame,
    RestartGame,
    UndoLastStep,
    BackToMenu,
}

pub fn game_button_interaction(
    mut interaction_query: Query<(&Interaction, &GameButton), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                GameButton::ResetGame => todo!(),
                GameButton::RestartGame => {
                    info!("New Game");
                    commands.trigger(RestartGame);
                }
                GameButton::UndoLastStep => todo!(),
                GameButton::BackToMenu => todo!(),
            }
        }
    }
}
