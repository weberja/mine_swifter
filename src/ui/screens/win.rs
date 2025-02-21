use bevy::prelude::*;

use crate::{
    assets::FontAssets,
    states::GameState,
    ui::components::buttons::{game_buttons::GameButton, NORMAL_BUTTON},
};

pub fn win_screen(app: &mut App) {
    app.add_systems(OnEnter(GameState::Won), setup_win_screen)
        .add_systems(OnExit(GameState::Won), destroy_win_screen);
}

#[derive(Component)]
pub struct WinScreenComp;

pub fn destroy_win_screen(mut commands: Commands, q_comp: Query<Entity, With<WinScreenComp>>) {
    for e in &q_comp {
        commands.entity(e).despawn_recursive();
    }
}

pub fn setup_win_screen(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..default()
            },
            WinScreenComp,
        ))
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                border: UiRect::all(Val::Percent(5.)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.65, 0.65, 0.65, 0.9)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Gewonnen"),
                                TextFont {
                                    font: fonts.fira_code.clone(),
                                    font_size: 100.0,
                                    ..default()
                                },
                                Label,
                            ));

                            parent
                                .spawn((
                                    Button,
                                    GameButton::RestartGame,
                                    Node {
                                        width: Val::Px(200.0),
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
                                    Text::new("Neues Spiel"),
                                    TextFont {
                                        font: fonts.fira_code.clone(),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));

                            parent
                                .spawn((
                                    Button,
                                    GameButton::BackToMenu,
                                    Node {
                                        width: Val::Px(200.0),
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
                                    Text::new("Zum Menu"),
                                    TextFont {
                                        font: fonts.fira_code.clone(),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));
                        });
                });
        });
}
