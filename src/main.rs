#![feature(int_roundings)]
#![feature(duration_constructors)]

mod game;
mod ui;

use bevy::prelude::*;

#[derive(Clone, Default, PartialEq, Eq, Debug, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Game,
    Menu,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::Game)]
enum GameState {
    #[default]
    Run,
    Pause,
    Lost,
    Won,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(game::game_plugin)
        .init_state::<AppState>()
        .add_sub_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(ui::ui_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
