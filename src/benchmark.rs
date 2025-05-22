use crate::rendering::vertex;
use crate::curves::{Curve, koch_snowflake::KochSnowflake};

/// can consume up to 7 GB of memory
pub fn run() {
    super::log_init("debug");
    let mut koch_snowflake = KochSnowflake::new();

    log::info!("starting benchmark");
    let now = std::time::Instant::now();

    let line_vertices = koch_snowflake.vertices(10);
    let raw_vertices = vertex::lines_as_triangles(line_vertices, 0.005);
    vertex::index(&raw_vertices);

    log::info!("completed benchmark in {:?}", now.elapsed());
}
