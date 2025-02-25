use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
    sprite::{AlphaMode2d, Material2d},
};

const SHADER_PATH: &str = "shaders/grid.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
#[uniform(0, GridMaterialUniform)]
pub struct GridMaterial {
    pub squars: Vec2,
}

#[derive(Clone, Default, ShaderType)]
pub struct GridMaterialUniform {
    pub squars: Vec2,
}

impl AsBindGroupShaderType<GridMaterialUniform> for GridMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> GridMaterialUniform {
        GridMaterialUniform {
            squars: self.squars,
        }
    }
}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
