use bevy::{prelude::*, utils::info};

use crate::{
    board::{
        board_data::Board,
        events::OpenField,
        field::{Field, FieldStatus},
    },
    materials::field::FieldMaterial,
};

pub fn handle_click(
    ev: Trigger<Pointer<Click>>,
    mut sprites: Query<(&mut MeshMaterial2d<FieldMaterial>, &Field)>,
    mut materials: ResMut<Assets<FieldMaterial>>,
    mut board: ResMut<Board>,
    mut commands: Commands,
) {
    info!("Click!");

    let id = ev.entity();
    let button = ev.button;
    let Ok((material, field)) = sprites.get_mut(id) else {
        info!("No Target");
        return;
    };

    if button == PointerButton::Primary {
        commands.trigger(OpenField(field.0));
        info("Clicked with left mouse");
    } else if PointerButton::Secondary == button {
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
}
