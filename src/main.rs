mod koch_snowflake;
mod render;
mod utils;
mod vec2;

use pollster::block_on;

use render::render;
use koch_snowflake::KochSnowflake;

fn main() {
    env_logger::init();

    let mut koch_snowflake = KochSnowflake::new();
    let vertices = koch_snowflake.vertices(4);
    let vertex_format = KochSnowflake::vertex_format();

    block_on(render(vertices, vertex_format));
}
