use crate::renderer::vertex::{Vertex, VertexFormat};

pub trait Fractal {
    fn new() -> Self;
    fn vertex_format() -> VertexFormat;
    fn data(&self) -> &Vec<Vec<Vertex>>;
    fn next_iteration(&mut self);

    /// iteration 0 meaning initial state
    fn vertices(&mut self, iteration: usize) -> &Vec<Vertex> {
        while self.data().len() <= iteration {
            self.next_iteration();
        }

        &self.data()[iteration]
    }
}
