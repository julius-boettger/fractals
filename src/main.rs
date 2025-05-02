mod koch_snowflake;
mod render;
mod utils;
mod vec2;

use pollster::block_on;

use render::render;
use utils::VertexFormat;

fn main() {
    env_logger::init();

    let mut koch_snowflake = koch_snowflake::KochSnowflake::new();
    let vertices = koch_snowflake.get_line_vertices(4);

    block_on(render(vertices, VertexFormat::Lines));
}
