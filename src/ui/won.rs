use bevy::prelude::*;

use super::{buttons::GameMenuButton, ui::NORMAL_BUTTON};

#[derive(Component)]
pub struct WinScreenComp;

pub fn destroy_win_screen(mut commands: Commands, q_comp: Query<Entity, With<WinScreenComp>>) {
    for e in &q_comp {
        commands.entity(e).despawn_recursive();
    }
}

pub fn setup_win_screen(mut commands: Commands, ass: Res<AssetServer>) {
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
                                    font: ass.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 100.0,
                                    ..default()
                                },
                                Label,
                            ));

                            parent
                                .spawn((
                                    Button,
                                    GameMenuButton::RestartGame,
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
                                        font: ass.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));

                            parent
                                .spawn((
                                    Button,
                                    GameMenuButton::ExitToMenue,
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
                                        font: ass.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));
                        });
                });
        });
}
