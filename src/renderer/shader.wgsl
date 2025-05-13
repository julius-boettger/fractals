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

// from rusts f32::consts::PI
const PI: f32 = 3.1415927410125732421875;

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
    let pos = in.position;

    // in range [0, 1]
    var scaled_iteration = f32(in.iteration) / f32(globals.max_iteration);
    // overwrite value if there was division by 0
    if globals.max_iteration == 0 {
        scaled_iteration = 1;
    }

    // in range [-PI, PI], 0 means pointing up (positive y)
    let angle = atan2(pos.x, pos.y);
    // in range [0, 1]
    let scaled_angle = (angle + PI) / (2 * PI);

    //let distance = sqrt(pow(pos.x, 2) + pow(pos.y, 2));
    // in range [0, 1]
    //let scaled_distance = distance / sqrt(2);

    var out: VertexOutput;

    ////////// position //////////

    out.position = vec4(pos, 0, 1);

    ////////// color //////////
    // compute color here, as moving any of this logic
    // to the fragment shader changes the visuals
    // in an unwanted way because of interpolation

    // based on angle
    var h = scaled_angle;
    // rotating clockwise (except on lowest iteration)
    if scaled_iteration != 0 {
        h += -globals.animation_value + 1;
    }
    // offset for iteration
    h += -scaled_iteration * 0.3 + 0.3;
    // ensure range [0, 1]
    h %= 1;

    // saturation
    var s = scale_to(scaled_iteration, 0.8, 1);

    // luminance
    let l = scale_to(scaled_iteration, 0.2, 0.55);

    out.color = hsl_to_rgb(h, s, l);

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.color, 1);
}
