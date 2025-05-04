mod fractals;
mod renderer;

use pollster::block_on;

use renderer::render;
use fractals::{Fractal, koch_snowflake::KochSnowflake};

fn main() {
    // set up logger with default level if env var RUST_LOG is unset
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("error,fractals=info")
    ).init();

    let mut koch_snowflake = KochSnowflake::new();
    let vertices = koch_snowflake.vertices(4);
    let vertex_format = KochSnowflake::vertex_format();

    block_on(render(vertices, vertex_format));
}
