struct Globals {
    max_iteration: u32,
};
@group(0) @binding(0)
var<uniform> globals: Globals;

// matches render::Vertex struct in rust code
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) iteration: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var x = f32(in.iteration) / f32(globals.max_iteration);
    // overwrite value if there was division by 0
    if globals.max_iteration == 0u {
        x = 0.0;
    }

    out.color = vec3<f32>(0.0, x, 1.0 - x);

    out.clip_position = vec4<f32>(in.position, 0.0, 1.0);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
