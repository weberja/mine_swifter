use bevy::ecs::event::Event;

#[derive(Event)]
pub struct ResetBoard;

#[derive(Event)]
pub struct CreateBoard;

#[derive(Event)]
pub struct DestroyBoard;
