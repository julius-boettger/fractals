use crate::{render::Vertex, utils::Color, vec2::Vec2};

pub struct KochSnowflake {
    data: Vec<Vec<Vertex>>
}

impl KochSnowflake {

    const COLOR: Color = [0.0, 1.0, 1.0];

    // adjust shape
    const WIDTH_DIVISOR: f32 = 3.;
    const HEIGHT_DIVISOR: f32 = 4.;

    pub fn new() -> Self {
        Self {
            data: vec![vec![
                Vertex::new(Vec2::new(-0.5, -0.5), Self::COLOR),
                Vertex::new(Vec2::new( 0.5, -0.5), Self::COLOR),

                Vertex::new(Vec2::new(-0.5, -0.5), Self::COLOR),
                Vertex::new(Vec2::new( 0.0,  0.5), Self::COLOR),

                Vertex::new(Vec2::new( 0.5, -0.5), Self::COLOR),
                Vertex::new(Vec2::new( 0.0,  0.5), Self::COLOR),
            ]],
        }
    }

    pub fn get_line_vertices(&mut self, iteration: usize) -> &Vec<Vertex> {
        while self.data.len() <= iteration {
            self.next_iteration();
        }

        &self.data[iteration]
    }

    fn next_iteration(&mut self) {
        log::debug!("computing iteration {}", self.data.len());

        let current = self.data.last().unwrap();

        let next = current
            .chunks(2)
            .map(|line| {
                let (a, b) = (line[0].position, line[1].position);

                let a_to_b = b - a;

                let third_a = a + ( a_to_b / Self::WIDTH_DIVISOR);
                let third_b = b + (-a_to_b / Self::WIDTH_DIVISOR);

                let top = {
                    // with reduced length
                    let away_orthogonal = a_to_b.away_orthogonal(a) / Self::HEIGHT_DIVISOR;

                    a + (a_to_b / 2.) + away_orthogonal
                };

                [
                          a, third_a,
                    third_a,     top,
                        top, third_b,
                    third_b,       b,
                ]
            })
            .flatten()
            .map(|p| Vertex::new(p, Self::COLOR))
            .collect();

        self.data.push(next);
    }
}