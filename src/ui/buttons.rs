use bevy::prelude::*;

use crate::game::{ExitGame, ResetBoard, RestartGame};

use super::ui::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

#[derive(Component)]
pub enum GameMenuButton {
    ResetBoard,
    RestartGame,
    ExitToMenue,
}

pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &GameMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                GameMenuButton::ResetBoard => commands.trigger(ResetBoard),
                GameMenuButton::RestartGame => commands.trigger(RestartGame),
                GameMenuButton::ExitToMenue => commands.trigger(ExitGame),
            }
        }
    }
}

pub fn rest_button_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands
        .spawn(Node {
            width: Val::Px(200.),
            height: Val::Px(85.),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    GameMenuButton::ResetBoard,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Reset"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

pub fn button_color_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::srgb(0.5, 0.5, 0.5);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
