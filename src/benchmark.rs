use crate::rendering::vertex::{self, VertexFormat};
use crate::curves::Curves;

#[allow(clippy::needless_pass_by_value)]
pub fn run(iteration: usize, curve: Curves) {
    super::log_init("debug");
    let mut curve_instance = curve.new_instance();
    let vertex_format = curve_instance.vertex_format();

    log::info!("starting benchmark");
    let now = std::time::Instant::now();

    let unformatted_vertices = curve_instance.vertices(iteration - 1);
    let raw_vertices = match vertex_format {
        VertexFormat::Lines => &vertex::lines_as_triangles(unformatted_vertices, 0.005),
        VertexFormat::Triangles => unformatted_vertices,
    };
    vertex::index(raw_vertices);

    log::info!("completed benchmark in {:?}", now.elapsed());
}
