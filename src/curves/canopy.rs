use rayon::prelude::*;
use std::f32::consts::PI;

use crate::curves::Curve;
use crate::renderer::vertex::{Vertex, VertexFormat, vec2::Vec2};

/// https://en.wikipedia.org/wiki/Fractal_canopy
pub struct Canopy {
    data: Vec<Vec<Vertex>>,
    /// factor of PI
    left_angle: f32,
    /// factor of PI
    right_angle: f32,
}

impl Canopy {
    // adjust shape
    const LENGTH_FACTOR: f32 = 0.675;
    /// factor of PI
    const INITIAL_LEFT_ANGLE: f32 = 0.2;
    /// factor of PI
    const INITIAL_RIGHT_ANGLE: f32 = 0.35;
    /// when adjusting angle
    const ANGLE_INCREMENT: f32 = 0.05;
    const ANGLE_MIN: f32 = Self::ANGLE_INCREMENT;
    const ANGLE_MAX: f32 = 0.5;
}

impl Canopy {
    /// returns `true` if value was actually changed <br>
    /// `!increment == decrement` <br>
    /// `!left == right`
    pub fn change_angle(&mut self, increment: bool, left: bool) -> bool {
        let angle = match left {
            true => &mut self.left_angle,
            false => &mut self.right_angle,
        };

        *angle += match increment {
            true => Self::ANGLE_INCREMENT,
            false => -Self::ANGLE_INCREMENT,
        };

        let clamped_angle = angle.clamp(Self::ANGLE_MIN, Self::ANGLE_MAX);
        // changed in range
        let changed = *angle == clamped_angle;

        match changed {
            true => log::info!("set {} angle to {:.2}π", if left { "left" } else { "right" }, angle),
            // ensure range
            false => *angle = clamped_angle,
        }

        changed
    }
}

impl Curve for Canopy {
    fn new() -> Self {
        Self {
            left_angle: Self::INITIAL_LEFT_ANGLE,
            right_angle: Self::INITIAL_RIGHT_ANGLE,
            // always pointing counterclockwise to make rotation work later
            // in this case: always point top to bottom
            data: vec![vec![
                Vertex::new(Vec2::new(0., -0.25), 0),
                Vertex::new(Vec2::new(0., -0.75), 0),
            ]],
        }
    }

    fn vertex_format(&self) -> VertexFormat { VertexFormat::Lines }
    fn default_iteration(&self) -> usize { 11 }
    
    fn     data(&    self) -> &    Vec<Vec<Vertex>> { &    self.data }
    fn mut_data(&mut self) -> &mut Vec<Vec<Vertex>> { &mut self.data }

    fn next_iteration(&self, last_vertices: &[Vertex], iteration: u32) -> Vec<Vertex> {
        last_vertices
            .par_chunks(2)
            // just lines with highest iteration 
            .filter(|line| line[0].iteration == iteration - 1)
            .map(|line| {
                let (top, bottom) = (line[0].position, line[1].position);
                let top_iter = line[0].iteration;

                let bottom_to_top = top - bottom;

                let top_left = top + (bottom_to_top.rotate_ccw(PI * self.left_angle) * Self::LENGTH_FACTOR);
                let top_right = top + (bottom_to_top.rotate_cw(PI * self.right_angle) * Self::LENGTH_FACTOR);

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
