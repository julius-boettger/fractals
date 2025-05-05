pub mod koch_snowflake;

use crate::renderer::vertex::{Vertex, VertexFormat};

/// https://en.wikipedia.org/wiki/Fractal_curve
pub trait Curve {
    fn new() -> Self;
    fn vertex_format() -> VertexFormat;
    fn data(&self) -> &Vec<Vec<Vertex>>;
    fn next_iteration(&mut self);

    /// iteration 0 meaning initial state
    fn vertices(&mut self, iteration: usize) -> &Vec<Vertex> {
        while self.data().len() <= iteration {
            self.next_iteration();
        }

        let fractal_name = std::any::type_name::<Self>()
            .rsplit_once("::")
            .unwrap()
            .1;
        log::info!("computed iteration {} of {}", iteration + 1, fractal_name);

        &self.data()[iteration]
    }
}
