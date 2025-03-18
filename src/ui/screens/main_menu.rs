use bevy::prelude::*;

use crate::{
    assets::{FontAssets, UiAssets},
    states::AppState,
    ui::components::buttons::{
        button::SpriteButton,
        menu_buttons::{start_9x9, start_16x16, start_30x16},
    },
};

#[derive(Component)]
pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), Self::spawn)
            .add_systems(
                Update,
                Self::button_interaction.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), Self::destroy);
    }
}

impl MainMenu {
    pub fn spawn(mut commands: Commands, fonts: Res<FontAssets>, ui_asset: Res<UiAssets>) {
        let button_image = ImageNode::new(ui_asset.button.clone()).with_mode(
            bevy::ui::widget::NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect::rectangle(30., 30.),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            }),
        );

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.),
                    ..default()
                },
                Self,
            ))
            .insert(PickingBehavior::IGNORE)
            .with_children(|ui| {
                ui.spawn((
                    Node {
                        width: Val::Vw(40.),
                        min_width: Val::Px(120.),
                        min_height: Val::Px(100.),
                        height: Val::Vh(20.),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    SpriteButton,
                    button_image.clone(),
                ))
                .with_child((
                    Text::new("9x9"),
                    TextFont {
                        font: fonts.fira_code.clone(),
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .observe(start_9x9);
                ui.spawn((
                    Node {
                        width: Val::Vw(40.),
                        min_width: Val::Px(120.),
                        min_height: Val::Px(100.),
                        height: Val::Vh(20.),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    SpriteButton,
                    button_image.clone(),
                ))
                .with_child((
                    Text::new("16x16"),
                    TextFont {
                        font: fonts.fira_code.clone(),
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .observe(start_16x16);
                ui.spawn((
                    Node {
                        width: Val::Vw(40.),
                        min_width: Val::Px(120.),
                        min_height: Val::Px(100.),
                        height: Val::Vh(20.),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    SpriteButton,
                    button_image.clone(),
                ))
                .with_child((
                    Text::new("16x30"),
                    TextFont {
                        font: fonts.fira_code.clone(),
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .observe(start_30x16);
            });
    }

    pub fn destroy(mut commands: Commands, q_comp: Query<Entity, With<Self>>) {
        for e in &q_comp {
            commands.entity(e).despawn_recursive();
        }
    }

    #[allow(clippy::complexity)]
    pub fn button_interaction(
        mut interaction_query: Query<
            (&Interaction, &mut ImageNode),
            (Changed<Interaction>, With<SpriteButton>),
        >,
        ui_assets: Res<UiAssets>,
    ) {
        for (interaction, mut image) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => image.image = ui_assets.button_pressed.clone(),
                Interaction::Hovered => image.image = ui_assets.button_hover.clone(),
                Interaction::None => image.image = ui_assets.button.clone(),
            }
        }
    }
}
