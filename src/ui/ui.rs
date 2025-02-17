use bevy::{prelude::*, ui, window::PrimaryWindow, winit::cursor::CursorIcon};

use crate::AppState;

use super::buttons::{button_color_system, button_interaction_system, rest_button_setup};

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

#[derive(Resource)]
struct UiAssets {
    cursor: Handle<Image>,
    button: Handle<Image>,
    button_pressed: Handle<Image>,
    button_hover: Handle<Image>,
}

#[derive(Resource)]
struct FontAssets {
    fira_code: Handle<Font>,
    future: Handle<Font>,
    future_narrow: Handle<Font>,
}

pub fn ui_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), rest_button_setup)
        .add_systems(
            OnEnter(AppState::Loading),
            (load_ui_assets, cursor_setup.after(load_ui_assets)),
        )
        .add_systems(OnExit(AppState::Game), cleanup)
        .add_systems(
            Update,
            (
                button_interaction_system,
                button_color_system.run_if(in_state(AppState::Game)),
            ),
        );
}

fn cursor_setup(
    mut commands: Commands,
    assets: Res<UiAssets>,
    window: Single<Entity, With<PrimaryWindow>>,
) {
    commands
        .entity(window.into_inner())
        .insert(CursorIcon::Custom(
            bevy::winit::cursor::CustomCursor::Image {
                handle: assets.cursor.clone(),
                hotspot: (5, 5),
            },
        ));
}

fn load_ui_assets(mut commands: Commands, ass: Res<AssetServer>) {
    let ui = UiAssets {
        cursor: ass.load("ui/cursor/PNG/Outline/Default/pointer_a.png"),
        button: ass.load(""),
        button_pressed: ass.load(""),
        button_hover: ass.load(""),
    };

    commands.insert_resource(ui);
}

fn cleanup() {}
