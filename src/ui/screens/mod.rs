use bevy::prelude::*;
use bevy_lunex::{UiLunexDebugPlugin, UiLunexPlugins};

pub mod lost;
pub mod main_menu;
pub mod win;

pub fn screens(app: &mut App) {
    app.add_plugins((
        UiLunexPlugins,
        UiLunexDebugPlugin::<1, 2>,
        main_menu::main_menu,
        lost::lost_screen,
        win::win_screen,
    ));
}
