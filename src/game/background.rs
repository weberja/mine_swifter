use bevy::{prelude::*, sprite::AlphaMode2d, window::WindowResized};

use crate::materials::background::BackgroundMaterial;

#[derive(Component)]
pub struct Background;

pub fn setup_background(
    mut commands: Commands,
    window: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let size = window.resolution.clone();
    let mesh = meshes.add(Rectangle::new(1., 1.));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(BackgroundMaterial {
            start_color: LinearRgba::RED,
            end_color: LinearRgba::BLUE,
            position: 0.,
            size: 0.4,
            angle: 45.,
            alpha_mode: AlphaMode2d::Blend,
        })),
        Transform::from_xyz(0., 0., -100.).with_scale(size.size().extend(0.)),
        Background,
    ));
}

pub fn on_resize_background(
    mut ev: EventReader<WindowResized>,
    mut transform: Single<&mut Transform, With<Background>>,
) {
    for e in ev.read() {
        debug!("ResizedWindow");
        transform.scale = Vec3 {
            x: e.width,
            y: e.height,
            z: 0.,
        };
    }
}
