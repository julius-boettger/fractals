pub mod koch_snowflake;

use crate::renderer::vertex::{Vertex, VertexFormat};

/// https://en.wikipedia.org/wiki/Fractal_curve
pub trait Curve {
    fn new() -> Self;
    fn vertex_format() -> VertexFormat;
    fn next_iteration(&self, last_vertices: &Vec<Vertex>, iteration: u32) -> Vec<Vertex>;

    /// one element for each iteration
    fn     data(&    self) -> &    Vec<Vec<Vertex>>;
    fn mut_data(&mut self) -> &mut Vec<Vec<Vertex>>;

    /// iteration 0 meaning initial state
    fn vertices(&mut self, iteration: usize) -> &Vec<Vertex> {
        // compute fractal iterations (if not done already)
        for current_iteration in self.data().len() ..= iteration {
            if current_iteration >= 9 {
                log::debug!("computing iteration {}", current_iteration + 1);
            }

            let vertices = self.next_iteration(
                &self.data()[current_iteration - 1],
                current_iteration.try_into().unwrap()
            );

            self.mut_data().push(vertices);
        }

        let fractal_name = std::any::type_name::<Self>()
            .rsplit_once("::")
            .unwrap()
            .1;
        log::info!("computed iteration {} of {}", iteration + 1, fractal_name);

        &self.data()[iteration]
    }
}
