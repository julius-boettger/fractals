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
    /// assumes `top_direction` is normalized
    fn new(a: Vertex, b: Vertex, top_direction: Vec2, height: f32) -> Self {
        let a_to_b = b.position - a.position;
        let middle = a.position + (a_to_b / 2.);
        let top = middle + (top_direction * height);
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
                    Line::new(
                        a, b,
                        a.position.away_orthogonal_to(b.position),
                        (b.position - a.position).len() / Self::HEIGHT_DIVISOR,
                    )
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
            let top = line.top;

            let a_to_b = b - a;

            let third_a = a + ( a_to_b / Self::WIDTH_DIVISOR);
            let third_b = b + (-a_to_b / Self::WIDTH_DIVISOR);

            [
                Line::new(
                    Vertex::new(a,       a_iter),
                    Vertex::new(third_a, a_iter),
                    Vec2::new(0., 0.),
                    0.,
                ),

                Line::new(
                    Vertex::new(third_a, a_iter),
                    Vertex::new(top,     iteration),
                    Vec2::new(0., 0.),
                    0.,
                ),

                Line::new(
                    Vertex::new(top,     iteration),
                    Vertex::new(third_b, b_iter),
                    Vec2::new(0., 0.),
                    0.,
                ),

                Line::new(
                    Vertex::new(third_b, b_iter),
                    Vertex::new(b,       b_iter),
                    Vec2::new(0., 0.),
                    0.,
                ),
            ]
        }).flatten().collect()
    }
}