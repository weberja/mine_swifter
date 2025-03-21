use bevy::prelude::*;
use game::GameUi;
use main_menu::MainMenu;

use super::components::buttons::SpriteButton;

pub mod game;
pub mod main_menu;

pub fn screens(app: &mut App) {
    app.add_plugins((MainMenu, GameUi, SpriteButton));
}
