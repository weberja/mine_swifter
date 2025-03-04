use std::time::Duration;

use bevy::{picking::pointer::PointerButton, prelude::*, ui::Interaction, utils::info};

use crate::{
    board::{
        board_data::Board,
        events::OpenField,
        field::{Field, FieldStatus},
    },
    materials::field::FieldMaterial,
};

#[derive(Event)]
pub struct OpenInteraction;

#[derive(Event)]
pub struct FlagInteraction;

#[derive(Event)]
pub struct BoardInteraction;

pub fn open_interaction_event(
    trigger: Trigger<OpenInteraction>,
    sprites: Query<&Field>,
    mut commands: Commands,
) {
    let Ok(field) = sprites.get(trigger.entity()) else {
        info!("No Target");
        return;
    };

    commands.trigger(OpenField(field.0));
    info("Clicked with left mouse");
}

pub fn flag_interaction_event(
    trigger: Trigger<FlagInteraction>,
    mut sprites: Query<(&mut MeshMaterial2d<FieldMaterial>, &Field)>,
    mut materials: ResMut<Assets<FieldMaterial>>,
    mut board: ResMut<Board>,
) {
    let Ok((material, field)) = sprites.get_mut(trigger.entity()) else {
        info!("No Target");
        return;
    };

    let Some(material_data) = &mut materials.get_mut(material.id()) else {
        warn!("Could not get material for fields");
        return;
    };

    if let Some(field_data) = board.fields.get_mut(&field.0) {
        if matches!(field_data.status, FieldStatus::Flaged) {
            field_data.status = FieldStatus::Open;
            material_data.index = 0;
        } else {
            field_data.status = FieldStatus::Flaged;
            material_data.index = 1;
        }
    }
}

pub fn handle_click(ev: Trigger<Pointer<Click>>, mut commands: Commands) {
    info!("Click!");

    let id = ev.entity();
    let button = ev.button;

    if button == PointerButton::Primary {
        commands.trigger_targets(OpenInteraction, id);
    } else if button == PointerButton::Secondary {
        commands.trigger_targets(FlagInteraction, id);
    }
    commands.trigger(BoardInteraction);
}

pub fn handle_touch(
    trigger: Trigger<TouchInput>,
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<Timer>,
    mut moved: Local<bool>,
) {
    match trigger.phase {
        bevy::input::touch::TouchPhase::Started => {
            timer.reset();
            *moved = false;
        }
        bevy::input::touch::TouchPhase::Moved => *moved = true,
        bevy::input::touch::TouchPhase::Ended => {
            timer.tick(time.elapsed());
            if timer.duration() >= Duration::from_secs(3) {
                commands.trigger_targets(FlagInteraction, trigger.entity());
                commands.trigger(BoardInteraction);
            }
        }
        bevy::input::touch::TouchPhase::Canceled => *moved = false,
    };
}
