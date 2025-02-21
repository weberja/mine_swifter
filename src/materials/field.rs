use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

const SHADER_PATH: &str = "shaders/field.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FieldMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    pub texture: Handle<Image>,
    pub alpha_mode: AlphaMode2d,
    #[uniform(2)]
    pub index: u32,
}

impl Material2d for FieldMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha_mode
    }
}
