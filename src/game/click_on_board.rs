use bevy::{picking::pointer::PointerButton, prelude::*};

use crate::{
    board::{
        board_data::Board,
        events::OpenField,
        field::{Field, FieldStatus},
    },
    materials::field::FieldMaterial,
};

use super::undo::AddMoveAndClose;

#[derive(Event)]
pub struct OpenInteraction;

#[derive(Event)]
pub struct FlagInteraction;

#[derive(Event)]
pub struct BoardInteraction;

#[derive(Resource, Debug)]
pub struct TouchStatus {
    pub timer: Timer,
}

impl Default for TouchStatus {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.15, TimerMode::Once),
        }
    }
}

pub fn open_interaction_event(
    trigger: Trigger<OpenInteraction>,
    sprites: Query<&Field>,
    mut commands: Commands,
) {
    let Ok(field) = sprites.get(trigger.entity()) else {
        debug!("No Target");
        return;
    };

    commands.trigger(OpenField(field.0));
}

pub fn flag_interaction_event(
    trigger: Trigger<FlagInteraction>,
    mut sprites: Query<(&mut MeshMaterial2d<FieldMaterial>, &Field)>,
    mut materials: ResMut<Assets<FieldMaterial>>,
    mut board: ResMut<Board>,
    mut commands: Commands,
) {
    let Ok((material, field)) = sprites.get_mut(trigger.entity()) else {
        debug!("No Target");
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
            commands.trigger(AddMoveAndClose(field.0));
        }
    }
}

pub fn handle_down(
    ev: Trigger<Pointer<Down>>,
    mut commands: Commands,
    mut touch_status: ResMut<TouchStatus>,
) {
    let id = ev.entity();

    if ev.pointer_id.is_touch() {
        debug!("Touch Down");
        touch_status.timer.reset();
        touch_status.timer.unpause();
    } else if ev.pointer_id.is_mouse() {
        let button = ev.button;

        if button == PointerButton::Primary {
            commands.trigger_targets(OpenInteraction, id);
        } else if button == PointerButton::Secondary {
            commands.trigger_targets(FlagInteraction, id);
        }
        commands.trigger(BoardInteraction);
    }
}

pub fn update_touch_timer(mut touch_status: ResMut<TouchStatus>, time: Res<Time>) {
    touch_status.timer.tick(time.delta());
}

pub fn handle_up(
    ev: Trigger<Pointer<Up>>,
    mut commands: Commands,
    touch_status: ResMut<TouchStatus>,
) {
    let id = ev.entity();

    if ev.pointer_id.is_touch() {
        debug!("Touch Status: {:?}", touch_status);
        if touch_status.timer.finished() {
            commands.trigger_targets(FlagInteraction, id);
        } else {
            commands.trigger_targets(OpenInteraction, id);
        }
        commands.trigger(BoardInteraction);
    }
}
