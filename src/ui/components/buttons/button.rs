use bevy::{prelude::*, ui::FocusPolicy};

/// Marker struct for buttons
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq)]
#[require(Node, FocusPolicy(|| FocusPolicy::Block), Interaction, ImageNode)]
pub struct SpriteButton;

