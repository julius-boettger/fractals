use rayon::prelude::*;

use crate::curves::{Curve, CurveComponent};
use crate::renderer::vertex::{Vertex, VertexFormat, vec2::Vec2};

/// https://en.wikipedia.org/wiki/Koch_snowflake
pub struct KochSnowflake {
    data: Vec<(Vec<Line>, Option<Vec<Vertex>>)>
}

pub struct Line {
    a: Vertex,
    b: Vertex,
    /// direction to point next extension of this line in  
    top: Vec2,
}

impl Line {
    fn new(a: Vertex, b: Vertex, top: Vec2) -> Self {
        Self { a, b, top }
    }
}

impl CurveComponent for Line {
    fn vertices(&self) -> Vec<Vertex> {
        vec![self.a, self.b]
    }
}

impl KochSnowflake {
    // adjust shape
    const WIDTH_DIVISOR: f32 = 3.;
    const HEIGHT_DIVISOR: f32 = 4.;
}

impl Curve<Line> for KochSnowflake {
    fn new() -> Self {
        Self {
            data: vec![(
                // actual lines
                vec![
                    Vertex::new(Vec2::new(-0.75, -0.45), 0),
                    Vertex::new(Vec2::new( 0.75, -0.45), 0),

                    Vertex::new(Vec2::new(-0.75, -0.45), 0),
                    Vertex::new(Vec2::new( 0.00,  0.75), 0),

                    Vertex::new(Vec2::new( 0.75, -0.45), 0),
                    Vertex::new(Vec2::new( 0.00,  0.75), 0),
                ]
                // add top
                .chunks(2).map(|line| {
                    let a = line[0];
                    let b = line[1];
                    let a_to_b = b.position - a.position;
                    Line::new(a, b, a_to_b.away_orthogonal(a.position))
                }).collect(),
                None
            )],
        }
    }

    fn vertex_format() -> VertexFormat { VertexFormat::Lines }
    
    fn     data(&    self) -> &    Vec<(Vec<Line>, Option<Vec<Vertex>>)> { &    self.data }
    fn mut_data(&mut self) -> &mut Vec<(Vec<Line>, Option<Vec<Vertex>>)> { &mut self.data }
    
    fn next_iteration(&self, last_lines: &Vec<Line>, iteration: u32) -> Vec<Line> {
        last_lines.par_iter().map(|line| {
            let (a, b) = (line.a.position, line.b.position);
            let (a_iter, b_iter) = (line.a.iteration, line.b.iteration);

            let a_to_b = b - a;

            let third_a = a + ( a_to_b / Self::WIDTH_DIVISOR);
            let third_b = b + (-a_to_b / Self::WIDTH_DIVISOR);

            let top = {
                // with reduced length
                let away_orthogonal = a_to_b.away_orthogonal(a) / Self::HEIGHT_DIVISOR;

                a + (a_to_b / 2.) + away_orthogonal
            };

            [
                Line::new(
                    Vertex::new(a,       a_iter),
                    Vertex::new(third_a, a_iter),
                    Vec2::new(0., 0.),
                ),

                Line::new(
                    Vertex::new(third_a, a_iter),
                    Vertex::new(top,     iteration),
                    Vec2::new(0., 0.),
                ),

                Line::new(
                    Vertex::new(top,     iteration),
                    Vertex::new(third_b, b_iter),
                    Vec2::new(0., 0.),
                ),

                Line::new(
                    Vertex::new(third_b, b_iter),
                    Vertex::new(b,       b_iter),
                    Vec2::new(0., 0.),
                ),
            ]
        }).flatten().collect()
    }
}