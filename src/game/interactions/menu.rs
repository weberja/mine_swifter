#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::{board::BoardSettings, states::AppState};

pub fn start_9x9(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Game);
    commands.insert_resource(BoardSettings {
        size: (9, 9).into(),
        solvable: false,
        bomb_count: 10,
    });
}

pub fn start_16x16(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Game);
    commands.insert_resource(BoardSettings {
        size: (16, 16).into(),
        solvable: false,
        bomb_count: 40,
    });
}

pub fn start_30x16(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Game);
    commands.insert_resource(BoardSettings {
        size: (30, 16).into(),
        solvable: false,
        bomb_count: 99,
    });
}
