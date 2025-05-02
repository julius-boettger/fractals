use crate::{Vertex, Color, utils};

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
                Vertex { position: [-0.5, -0.5], color: Self::COLOR },
                Vertex { position: [ 0.5, -0.5], color: Self::COLOR },

                Vertex { position: [-0.5, -0.5], color: Self::COLOR },
                Vertex { position: [ 0.0,  0.5], color: Self::COLOR },

                Vertex { position: [ 0.5, -0.5], color: Self::COLOR },
                Vertex { position: [ 0.0,  0.5], color: Self::COLOR },
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
        log::info!("computing iteration {}", self.data.len());

        let current = self.data.last().unwrap();

        let next = current
            .chunks(2)
            .map(|line| {
                let (a, b) = (line[0].position, line[1].position);

                // pointing from a to b
                let vector = [b[0] - a[0],
                                        b[1] - a[1]];

                let third_a = [a[0] + ( vector[0] / Self::WIDTH_DIVISOR),
                                         a[1] + ( vector[1] / Self::WIDTH_DIVISOR)];

                let third_b = [b[0] + (-vector[0] / Self::WIDTH_DIVISOR),
                                         b[1] + (-vector[1] / Self::WIDTH_DIVISOR)];

                let top = {
                    let away_orthogonal = utils::away_orthogonal(vector, a);
                    // reduce length
                    let away_orthogonal = [away_orthogonal[0] / Self::HEIGHT_DIVISOR,
                                                     away_orthogonal[1] / Self::HEIGHT_DIVISOR];

                    [a[0] + (vector[0] / 2.) + away_orthogonal[0],
                     a[1] + (vector[1] / 2.) + away_orthogonal[1]]
                };

                [
                          a, third_a,
                    third_a,     top,
                        top, third_b,
                    third_b,       b,
                ]
            })
            .flatten()
            .map(|p| Vertex { position: p, color: Self::COLOR })
            .collect();

        self.data.push(next);
    }
}