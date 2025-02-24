use crate::{
    board::{board_data::Board, events::OpenField, field::Field},
    materials::field::FieldMaterial,
};
use bevy::{prelude::*, utils::HashSet};

use super::events::GameLost;

pub fn open_field(
    trigger: Trigger<OpenField>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    fields: Query<&MeshMaterial2d<FieldMaterial>, With<Field>>,
    mut materials: ResMut<Assets<FieldMaterial>>,
) {
    let mut visited = HashSet::new();
    let mut to_visit = HashSet::from([trigger.0]);

    if !board.generated {
        board.generate();
    }

    while !to_visit.is_empty() {
        let working_list = to_visit.clone();
        to_visit.clear();
        for pos in working_list {
            visited.insert(pos);

            let Some(field_data) = board.fields.get_mut(&pos) else {
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

            if board.is_bomb(pos) {
                to_visit.clear();
                commands.trigger(GameLost);
                break;
            } else {
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

                if board.fullfilled(pos) {
                    to_visit.extend(
                        board
                            .neighbors(pos)
                            .iter()
                            .filter(|&var| !visited.contains(var)),
                    );
                }
            }
        }
    }
}
