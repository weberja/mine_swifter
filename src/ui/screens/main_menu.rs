use bevy::prelude::*;
use bevy_lunex::{
    Anchor, Rh, Rl, Rw, UiColor, UiFetchFromCamera, UiLayout, UiLayoutRoot, UiTextSize,
};

use crate::{
    assets::{FontAssets, UiAssets},
    states::AppState,
};

#[derive(Component)]
struct MainMenuComponent;

pub fn main_menu(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(OnExit(AppState::MainMenu), destroy_menu);
}

fn setup_menu(mut commands: Commands, fonts: Res<FontAssets>, ui_asset: Res<UiAssets>) {
    let mut button = Sprite::from_image(ui_asset.button.clone());
    button.image_mode = SpriteImageMode::Sliced(TextureSlicer {
        border: BorderRect::rectangle(10., 20.),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    });

    commands
        .spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>))
        .with_children(|ui| {
            ui.spawn((
                Name::new("My R"),
                UiLayout::window()
                    .anchor(Anchor::Center)
                    .pos(Rl((50., 50.)))
                    .size(Rl((20., 20.)))
                    .pack(),
                //UiColor::from(Color::srgb(1.0, 0., 0.)),
                button.clone(),
            ))
            .with_child((
                Name::new("Text"),
                UiLayout::window()
                    .anchor(Anchor::Center)
                    .pos(Rl((50., 50.)))
                    .pack(),
                UiTextSize::from(Rw(30.)),
                Text2d::new("16x16"),
                TextFont {
                    font: fonts.future_narrow.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ))
            .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
                info!("Click");
            });
        });
}

fn destroy_menu(mut commands: Commands, q_comp: Query<Entity, With<MainMenuComponent>>) {
    for e in &q_comp {
        commands.entity(e).despawn_recursive();
    }
}
