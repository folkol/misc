#import bevy_sprite::mesh2d_view_bindings globals
#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let t = globals.time * 5.;
    let x =  in.uv[0] * 10.;
    let n = in.uv[0]*10. + in.uv[1]* 10.;
    let c = 0.15 + abs(0.7 * sin(t + x)) + 1. * sin(n);
    return vec4<f32>(1. - c, c, c, 1.0);
}