use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{board::BoardSettings, states::AppState};

#[derive(Resource, AssetCollection)]
pub struct BoardAssets {
    #[asset(path = "fields.png")]
    #[asset(image(array_texture_layers = 11))]
    pub field: Handle<Image>,
}

#[derive(Resource, AssetCollection)]
pub struct UiAssets {
    #[asset(path = "ui/cursor/PNG/Outline/Default/pointer_c.png")]
    pub cursor: Handle<Image>,
    #[asset(path = "ui/PNG/Grey/Default/button_rectangle_depth_flat.png")]
    pub button: Handle<Image>,
    #[asset(path = "ui/PNG/Grey/Default/button_rectangle_depth_gradient.png")]
    pub button_hover: Handle<Image>,
    #[asset(path = "ui/PNG/Grey/Default/button_rectangle_depth_gloss.png")]
    pub button_pressed: Handle<Image>,
}

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_code: Handle<Font>,
    #[asset(path = "fonts/Kenney Future.ttf")]
    pub future: Handle<Font>,
    #[asset(path = "fonts/Kenney Future Narrow.ttf")]
    pub future_narrow: Handle<Font>,
}

pub fn startup_assets(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(AppState::LoadingAssets)
            .continue_to_state(AppState::MainMenu)
            .load_collection::<UiAssets>()
            .load_collection::<FontAssets>()
            .load_collection::<BoardAssets>(),
    );
    app.insert_resource(BoardSettings {
        size: (9, 9).into(),
        solvable: false,
        bomb_count: 15,
    });
}
