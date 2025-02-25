use background::{on_resize_background, setup_background};
use bevy::{prelude::*, sprite::AlphaMode2d};
use click_on_board::handle_click;
use events::{CreateBoard, DestroyBoard};

use crate::{
    assets::BoardAssets,
    board::{board_data::Board, field::Field, BoardSettings},
    materials::{field::FieldMaterial, grid::GridMaterial},
    states::AppState,
};

pub mod background;
pub mod click_on_board;
pub mod events;
pub mod open_field;

#[derive(Component)]
pub struct GameObject;

pub fn game(app: &mut App) {
    app.add_observer(board_setup_observer)
        .add_observer(board_destroy_observer)
        .add_observer(open_field::open_field)
        .add_systems(OnExit(AppState::LoadingAssets), setup_background)
        .add_systems(
            Update,
            on_resize_background.run_if(not(in_state(AppState::LoadingAssets))),
        );
}

pub fn board_setup_observer(
    _trigger: Trigger<CreateBoard>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    field_materials: ResMut<Assets<FieldMaterial>>,
    grid_materials: ResMut<Assets<GridMaterial>>,
    settings: Res<BoardSettings>,
    field_assets: Res<BoardAssets>,
) {
    board_setup(
        &mut commands,
        meshes,
        field_materials,
        grid_materials,
        settings,
        field_assets,
    );
}
pub fn board_reset_observer(
    _trigger: Trigger<CreateBoard>,
    mut commands: Commands,
    objects: Query<Entity, With<GameObject>>,
) {
    board_destroy(&mut commands, objects);
}
pub fn board_destroy_observer(
    _trigger: Trigger<DestroyBoard>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    field_materials: ResMut<Assets<FieldMaterial>>,
    grid_materials: ResMut<Assets<GridMaterial>>,
    settings: Res<BoardSettings>,
    field_assets: Res<BoardAssets>,
    objects: Query<Entity, With<GameObject>>,
) {
    board_destroy(&mut commands, objects);
    board_setup(
        &mut commands,
        meshes,
        field_materials,
        grid_materials,
        settings,
        field_assets,
    );
}

fn board_setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut field_materials: ResMut<Assets<FieldMaterial>>,
    mut grid_materials: ResMut<Assets<GridMaterial>>,
    settings: Res<BoardSettings>,
    assets: Res<BoardAssets>,
) {
    let mut board = Board::new(settings.size, settings.bomb_count);
    let mesh = meshes.add(Rectangle::new(64., 64.));

    for x in 0..settings.size.x {
        for y in 0..settings.size.y {
            let id = commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(field_materials.add(FieldMaterial {
                        texture: assets.field.clone(),
                        alpha_mode: AlphaMode2d::Blend,
                        index: 0,
                    })),
                    Transform::from_translation(Vec3 {
                        x: 32. - (settings.size.x as f32 * 64. / 2.) + (x as f32 * 64.0),
                        y: 32. - (settings.size.y as f32 * 64. / 2.) + (y as f32 * 64.0),
                        z: -10.0,
                    }),
                    Field((x, y).into()),
                    GameObject,
                ))
                .observe(handle_click)
                .id();
            board.add_field((x, y).into(), id);
        }
    }

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            64. * settings.size.x as f32,
            64. * settings.size.y as f32,
        ))),
        MeshMaterial2d(grid_materials.add(GridMaterial {
            squars: settings.size.as_vec2(),
        })),
        Transform::from_translation(Vec3 {
            x: 0.,
            y: 0.,
            z: 0.0,
        }),
        PickingBehavior::IGNORE,
    ));

    commands.insert_resource(board);
}

fn board_destroy(commands: &mut Commands, objects: Query<Entity, With<GameObject>>) {
    for id in objects.iter() {
        commands.entity(id).despawn_recursive();
    }
}
