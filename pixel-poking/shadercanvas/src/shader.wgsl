struct VertexOutput {
    // gl_Position / clip coordinates
    // framebuffer space (e.g. 0-800, 0-600, 0 is top)
    // https://gpuweb.github.io/gpuweb/#coordinate-systems
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(
    // https://www.w3.org/TR/WGSL/#vertex-index-builtin-value
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// @location(0) <-- first color target
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}