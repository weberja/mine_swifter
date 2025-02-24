use bevy::ecs::event::Event;

#[derive(Event)]
pub struct ResetBoard;

#[derive(Event)]
pub struct CreateBoard;

#[derive(Event)]
pub struct DestroyBoard;

#[derive(Event)]
pub struct GameLost;

#[derive(Event)]
pub struct GameWin;
