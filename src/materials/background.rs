use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
    window::WindowResized,
};

const BACKGROUND_SHADER_PATH: &str = "shaders/background.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[uniform(0)]
    pub(crate) start_color: LinearRgba,
    #[uniform(1)]
    pub(crate) end_color: LinearRgba,
    #[uniform(2)]
    pub(crate) position: f32,
    #[uniform(3)]
    pub(crate) size: f32,
    #[uniform(4)]
    pub(crate) angle: f32,
    pub(crate) alpha_mode: AlphaMode2d,
}

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
            alpha_mode: AlphaMode2d::Blend,
            position: 0.,
            size: 0.4,
            angle: 45.,
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

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        BACKGROUND_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha_mode
    }
}
