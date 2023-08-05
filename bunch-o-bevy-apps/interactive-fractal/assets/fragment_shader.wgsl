//#import bevy_sprite::mesh2d_view_bindings globals
#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput


struct MyMaterial {
    foo: vec4<f32>,
}

@group(1)
@binding(0)
var<uniform> foo: MyMaterial;

const MAX_STEPS = 200.0;

fn mandelbrot(i: vec2<f32>) -> f32 {
    var z = vec2(0.0, 0.0);
    for (var step = 0.0; step < MAX_STEPS; step = step + 1.0) {
        z = vec2(z[0] * z[0] - z[1] * z[1], 2.0 * z[0] * z[1]);
        z += i;
        if(z[0] * z[0] + z[1] * z[1] > 2.0) {
            return step;
        }
    }
    return 0.0;
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let x =  foo.foo[0] - foo.foo[2] * 0.5 + in.uv[0] * foo.foo[2];
    let y =  foo.foo[1] - foo.foo[3] * 0.5 + in.uv[1] * foo.foo[3];
    let steps = mandelbrot(vec2(x, y));
    let c = sqrt(steps) / sqrt(MAX_STEPS);
    return vec4<f32>(0.0, 0.3 * c, 0.1 + 0.9 * c , 1.0);
}
