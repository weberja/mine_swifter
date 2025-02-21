use bevy::prelude::*;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    LoadingAssets,
    MainMenu,
    Game,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    #[default]
    Run,
    Pause,
    Lost,
    Won,
}

pub fn states(app: &mut App) {
    app.init_state::<AppState>().add_sub_state::<GameState>();
}
