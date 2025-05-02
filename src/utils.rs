use crate::{render::Vertex, vec2::Vec2};

/// RGB with values in \[0, 1\]
pub type Color = [f32; 3];

/// describes how an array of vertices should be interpreted
pub enum VertexFormat {
    /// groups of two to form lines
    Lines,
    /// groups of three to form triangles
    Triangles,
}

/// get unique elements of an array slice without hashing
pub fn unique<T: Clone + PartialEq>(slice: &[T]) -> Vec<T> {
    let mut result = vec![];
    for item in slice {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }
    result
}

/// transform ordered, partly duplicate vertices into unique vertices and indices 
pub fn index_vertices(vertices: &[Vertex]) -> (Vec<Vertex>, Vec<u16>) {
    let unique_vertices = unique(vertices);
    let indices = vertices.iter().map(|vertex| {
        unique_vertices.iter().position(|x| x == vertex).unwrap().try_into().unwrap()
    }).collect();
    (unique_vertices, indices)
}

pub fn lines_as_triangles(vertices: &[Vertex], line_width: f32) -> Vec<Vertex> {
    vertices.chunks(2).map(|line| {
        let (a, b) = (line[0].position, line[1].position);
        let (a_color, b_color) = (line[0].color, line[1].color);

        let a_to_b = b - a;

        // orthogonal to vector to make rectangle
        let offset1 = Vec2::new(-a_to_b.y,  a_to_b.x).set_len(line_width / 2.);
        let offset2 = Vec2::new( a_to_b.y, -a_to_b.x).set_len(line_width / 2.);

        // on same line as vector to make smoother ends
        let a_offset = (-a_to_b).set_len(line_width / 2.);
        let b_offset = ( a_to_b).set_len(line_width / 2.);

        [
            // line as rectangle of two triangles

            Vertex { position: a + offset1, color: a_color },
            Vertex { position: a + offset2, color: a_color },
            Vertex { position: b + offset1, color: b_color },

            Vertex { position: b + offset1, color: b_color },
            Vertex { position: a + offset2, color: a_color },
            Vertex { position: b + offset2, color: b_color },

            // smoother ends of line

            Vertex { position: a +  offset1, color: a_color },
            Vertex { position: a + a_offset, color: a_color },
            Vertex { position: a +  offset2, color: a_color },

            Vertex { position: b +  offset1, color: b_color },
            Vertex { position: b +  offset2, color: b_color },
            Vertex { position: b + b_offset, color: b_color },
        ]
    }).flatten().collect()
}
