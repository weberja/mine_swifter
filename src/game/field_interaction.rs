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

pub fn open_field(
    trigger: Trigger<OpenField>,
    mut board: ResMut<Board>,
    fields: Query<&MeshMaterial2d<FieldMaterial>, With<Field>>,
    mut materials: ResMut<Assets<FieldMaterial>>,
    rng: ResMut<RandomSource<ChaCha8Rng>>,
    mut next_state: ResMut<NextState<GameState>>,
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
                material_data.index = 1;
                next_state.set(GameState::Lost);
                break;
            } else {
                field_data.status = FieldStatus::Open;

                let count_neighbors = board.bombs(pos);
                info!("{} with {}", pos, count_neighbors);

                match count_neighbors {
                    1 => material_data.index = 3,
                    2 => material_data.index = 4,
                    3 => material_data.index = 5,
                    4 => material_data.index = 6,
                    5 => material_data.index = 7,
                    6 => material_data.index = 8,
                    7 => material_data.index = 9,
                    8 => material_data.index = 10,
                    _ => material_data.index = 2,
                }

                info!(
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
}
