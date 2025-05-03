mod koch_snowflake;
mod fractals;
mod renderer;

use pollster::block_on;

use renderer::render;
use fractals::Fractal;
use koch_snowflake::KochSnowflake;

fn main() {
    env_logger::init();

    let mut koch_snowflake = KochSnowflake::new();
    let vertices = koch_snowflake.vertices(4);
    let vertex_format = KochSnowflake::vertex_format();

    block_on(render(vertices, vertex_format));
}
