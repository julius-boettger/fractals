mod koch_snowflake;
mod render;
mod utils;
mod vec2;

use utils::VertexFormat;
use render::render;

fn main() {
    env_logger::init();

    let mut koch_snowflake = koch_snowflake::KochSnowflake::new();
    let vertices = koch_snowflake.get_line_vertices(4);

    pollster::block_on(render(vertices, VertexFormat::Lines));
}
