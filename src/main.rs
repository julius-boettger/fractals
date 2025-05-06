mod curves;
mod renderer;

use pollster::block_on;

use renderer::render;
use curves::Curve;

fn main() {
    // set up logger with default level if env var RUST_LOG is unset
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("error,fractals=info")
    ).init();

    type Curve = curves::koch_snowflake::KochSnowflake;
    const ITERATION: usize = 4;

    let mut curve = Curve::new();
    let vertices = curve.vertices(ITERATION);
    let vertex_format = Curve::vertex_format();

    block_on(render(vertices, vertex_format));
}
