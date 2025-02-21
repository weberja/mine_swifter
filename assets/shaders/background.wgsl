#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0) var<uniform> start_color: vec4<f32>;
@group(2) @binding(1) var<uniform> end_color: vec4<f32>;
@group(2) @binding(2) var<uniform> position: f32;
@group(2) @binding(3) var<uniform> size: f32;
@group(2) @binding(4) var<uniform> angle: f32;

//dev/fn hash(p: vec2f) -> vec2f {
   // let p = vec2f(dot(p, vec2f(2127.1, 81.17)), dot(1269.5, 283.37));
    //return fract(sin(p) * 43758.5453);
//}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time_shift = 0.5 + 0.5 * sin(globals.time / 4.);
    return vec4f(mesh.uv.y * time_shift, mesh.uv.x, 0.5, 1.0);
}
