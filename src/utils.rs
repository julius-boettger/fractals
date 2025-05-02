use super::Vertex;

/// 2D position with X and Y in [-1, 1]
pub type Position = [f32; 2];
/// RGB with values in [0, 1]
pub type Color = [f32; 3];

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

pub fn set_length(vector: Position, new_length: f32) -> Position {
    let current_length: f32 = (vector[0].powf(2.) + vector[1].powf(2.)).sqrt();

    if current_length == new_length {
        return vector;
    }

    let normalized = [vector[0] / current_length, vector[1] / current_length];
    [normalized[0] * new_length, normalized[1] * new_length]
}

pub fn lines_as_triangles(vertices: &[Vertex], line_width: f32) -> Vec<Vertex> {
    vertices.chunks(2).map(|line| {
        let a = line[0];
        let b = line[1];
        let a_pos = a.position;
        let b_pos = b.position;

        // pointing from a to b
        let vector = [b_pos[0] - a_pos[0], b_pos[1] - a_pos[1]];

        // orthogonal to vector to make rectangle
        let offset1 = set_length([-vector[1],  vector[0]], line_width / 2.);
        let offset2 = set_length([ vector[1], -vector[0]], line_width / 2.);

        // on same line as vector to make smoother ends
        let a_offset = set_length([-vector[0], -vector[1]], line_width / 2.);
        let b_offset = set_length(vector, line_width / 2.);

        [
            // line as rectangle of two triangles

            Vertex { position: [a_pos[0] + offset1[0], a_pos[1] + offset1[1]], color: a.color },
            Vertex { position: [a_pos[0] + offset2[0], a_pos[1] + offset2[1]], color: a.color },
            Vertex { position: [b_pos[0] + offset1[0], b_pos[1] + offset1[1]], color: b.color },

            Vertex { position: [b_pos[0] + offset1[0], b_pos[1] + offset1[1]], color: b.color },
            Vertex { position: [a_pos[0] + offset2[0], a_pos[1] + offset2[1]], color: a.color },
            Vertex { position: [b_pos[0] + offset2[0], b_pos[1] + offset2[1]], color: b.color },

            // smoother ends of line

            Vertex { position: [a_pos[0] +  offset1[0], a_pos[1] +  offset1[1]], color: a.color },
            Vertex { position: [a_pos[0] + a_offset[0], a_pos[1] + a_offset[1]], color: a.color },
            Vertex { position: [a_pos[0] +  offset2[0], a_pos[1] +  offset2[1]], color: a.color },

            Vertex { position: [b_pos[0] +  offset1[0], b_pos[1] +  offset1[1]], color: b.color },
            Vertex { position: [b_pos[0] +  offset2[0], b_pos[1] +  offset2[1]], color: b.color },
            Vertex { position: [b_pos[0] + b_offset[0], b_pos[1] + b_offset[1]], color: b.color },
        ]
    }).flatten().collect()
}

pub fn dot_product(vector1: Position, vector2: Position) -> f32 {
    vector1[0]*vector2[0] + vector1[1]*vector2[1]
}

/// returns orthogonal of given vector that points away from the origin when placed at the given tail.
/// the orthogonal has same length as the given vector.
pub fn away_orthogonal(vector: Position, tail: Position) -> Position {
    // we don't know if this points away from or towards the origin yet
    let mut orthogonal = [-vector[1], vector[0]];

    // if it points towards the origin, turn it around
    if dot_product(tail, orthogonal) < 0. {
        orthogonal = [-orthogonal[0], -orthogonal[1]];
    }

    orthogonal
}
