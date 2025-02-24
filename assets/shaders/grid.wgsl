#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d
#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_pbr::utils::coords_to_viewport_uv


@group(2) @binding(0) var<uniform> lane_width: f32;
@group(2) @binding(1) var<uniform> pitch: vec2f;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    if (mesh.position.x - 32.) % pitch.x < 1 || (mesh.position.y - 8.) % pitch.y < 1 {
        return vec4f(0., 0., 0., 0.75);
    } else {
        return vec4f(0., 0., 0., 0.);
    }
}
