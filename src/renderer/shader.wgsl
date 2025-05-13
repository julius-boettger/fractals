struct Globals {
    max_iteration: u32,
    animation_value: f32,
};
@group(0) @binding(0)
var<uniform> globals: Globals;

// matches render::Vertex struct in rust code
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) iteration: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) scaled_iteration: f32,
    @location(1) color: vec3<f32>,
};

// all input and output values in range [0, 1]
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    let a = s * min(l, 1 - l);
    return vec3(
        hsl_to_rgb_helper(0, a, h, s, l),
        hsl_to_rgb_helper(8, a, h, s, l),
        hsl_to_rgb_helper(4, a, h, s, l),
    );
}   
fn hsl_to_rgb_helper(n: f32, a: f32, h: f32, s: f32, l: f32) -> f32 {
    let k = (n + (h * 12)) % 12;
    return l - (a * max(min(min(k - 3, 9 - k), 1), -1));
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4(in.position, 0, 1);

    out.scaled_iteration = f32(in.iteration) / f32(globals.max_iteration);
    // overwrite value if there was division by 0
    if globals.max_iteration == 0 {
        out.scaled_iteration = 1;
    }

    out.color = hsl_to_rgb(globals.animation_value, 1, 0.5);

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.color, 1);
}
