use crate::{
    assets::{IconAssets, UiAssets},
    states::RunningState,
    ui::components::buttons::SpriteButton,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameUi;

impl Plugin for GameUi {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(RunningState::Run), Self::spawn)
            .add_systems(OnExit(RunningState::Run), Self::destroy);
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
                        width: Val::VMax(10.),
                        min_width: Val::Px(100.),
                        min_height: Val::Px(100.),
                        height: Val::VMax(10.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    SpriteButton,
                    SpriteButton::image(ui_asset.button.clone()),
                ))
                .with_child((ImageNode::new(icon_asset.reset.clone())))
                .observe(|_: Trigger<Pointer<Click>>| {
                    info!("Click - Reset");
                });
            });
    }

    pub fn destroy(mut commands: Commands, q_comp: Query<Entity, With<Self>>) {
        for e in &q_comp {
            commands.entity(e).despawn_recursive();
        }
    }
}
