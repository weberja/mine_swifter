use crate::{
    board::{
        board_data::Board,
        events::OpenField,
        field::{Field, FieldStatus},
    },
    materials::field::FieldMaterial,
    states::GameState,
    utils::random::RandomSource,
};
use bevy::{prelude::*, utils::HashSet};
use rand_chacha::ChaCha8Rng;

use super::undo::{AddMove, MoveDone, ReverteAction};

pub fn close_field(
    trigger: Trigger<ReverteAction>,
    mut board: ResMut<Board>,
    fields: Query<&MeshMaterial2d<FieldMaterial>, With<Field>>,
    mut materials: ResMut<Assets<FieldMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for field_pos in trigger.action.moves.iter() {
        let Some(field_data) = board.fields.get_mut(field_pos) else {
            warn!("Can not get mutabale field data for pos");
            return;
        };

        let Ok(material) = fields.get(field_data.entity) else {
            warn!("Could not find material for entity");
            return;
        };

        let Some(material_data) = materials.get_mut(material.id()) else {
            warn!("Could not get material_data for fields");
            return;
        };

        if field_data.bomb {
            next_state.set(GameState::Run);
        }

        field_data.status = FieldStatus::Closed;
        material_data.index = 0;
    }
}

pub fn open_field(
    trigger: Trigger<OpenField>,
    mut board: ResMut<Board>,
    fields: Query<&MeshMaterial2d<FieldMaterial>, With<Field>>,
    mut materials: ResMut<Assets<FieldMaterial>>,
    rng: ResMut<RandomSource<ChaCha8Rng>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let mut visited = HashSet::new();
    let mut to_visit = HashSet::from([trigger.0]);

    if !board.generated {
        board.generate(rng, trigger.0);
    }

    let max_depth = 1;
    let mut depth = 0;

    while !to_visit.is_empty() {
        let working_list = to_visit.clone();
        to_visit.clear();

        for pos in working_list.clone() {
            visited.insert(pos);

            let Some(field_data) = board.fields.get_mut(&pos) else {
                warn!("Can not get mutabale field data for pos");
                return;
            };

            // skip field if it is a flag
            if matches!(field_data.status, FieldStatus::Flaged) {
                continue;
            }

            let Ok(material) = fields.get(field_data.entity) else {
                warn!("Could not find material for entity");
                return;
            };

            let Some(material_data) = materials.get_mut(material.id()) else {
                warn!("Could not get material_data for fields");
                return;
            };

            if field_data.bomb {
                to_visit.clear();
                material_data.index = 12;
                next_state.set(GameState::Lost);
                commands.trigger(AddMove(pos));
                break;
            } else {
                field_data.status = FieldStatus::Open;

                let count_neighbors = board.bombs(pos);
                debug!("{} with {}", pos, count_neighbors);

                let new_index = match count_neighbors {
                    1 => 3,
                    2 => 4,
                    3 => 5,
                    4 => 6,
                    5 => 7,
                    6 => 8,
                    7 => 9,
                    8 => 10,
                    _ => 2,
                };

                if material_data.index != new_index {
                    material_data.index = new_index;
                    commands.trigger(AddMove(pos));
                }

                debug!(
                    "Field {} fullfilled: {}, empty: {}",
                    pos,
                    board.fullfilled(pos),
                    (depth >= max_depth && board.bombs(pos) == 0)
                );

                if (board.fullfilled(pos) && depth < max_depth)
                    || (depth >= max_depth && board.bombs(pos) == 0)
                {
                    to_visit.extend(
                        board
                            .neighbors(pos)
                            .iter()
                            .filter(|&var| !visited.contains(var) && !working_list.contains(var)),
                    );
                }
            }
            depth += 1;
        }
    }
    commands.trigger(MoveDone);
}
