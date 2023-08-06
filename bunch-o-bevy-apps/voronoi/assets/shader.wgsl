#import bevy_pbr::mesh_vertex_output MeshVertexOutput

@group(1) @binding(0)
var<uniform> points: array<vec4<f32>, 100>;
@group(1) @binding(1)
var<uniform> colors: array<vec4<f32>, 100>;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    var min_d  = 1337.0;
    var neighbour = 0;
    for (var i = 0; i < 20; i++) {
        let d = distance(mesh.uv, points[i].xy);
        if(d < min_d) {
            neighbour = i;
            min_d = d;
        }
    }
    let intensity = pow(1.0 - min_d, 10.5);
    let color = colors[neighbour] * intensity;
    return vec4(color.xyz, 1.0);
}