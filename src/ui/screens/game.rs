use crate::{
    assets::{IconAssets, UiAssets},
    game::interactions::game::undo,
    states::AppState,
    ui::components::buttons::SpriteButton,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameUi;

impl Plugin for GameUi {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), Self::spawn)
            .add_systems(OnExit(AppState::Game), Self::destroy);
    }
}

impl GameUi {
    pub fn spawn(mut commands: Commands, ui_asset: Res<UiAssets>, icon_asset: Res<IconAssets>) {
        commands
            .spawn((
                Node {
                    border: UiRect::all(Val::Vh(5.)),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Stretch,
                    row_gap: Val::Px(10.),
                    ..default()
                },
                Self,
                PickingBehavior::IGNORE,
            ))
            .with_children(|ui| {
                ui.spawn((
                    Node {
                        width: Val::VMax(2.),
                        min_width: Val::Px(80.),
                        min_height: Val::Px(80.),
                        height: Val::VMax(2.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    SpriteButton,
                    SpriteButton::image(ui_asset.button.clone()),
                ))
                .with_child((
                    Node {
                        width: Val::Percent(75.),
                        height: Val::Percent(75.),
                        ..default()
                    },
                    ImageNode::new(icon_asset.reset.clone()),
                ))
                .observe(undo);
            });
    }

    pub fn destroy(mut commands: Commands, q_comp: Query<Entity, With<Self>>) {
        for e in &q_comp {
            commands.entity(e).despawn_recursive();
        }
    }
}
