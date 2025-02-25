#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d
#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_pbr::utils::coords_to_viewport_uv

struct GridMaterial {
    squares: vec2<f32>,
  }

@group(2) @binding(0) var<uniform> material: GridMaterial;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let coords = mesh.uv * material.squares;
    let derivate = fwidth(coords);
    let grid = abs(fract(coords - 0.5) - 0.5) / derivate;
    let line = min(grid.x, grid.y);
    let color = vec4f(0.0, 0.0, 0.0, 1.0 - min(line, 1.0));

    return color;
}
