struct Globals {
    max_iteration: u32,
    animation_value: f32,
};
@group(0) @binding(0)
var<uniform> globals: Globals;

// matches renderer::vertex::Vertex struct in rust code
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) iteration: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
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
    return l - (a * clamp(min(k - 3, 9 - k), -1, 1));
}

// value in range [0, 1]
fn scale_to(value: f32, min: f32, max: f32) -> f32 {
    return (value * (max - min)) + min;
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4(in.position, 0, 1);

    // in range [0, 1]
    var scaled_iteration = f32(in.iteration) / f32(globals.max_iteration);
    // overwrite value if there was division by 0
    if globals.max_iteration == 0 {
        scaled_iteration = 1;
    }

    // in range [0, 1]
    var h = (in.position.y + 1) / 2;
    // color going upwards
    h -= globals.animation_value;
    // make sure it is still in range
    if h < 0 { h += 1; }

    // limited saturation based on iteration
    var s = scale_to(scaled_iteration, 0.8, 1);

    // limited luminance based on iteration
    let l = scale_to(scaled_iteration, 0.3, 0.55);

    out.color = hsl_to_rgb(h, s, l);

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.color, 1);
}
