use bevy::prelude::*;

pub mod lost;
pub mod main_menu;
pub mod win;

pub fn screens(app: &mut App) {
    app.add_plugins((main_menu::main_menu, lost::lost_screen, win::win_screen));
}
