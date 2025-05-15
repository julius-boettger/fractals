use rayon::prelude::*;
use std::f32::consts::PI;

use crate::curves::Curve;
use crate::renderer::vertex::{Vertex, VertexFormat, vec2::Vec2};

/// https://en.wikipedia.org/wiki/Fractal_canopy
pub struct Canopy {
    data: Vec<Vec<Vertex>>
}

impl Canopy {
    // adjust shape
    const BRANCH_LENGTH_FACTOR: f32 = 0.75;
    /// in radians
    const LEFT_BRANCH_ANGLE: f32 = (2. * PI) / 11.;
    /// in radians
    const RIGHT_BRANCH_ANGLE: f32 = Self::LEFT_BRANCH_ANGLE;
}

impl Curve for Canopy {
    fn new() -> Self {
        Self {
            // always pointing counterclockwise to make rotation work later
            // in this case: always point top to bottom
            data: vec![vec![
                Vertex::new(Vec2::new(0., -0.25), 0),
                Vertex::new(Vec2::new(0., -0.75), 0),
            ]],
        }
    }

    fn vertex_format(&self) -> VertexFormat { VertexFormat::Lines }
    
    fn     data(&    self) -> &    Vec<Vec<Vertex>> { &    self.data }
    fn mut_data(&mut self) -> &mut Vec<Vec<Vertex>> { &mut self.data }

    fn next_iteration(&self, last_vertices: &Vec<Vertex>, iteration: u32) -> Vec<Vertex> {
        last_vertices
            .par_chunks(2)
            // just lines with highest iteration 
            .filter(|line| line[0].iteration == iteration - 1)
            .map(|line| {
                let (top, bottom) = (line[0].position, line[1].position);
                let top_iter = line[0].iteration;

                let bottom_to_top = top - bottom;

                let top_left = top + (bottom_to_top.rotate_ccw(Self::LEFT_BRANCH_ANGLE) * Self::BRANCH_LENGTH_FACTOR);
                let top_right = top + (bottom_to_top.rotate_cw(Self::RIGHT_BRANCH_ANGLE) * Self::BRANCH_LENGTH_FACTOR);

                [
                    Vertex::new(top_left, iteration),
                    Vertex::new(top, top_iter),

                    Vertex::new(top_right, iteration),
                    Vertex::new(top, top_iter),
                ]
            })
            .flatten()
            // add last vertices
            .chain(last_vertices.par_chunks(2).flatten().cloned())
            .collect()
    }
}
