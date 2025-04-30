use super::Vertex;

/// 2D position with X and Y in [-1, 1]
pub type Position = [f32; 2];
/// RGB with values in [0, 1]
pub type Color = [f32; 3];

/// transform ordered, partly duplicate vertices into unique vertices and indices 
pub fn index_vertices(vertices: &[Vertex]) -> (Vec<Vertex>, Vec<u16>) {
    let unique_vertices = unique(vertices);
    let indices = vertices.iter().map(|vertex| {
        unique_vertices.iter().position(|x| x == vertex).unwrap().try_into().unwrap()
    }).collect();
    (unique_vertices, indices)
}

/// get unique elements of an iterator without hashing
fn unique<T: Clone + PartialEq>(slice: &[T]) -> Vec<T> {
    let mut result = vec![];
    for item in slice {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }
    result
}

pub fn lines_as_triangles(vertices: &[Vertex], line_width: f32) -> Vec<Vertex> {
    vertices.chunks(2).map(|line| {
        let a = line[0];
        let b = line[1];
        let a_pos = a.position;
        let b_pos = b.position;

        // pointing from a to b
        let vector = [b_pos[0] - a_pos[0], b_pos[1] - a_pos[1]];

        // orthogonal to vector
        let offset1 = set_length([-vector[1],  vector[0]], line_width / 2.);
        let offset2 = set_length([ vector[1], -vector[0]], line_width / 2.);

        // i have no idea if these triangles are consistently clockwise, counterclockwise, or just inconsistent
        vec![
            Vertex { position: [a_pos[0] + offset1[0], a_pos[1] + offset1[1]], color: a.color },
            Vertex { position: [a_pos[0] + offset2[0], a_pos[1] + offset2[1]], color: a.color },
            Vertex { position: [b_pos[0] + offset1[0], b_pos[1] + offset1[1]], color: b.color },

            Vertex { position: [b_pos[0] + offset1[0], b_pos[1] + offset1[1]], color: b.color },
            Vertex { position: [a_pos[0] + offset2[0], a_pos[1] + offset2[1]], color: a.color },
            Vertex { position: [b_pos[0] + offset2[0], b_pos[1] + offset2[1]], color: b.color },
        ]
    }).flatten().collect()
}

fn set_length(vector: Position, new_length: f32) -> Position {
    let current_length: f32 = (vector[0].powf(2.) + vector[1].powf(2.)).sqrt();

    if current_length == new_length {
        return vector;
    }

    let normalized = [vector[0] / current_length, vector[1] / current_length];
    [normalized[0] * new_length, normalized[1] * new_length]
}
