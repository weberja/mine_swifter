use bevy::{color::palettes::tailwind::GRAY_700, prelude::*};

use crate::{
    assets::FontAssets,
    states::AppState,
    ui::components::buttons::{menu_buttons::MenuButton, NORMAL_BUTTON},
};

#[derive(Component)]
struct MainMenuComponent;

pub fn main_menu(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(OnExit(AppState::MainMenu), destroy_menu);
}

fn setup_menu(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn((
        Text::new("MineSwifter!"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: fonts.fira_code.clone(),
            font_size: 67.0,
            ..default()
        },
        TextColor(GRAY_700.into()),
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Right),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            right: Val::Px(25.0),
            ..default()
        },
        MainMenuComponent,
    ));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
            MainMenuComponent,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    row_gap: Val::Px(100.),
                    ..default()
                })
                .with_children(|setting_node| {
                    setting_node
                        .spawn((
                            Button,
                            MenuButton::S9x9,
                            Node {
                                padding: UiRect::all(Val::Px(5.)),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(100.),
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BackgroundColor(NORMAL_BUTTON),
                        ))
                        .with_children(|node| {
                            node.spawn((
                                Text::new("9 x 9"),
                                TextFont {
                                    font: fonts.fira_code.clone(),
                                    font_size: 33.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));

                            node.spawn((
                                Text::new("10 Bombs"),
                                TextFont {
                                    font: fonts.fira_code.clone(),
                                    font_size: 33.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });

                    setting_node
                        .spawn((
                            Button,
                            MenuButton::S16x16,
                            Node {
                                padding: UiRect::all(Val::Px(5.)),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(100.),
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BackgroundColor(NORMAL_BUTTON),
                        ))
                        .with_children(|node| {
                            node.spawn((
                                Text::new("16 x 16"),
                                TextFont {
                                    font: fonts.fira_code.clone(),
                                    font_size: 33.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));

                            node.spawn((
                                Text::new("40 Bombs"),
                                TextFont {
                                    font: fonts.fira_code.clone(),
                                    font_size: 33.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });
                    setting_node
                        .spawn((
                            Button,
                            MenuButton::S30x16,
                            Node {
                                padding: UiRect::all(Val::Px(5.)),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(100.),
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BackgroundColor(NORMAL_BUTTON),
                        ))
                        .with_children(|node| {
                            node.spawn((
                                Text::new("30 x 16"),
                                TextFont {
                                    font: fonts.fira_code.clone(),
                                    font_size: 33.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));

                            node.spawn((
                                Text::new("99 Bombs"),
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

fn destroy_menu(mut commands: Commands, q_comp: Query<Entity, With<MainMenuComponent>>) {
    for e in &q_comp {
        commands.entity(e).despawn_recursive();
    }
}
