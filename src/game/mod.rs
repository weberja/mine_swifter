mod board;
pub mod board_generator;
mod field_system;

use bevy::prelude::*;
use board::{Board, BoardAssets, BoardSettings};
use field_system::{setup_board, setup_board_init};

use crate::{AppState, GameState};

#[derive(Component)]
struct GameObject;

#[derive(Event)]
pub struct RestartGame;

#[derive(Event)]
pub struct ExitGame;

#[derive(Event)]
pub struct ResetBoard;

#[derive(Event)]
pub struct CreateBoard;

#[derive(Event)]
pub struct DestroyBoard;

#[derive(Event)]
pub struct OpenNeighbors {
    pos: UVec2,
    open_more: bool,
}

#[derive(Event)]
pub struct OpenField {
    pos: UVec2,
}

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Loading), loading_assets)
        .add_systems(OnEnter(AppState::Game), setup_board_init)
        .add_systems(OnExit(AppState::Game), destroy_init)
        .add_observer(reset_board)
        .add_observer(destroy)
        .add_observer(setup_board)
        .add_observer(Board::open_neighbors)
        .add_observer(Board::open_field)
        .add_observer(restart_game);
}

fn restart_game(
    _trigger: Trigger<RestartGame>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    commands.trigger(DestroyBoard);
    commands.trigger(CreateBoard);
    next_state.set(GameState::Run);
}

fn reset_board(_trigger: Trigger<ResetBoard>, mut commands: Commands) {
    commands.trigger(DestroyBoard);
    commands.trigger(CreateBoard);
}

fn loading_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 3, 3, None, None);

    let texture_atlas_layout = texture_atlas_layout.add(layout);

    commands.insert_resource(BoardSettings {
        size: UVec2::splat(10),
        bombs: 10,
        solvable: false,
    });

    commands.insert_resource(BoardAssets {
        field_texture: asset_server.load("fields.png"),
        atlas_unpressed: TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        default_scale: Vec3::splat(1.0),
    });

    game_state.set(AppState::Game);
}

fn destroy_init(mut commands: Commands) {
    commands.trigger(DestroyBoard);
}

fn destroy(
    _trigger: Trigger<DestroyBoard>,
    mut q_objects: Query<Entity, With<GameObject>>,
    mut commands: Commands,
) {
    for obj in &mut q_objects {
        commands.entity(obj).try_despawn_recursive();
    }
}
