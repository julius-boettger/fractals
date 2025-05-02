use crate::{render::Vertex, vec2::Vec2};

/// describes how an array of vertices should be interpreted
pub enum VertexFormat {
    /// groups of two to form lines
    Lines,
    /// groups of three to form triangles
    #[allow(dead_code)]
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
        let (a_iter, b_iter) = (line[0].iteration, line[1].iteration);

        let a_to_b = b - a;

        // orthogonal to vector to make rectangle
        let offset1 = Vec2::new(-a_to_b.y,  a_to_b.x).set_len(line_width / 2.);
        let offset2 = Vec2::new( a_to_b.y, -a_to_b.x).set_len(line_width / 2.);

        // on same line as vector to make smoother ends
        let a_offset = (-a_to_b).set_len(line_width / 2.);
        let b_offset = ( a_to_b).set_len(line_width / 2.);

        [
            // line as rectangle of two triangles

            Vertex::new(a + offset1, a_iter),
            Vertex::new(a + offset2, a_iter),
            Vertex::new(b + offset1, b_iter),

            Vertex::new(b + offset1, b_iter),
            Vertex::new(a + offset2, a_iter),
            Vertex::new(b + offset2, b_iter),

            // smoother ends of line

            Vertex::new(a +  offset1, a_iter),
            Vertex::new(a + a_offset, a_iter),
            Vertex::new(a +  offset2, a_iter),

            Vertex::new(b +  offset1, b_iter),
            Vertex::new(b +  offset2, b_iter),
            Vertex::new(b + b_offset, b_iter),
        ]
    }).flatten().collect()
}
