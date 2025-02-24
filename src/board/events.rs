use bevy::prelude::*;

#[derive(Event)]
pub struct RestartGame;

#[derive(Event)]
pub struct OpenField(pub UVec2);
