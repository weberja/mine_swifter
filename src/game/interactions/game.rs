use bevy::prelude::*;

use crate::game::revert::Undo;

pub fn undo(_trigger: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(Undo);
}
