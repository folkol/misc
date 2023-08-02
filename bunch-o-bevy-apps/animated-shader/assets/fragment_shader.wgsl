#import bevy_sprite::mesh2d_view_bindings globals
#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let t = globals.time * 5.;
    let x =  in.uv[0] * 10.;
    let c = 0.15 + abs(0.7 * sin(t + x));
    return vec4<f32>(1. - c, c, c, 1.0);
}