pub mod events;

use bevy::{
    input::{gestures::PinchGesture, mouse::MouseWheel},
    prelude::*,
};
use events::{ResetView, Zoom};

use crate::{assets::BoardAssets, board::BoardSettings};

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
pub struct ZoomableObject;

pub fn camera(app: &mut App) {
    app.add_systems(Startup, main_camera_setup)
        .add_systems(Update, (zoom_touch, zoom_mouse))
        .add_observer(zoom)
        .add_observer(reset_view);
}

fn main_camera_setup(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn zoom_touch(mut trigger: EventReader<PinchGesture>, mut commands: Commands) {
    for ev in trigger.read() {
        commands.trigger(Zoom(ev.0));
    }
}

fn zoom_mouse(mut trigger: EventReader<MouseWheel>, mut commands: Commands) {
    for ev in trigger.read() {
        commands.trigger(Zoom(ev.y));
    }
}

fn zoom(trigger: Trigger<Zoom>, mut obj: Single<&mut Transform, With<ZoomableObject>>) {
    obj.scale = (obj.scale + (Vec3::new(1., 1., 0.) * trigger.0))
        .clamp(Vec3::new(0.4, 0.4, f32::MIN), Vec3::new(30., 30., f32::MAX));
}

fn reset_view(
    _trigger: Trigger<ResetView>,
    obj: Single<(&GlobalTransform, &mut Transform), With<ZoomableObject>>,
    settings: Res<BoardSettings>,
    assets: Res<BoardAssets>,
    images: Res<Assets<Image>>,
    q_camera: Single<(&GlobalTransform, &Camera)>,
) {
    let Some(image) = images.get(&assets.field) else {
        error!("Could not get Asset");
        return;
    };

    let (camera_tranform, camera) = *q_camera;

    let (global_trans, mut trans) = obj.into_inner();

    let Some(viewport) = camera.logical_viewport_rect() else {
        warn!("Camera has no viewport!");
        return;
    };

    let a = match camera.viewport_to_world_2d(camera_tranform, viewport.min) {
        Ok(t) => t,
        Err(e) => {
            warn!("{:?}", e);
            return;
        }
    };
    let b = match camera.viewport_to_world_2d(camera_tranform, viewport.max) {
        Ok(t) => t,
        Err(e) => {
            warn!("{:?}", e);
            return;
        }
    };

    let view_world = Rect::from_corners(a, b);

    let board = Rect::from_center_size(
        global_trans.translation().xy(),
        (image.size() * settings.size).as_vec2() * global_trans.scale().xy(),
    );

    let border_edge = board.max + viewport.max * 0.05;

    let change = (view_world.max / border_edge).min_element();

    trans.scale *= Vec2::splat(change).extend(1.);
}
