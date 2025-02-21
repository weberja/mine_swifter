#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d
#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_pbr::utils::coords_to_viewport_uv

const aspect: f32 = 1.0;
const alpha: f32 = 0.5;

@group(2) @binding(0) var texture: texture_2d_array<f32>;
@group(2) @binding(1) var texture_sampler: sampler;
@group(2) @binding(2) var<uniform> index: u32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, mesh.uv, index) * vec4f(1.0, 1.0, 1.0, 0.5);
}
