mod buttons;
pub mod lost;
mod ui;
pub mod won;

use std::{ops::Sub, time::Duration};

use bevy::{prelude::*, time::Stopwatch};

use crate::{AppState, GameState};

#[derive(Component)]
pub struct TimeCounter {
    timer: Stopwatch,
}

#[derive(Component)]
pub struct TimerLable;

pub fn ui_plugin(app: &mut App) {
    app.add_plugins(ui::ui_plugin)
        .add_systems(OnEnter(AppState::Game), timer_setup)
        .add_systems(OnExit(AppState::Game), timer_destroy)
        .add_systems(OnEnter(GameState::Run), timer_reset)
        .add_systems(Update, timer.run_if(in_state(GameState::Run)))
        .add_systems(OnEnter(GameState::Won), crate::ui::won::setup_win_screen)
        .add_systems(OnExit(GameState::Won), crate::ui::won::destroy_win_screen)
        .add_systems(OnEnter(GameState::Lost), crate::ui::lost::setup_lost_screen)
        .add_systems(
            OnExit(GameState::Lost),
            crate::ui::lost::destroy_lost_screen,
        );
}

pub fn timer(
    timer: Single<&mut TimeCounter>,
    text_lables: Single<&mut Text, With<TimerLable>>,
    game_time: Res<Time>,
) {
    let mut timer = timer.into_inner();

    let mut lable = text_lables.into_inner();

    timer.timer.tick(game_time.delta());

    let time = timer.timer.elapsed();
    let minutes = time.as_secs().div_floor(60);
    let secounds = time.sub(Duration::from_mins(minutes)).as_secs();

    **lable = format!("{:0>2}:{:0>2}", minutes, secounds);
}

pub fn timer_setup(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn(TimeCounter {
        timer: Stopwatch::new(),
    });

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Px(60.),
            justify_content: JustifyContent::Center,
            margin: UiRect {
                left: Val::ZERO,
                right: Val::ZERO,
                top: Val::Px(25.),
                bottom: Val::ZERO,
            },
            ..default()
        })
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {
            parent.spawn((
                Text::new("Timer: "),
                TextFont {
                    font: ass.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 25.0,
                    ..default()
                },
                Label,
            ));
            parent.spawn((
                TimerLable,
                Text::new("0:00"),
                TextFont {
                    font: ass.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 25.0,
                    ..default()
                },
                Label,
            ));
        });
}

pub fn timer_reset(timer: Single<&mut TimeCounter>) {
    timer.into_inner().timer.reset();
}

pub fn timer_destroy(mut commands: Commands, timer: Single<Entity, With<TimeCounter>>) {
    commands.entity(timer.into_inner()).despawn();
}
