#![allow(clippy::too_many_arguments)]

use background::{on_resize_background, setup_background};
use bevy::{prelude::*, sprite::AlphaMode2d};
use click_on_board::{BoardInteraction, TouchStatus, handle_down, handle_up, update_touch_timer};

use crate::{
    assets::BoardAssets,
    board::{BoardSettings, board_data::Board, events::RestartGame, field::Field},
    camera::{ZoomableObject, events::ResetView},
    materials::{field::FieldMaterial, grid::GridMaterial},
    states::{AppState, GameState},
};

pub mod background;
pub mod click_on_board;
pub mod events;
pub mod field_interaction;

#[derive(Component)]
pub struct GameObject;

pub fn game(app: &mut App) {
    app.add_observer(field_interaction::open_field)
        .add_observer(click_on_board::flag_interaction_event)
        .add_observer(click_on_board::open_interaction_event)
        .add_observer(board_rest)
        .add_observer(won)
        .init_resource::<TouchStatus>()
        .add_systems(OnExit(AppState::LoadingAssets), setup_background)
        .add_systems(OnExit(AppState::Game), board_destroy)
        .add_systems(OnEnter(AppState::Game), board_setup)
        .add_systems(
            Update,
            (
                on_resize_background.run_if(not(in_state(AppState::LoadingAssets))),
                update_touch_timer,
            ),
        );
}

fn won(
    _trigger: Trigger<BoardInteraction>,
    mut next_state: ResMut<NextState<GameState>>,
    board: Res<Board>,
) {
    if board.solved() {
        next_state.set(GameState::Won);
    }
}

fn board_rest(
    _trigger: Trigger<RestartGame>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut field_materials: ResMut<Assets<FieldMaterial>>,
    mut grid_materials: ResMut<Assets<GridMaterial>>,
    _q: Query<&ZoomableObject>,
    settings: Res<BoardSettings>,
    assets: Res<BoardAssets>,
    objects: Query<Entity, With<GameObject>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Run);

    for id in objects.iter() {
        commands.entity(id).despawn_recursive();
    }

    let mut board = Board::new(settings.size, settings.bomb_count);
    let mesh = meshes.add(Rectangle::new(64., 64.));

    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            ZoomableObject,
            GameObject,
            PickingBehavior::IGNORE,
        ))
        .with_children(|parent| {
            for x in 0..settings.size.x {
                for y in 0..settings.size.y {
                    let id = parent
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
                        ))
                        .observe(handle_up)
                        .observe(handle_down)
                        .id();
                    board.add_field((x, y).into(), id);
                }
            }

            parent.spawn((
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
        });

    commands.insert_resource(board);
    commands.trigger(ResetView);
}

fn board_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut field_materials: ResMut<Assets<FieldMaterial>>,
    mut grid_materials: ResMut<Assets<GridMaterial>>,
    settings: Res<BoardSettings>,
    assets: Res<BoardAssets>,
) {
    let mut board = Board::new(settings.size, settings.bomb_count);
    let mesh = meshes.add(Rectangle::new(64., 64.));

    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            ZoomableObject,
            GameObject,
            PickingBehavior::IGNORE,
        ))
        .with_children(|parent| {
            for x in 0..settings.size.x {
                for y in 0..settings.size.y {
                    let id = parent
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
                        ))
                        .observe(handle_up)
                        .observe(handle_down)
                        .id();
                    board.add_field((x, y).into(), id);
                }
            }

            parent.spawn((
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
        });

    commands.insert_resource(board);
    commands.trigger(ResetView);
}

fn board_destroy(mut commands: Commands, objects: Query<Entity, With<GameObject>>) {
    for id in objects.iter() {
        commands.entity(id).despawn_recursive();
    }
}
