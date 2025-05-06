pub mod koch_snowflake;

use crate::renderer::vertex::{Vertex, VertexFormat};

/// https://en.wikipedia.org/wiki/Fractal_curve
pub trait Curve<Component: CurveComponent> {
    fn new() -> Self;
    fn vertex_format() -> VertexFormat;
    fn next_iteration(&self, last_components: &Vec<Component>, iteration: u32) -> Vec<Component>;

    /// one element for each iteration, where each element consists of
    /// the iterations components, and optionally their computed vertices
    fn     data(&    self) -> &    Vec<(Vec<Component>, Option<Vec<Vertex>>)>;
    fn mut_data(&mut self) -> &mut Vec<(Vec<Component>, Option<Vec<Vertex>>)>;

    /// iteration 0 meaning initial state
    fn vertices<'a>(&'a mut self, iteration: usize) -> &'a Vec<Vertex>
        where Component: 'a
    {
        // compute fractal iterations (if not done already)
        for current_iteration in self.data().len() ..= iteration {
            if current_iteration >= 10 {
                log::debug!("computing iteration {}", current_iteration);
            }

            let next_data = self.next_iteration(
                &self.data()[current_iteration - 1].0,
                current_iteration.try_into().unwrap()
            );

            self.mut_data().push((next_data, None));
        }

        // map components of iteration to vertices (if not done already)
        if self.data()[iteration].1 == None {
            let vertices = self.data()[iteration].0.iter()
                .map(|c| c.vertices())
                .flatten()
                .collect();

            self.mut_data()[iteration].1 = Some(vertices);
        };

        let fractal_name = std::any::type_name::<Self>()
            .rsplit_once("::")
            .unwrap()
            .1;
        log::info!("computed iteration {} of {}", iteration + 1, fractal_name);

        &self.data()[iteration].1.as_ref().unwrap()
    }
}

pub trait CurveComponent {
    fn vertices(&self) -> Vec<Vertex>;
}
