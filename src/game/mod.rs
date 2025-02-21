use bevy::{prelude::*, render::settings, sprite::AlphaMode2d};
use click_on_board::handle_click;
use events::{CreateBoard, DestroyBoard};

use crate::{
    assets::BoardAssets,
    board::{board_data::Board, field::Field, BoardSettings},
    materials::field::FieldMaterial,
};

pub mod click_on_board;
pub mod events;

#[derive(Component)]
pub struct GameObject;

pub fn game(app: &mut App) {
    app.add_observer(board_setup_observer)
        .add_observer(board_destroy_observer);
}

pub fn board_setup_observer(
    _trigger: Trigger<CreateBoard>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<FieldMaterial>>,
    settings: Res<BoardSettings>,
    field_assets: Res<BoardAssets>,
) {
    board_setup(&mut commands, meshes, materials, settings, field_assets);
}
pub fn board_reset_observer(_trigger: Trigger<CreateBoard>, mut commands: Commands) {
    board_destroy(&mut commands);
}
pub fn board_destroy_observer(
    _trigger: Trigger<DestroyBoard>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<FieldMaterial>>,
    settings: Res<BoardSettings>,
    field_assets: Res<BoardAssets>,
) {
    board_destroy(&mut commands);
    board_setup(&mut commands, meshes, materials, settings, field_assets);
}

fn board_setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FieldMaterial>>,
    settings: Res<BoardSettings>,
    assets: Res<BoardAssets>,
) {
    commands.insert_resource(Board::new(settings.size, settings.bomb_count));
    let mesh = meshes.add(Rectangle::new(64., 64.));

    for x in 0..settings.size.x {
        for y in 0..settings.size.y {
            commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(materials.add(FieldMaterial {
                        texture: assets.field.clone(),
                        alpha_mode: AlphaMode2d::Blend,
                        index: 0,
                    })),
                    Transform::from_translation(Vec3 {
                        x: 32. - (settings.size.x as f32 * 64. / 2.) + (x as f32 * 64.0),
                        y: 32. - (settings.size.y as f32 * 64. / 2.) + (y as f32 * 64.0),
                        z: 0.0,
                    }),
                    Field((x, y).into()),
                    GameObject,
                ))
                .observe(handle_click);
        }
    }
}

fn board_destroy(commands: &mut Commands) {}
