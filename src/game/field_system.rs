use bevy::prelude::*;

use crate::game::OpenField;

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
                    Field { pos: (x, y).into() },
                    GameObject,
                ))
                .observe(handle_click)
                .id();
            board.fields.insert(
                (x, y).into(),
                FieldData {
                    entity: id,
                    ..default()
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
        info!("No Target");
        return;
    };

    if button == PointerButton::Primary {
        commands.trigger(OpenField { pos: field.pos });
    } else if PointerButton::Secondary == button {
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            warn!("Could not get texture atlas for fields");
            return;
        };

        if let Some(field_data) = board.fields.get_mut(&field.pos) {
            if matches!(field_data.status, FieldStatus::Flaged) {
                field_data.status = FieldStatus::Open;
                texture_atlas.index = 0;
            } else {
                field_data.status = FieldStatus::Flaged;
                texture_atlas.index = 1;
            }
        }
    }
}
