use bevy::prelude::*;

use crate::game::OpenNeighbors;

use super::{
    board::{Board, BoardAssets, BoardSettings, Field, FieldData, FieldStatus},
    CreateBoard, GameObject,
};

pub fn setup_board_init(mut commands: Commands) {
    commands.trigger(CreateBoard);
}

pub fn setup_board(
    _trigger: Trigger<CreateBoard>,
    mut commands: Commands,
    field: Res<BoardSettings>,
    field_assets: Res<BoardAssets>,
) {
    let mut board = Board::new(field.size.x, field.size.y);

    for x in 0..field.size.x {
        for y in 0..field.size.y {
            let id = commands
                .spawn((
                    Sprite::from_atlas_image(
                        field_assets.field_texture.clone(),
                        field_assets.atlas_unpressed.clone(),
                    ),
                    Transform::from_scale(field_assets.default_scale).with_translation(Vec3 {
                        x: (x as f32 - (field.size.x as f32 / 2.0)) * 64.0,
                        y: (y as f32 - (field.size.y as f32 / 2.0)) * 64.0,
                        z: 0.0,
                    }),
                    Field { x, y },
                    GameObject,
                ))
                .observe(handle_click)
                .id();
            board.fields.set(
                x as usize,
                y as usize,
                FieldData {
                    entity: id,
                    status: FieldStatus::default(),
                },
            );
        }
    }

    commands.insert_resource(board);
}

fn handle_click(
    ev: Trigger<Pointer<Click>>,
    mut sprites: Query<(&mut Sprite, &Field)>,
    mut board: ResMut<Board>,
    mut commands: Commands,
) {
    info!("Click!");
    let id = ev.entity();
    let button = ev.button;

    let Ok((mut sprite, field)) = sprites.get_mut(id) else {
        return;
    };

    if button == PointerButton::Primary {
        commands.trigger(OpenNeighbors {
            pos: UVec2 {
                x: field.x,
                y: field.y,
            },
        });
    } else if PointerButton::Secondary == button {
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            warn!("Could not get texture atlas for fields");
            return;
        };

        if texture_atlas.index == 1 || texture_atlas.index == 2 {
            return;
        }

        if let Some(field_data) = board.fields.get_mut(field.x as usize, field.y as usize) {
            if matches!(field_data.status, FieldStatus::Flaged) {
                field_data.status = FieldStatus::Open;
                board.flags.set(field.x as usize, field.y as usize, false);
                texture_atlas.index = 0;
            } else {
                field_data.status = FieldStatus::Flaged;
                board.flags.set(field.x as usize, field.y as usize, true);
                texture_atlas.index = 1;
            }
        }
    }
}
