pub mod events;

use bevy::{
    input::{gestures::PinchGesture, mouse::MouseWheel},
    prelude::*,
};
use events::{ResetView, Zoom};

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

fn reset_view(trigger: Trigger<ResetView>, mut obj: Single<&mut Transform, With<ZoomableObject>>) {}
