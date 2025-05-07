use rayon::prelude::*;

use crate::curves::Curve;
use crate::renderer::vertex::{Vertex, VertexFormat, vec2::Vec2};

/// https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle
pub struct SierpinskiTriangle {
    data: Vec<Vec<Vertex>>
}

impl Curve for SierpinskiTriangle {
    fn new() -> Self {
        Self {
            data: vec![vec![
                Vertex::new(Vec2::new( 0.00,  0.75), 0),
                Vertex::new(Vec2::new(-0.75, -0.75), 0),
                Vertex::new(Vec2::new( 0.75, -0.75), 0),
            ]],
        }
    }

    fn vertex_format(&self) -> VertexFormat { VertexFormat::Triangles }
    
    fn     data(&    self) -> &    Vec<Vec<Vertex>> { &    self.data }
    fn mut_data(&mut self) -> &mut Vec<Vec<Vertex>> { &mut self.data }

    fn next_iteration(&self, last_vertices: &Vec<Vertex>, iteration: u32) -> Vec<Vertex> {
        last_vertices.par_chunks(3).map(|triangle| {
            let top = triangle[0].position;
            let left = triangle[1].position;
            let right = triangle[2].position;

            let top_left = top + ((left - top) / 2.);
            let top_right = top + ((right - top) / 2.);
            let bottom = left + ((right - left) / 2.);

            [
                // top triangle
                Vertex::new(top,       iteration),
                Vertex::new(top_left,  iteration),
                Vertex::new(top_right, iteration),

                // left triangle
                Vertex::new(top_left,       iteration),
                Vertex::new(left,  iteration),
                Vertex::new(bottom, iteration),

                // right triangle
                Vertex::new(top_right,       iteration),
                Vertex::new(bottom,  iteration),
                Vertex::new(right, iteration),
            ]
        }).flatten().collect()
    }
}
