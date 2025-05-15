pub mod canopy;
pub mod koch_snowflake;
pub mod sierpinski_triangle;

use crate::renderer::vertex::{Vertex, VertexFormat};

pub const INITIAL_ITERATION: usize = 4;

/// slice of functions to get new curve of each implementing struct.
/// first element will be the initial curve rendered.
pub const CURVES: &[fn() -> Box<dyn Curve>] = &[
    || Box::new(canopy::Canopy::new()),
    || Box::new(koch_snowflake::KochSnowflake::new()),
    || Box::new(sierpinski_triangle::SierpinskiTriangle::new()),
];

/// https://en.wikipedia.org/wiki/Fractal_curve
pub trait Curve {
    fn new() -> Self
        where Self: Sized; // for dyn-compatability

    // technically doesnt need to be a method, but is easier to work with
    fn vertex_format(&self) -> VertexFormat;

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
