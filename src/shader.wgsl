struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    switch in_vertex_index {
        case 0u: { out.clip_position = vec4( 0.0,  0.5, 0.0, 1.0); }
        case 1u: { out.clip_position = vec4( 0.5, -0.5, 0.0, 1.0); }
        case 2u: { out.clip_position = vec4(-0.5, -0.5, 0.0, 1.0); }
        default: {}
    }
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
