use rayon::prelude::*;

use crate::curves::Curve;
use crate::renderer::vertex::{Vertex, VertexFormat, vec2::Vec2};

/// https://en.wikipedia.org/wiki/Koch_snowflake
pub struct KochSnowflake {
    data: Vec<Vec<Vertex>>
}

impl KochSnowflake {
    // adjust shape
    const WIDTH_DIVISOR: f32 = 3.;
    const HEIGHT_DIVISOR: f32 = 4.;
}

impl Curve for KochSnowflake {
    fn new() -> Self {
        Self {
            data: vec![vec![
                Vertex::new(Vec2::new(-0.75, -0.45), 0),
                Vertex::new(Vec2::new( 0.75, -0.45), 0),

                Vertex::new(Vec2::new(-0.75, -0.45), 0),
                Vertex::new(Vec2::new( 0.00,  0.75), 0),

                Vertex::new(Vec2::new( 0.75, -0.45), 0),
                Vertex::new(Vec2::new( 0.00,  0.75), 0),
            ]],
        }
    }

    fn vertex_format() -> VertexFormat { VertexFormat::Lines }
    
    fn data(&self) -> &Vec<Vec<Vertex>> {
        &self.data
    }

    fn next_iteration(&mut self) {
        let last_vertices = self.data.last().unwrap();

        let iteration = self.data.len().try_into().unwrap();
        if iteration >= 10 {
            log::debug!("computing iteration {}", iteration);
        }

        let vertices = last_vertices
            .par_chunks(2)
            .map(|line| {
                let (a, b) = (line[0].position, line[1].position);
                let (a_iter, b_iter) = (line[0].iteration, line[1].iteration);

                let a_to_b = b - a;

                let third_a = a + ( a_to_b / Self::WIDTH_DIVISOR);
                let third_b = b + (-a_to_b / Self::WIDTH_DIVISOR);

                let top = {
                    // with reduced length
                    let away_orthogonal = a_to_b.away_orthogonal(a) / Self::HEIGHT_DIVISOR;

                    a + (a_to_b / 2.) + away_orthogonal
                };

                [
                    Vertex::new(a,       a_iter),
                    Vertex::new(third_a, a_iter),

                    Vertex::new(third_a, a_iter),
                    Vertex::new(top,     iteration),

                    Vertex::new(top,     iteration),
                    Vertex::new(third_b, b_iter),

                    Vertex::new(third_b, b_iter),
                    Vertex::new(b,       b_iter),
                ]
            })
            .flatten()
            .collect();

        self.data.push(vertices);
    }
}