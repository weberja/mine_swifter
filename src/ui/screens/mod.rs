use bevy::prelude::*;
use main_menu::MainMenu;

pub mod lost;
pub mod main_menu;
pub mod win;

pub fn screens(app: &mut App) {
    app.add_plugins((MainMenu, lost::lost_screen, win::win_screen));
}
