use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    sprite::{AlphaMode2d, Material2d},
};

const SHADER_PATH: &str = "shaders/field.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[uniform(2, FieldMaterialUniform)]
pub struct FieldMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    pub texture: Handle<Image>,
    pub alpha_mode: AlphaMode2d,
    pub index: u32,
}

#[derive(Clone, Default, ShaderType)]
pub struct FieldMaterialUniform {
    pub index: u32,
    pub _padding: UVec3,
}

impl AsBindGroupShaderType<FieldMaterialUniform> for FieldMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &bevy::render::render_asset::RenderAssets<bevy::render::texture::GpuImage>,
    ) -> FieldMaterialUniform {
        FieldMaterialUniform {
            index: self.index,
            _padding: UVec3::default(),
        }
    }
    // add code here
}

impl Material2d for FieldMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha_mode
    }
}
