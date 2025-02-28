use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct Zoom(pub f32);

#[derive(Debug, Event)]
pub struct ResetView;

#[derive(Debug, Event)]
pub struct FixCamera;

#[derive(Debug, Event)]
pub struct UnFixCamera;
