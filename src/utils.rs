use super::Vertex;

/// transform ordered, partly duplicate vertices into unique vertices and indices 
fn index_vertices(vertices: &[Vertex]) -> (Vec<Vertex>, Vec<u16>) {
    let unique_vertices = unique(vertices.iter().map(|x| *x));
    let indices = vertices.iter().map(|vertex| {
        unique_vertices.iter().position(|x| x == vertex).unwrap().try_into().unwrap()
    }).collect();
    (unique_vertices, indices)
}

/// get unique elements of an iterator without hashing
fn unique<T: Clone + PartialEq, I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
    let mut result = vec![];
    for item in iter {
        if !result.contains(&item) {
            result.push(item.clone());
        }
    }
    result
}
