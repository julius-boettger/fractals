use itertools::Itertools;

use crate::{render::Vertex, vec2::Vec2};

/// describes how an array of vertices should be interpreted
pub enum VertexFormat {
    /// groups of two to form lines
    Lines,
    /// groups of three to form triangles
    #[allow(dead_code)]
    Triangles,
}

/// transform ordered, partly duplicate vertices into unique vertices and indices 
pub fn index_vertices(vertices: &[Vertex]) -> (Vec<Vertex>, Vec<u32>) {
    // efficiently determining unique values of a collection
    // usually requires hashing, and rusts floating point primitives
    // (that are part of each Vertex) are not that easy to hash.
    // solution: map vertices to alternative, hashable structs/types,
    //           work with those, and finally map back

    #[derive(Clone, PartialEq, Eq, Hash)]
    struct HashableVec2 { x: u32, y: u32 }

    #[derive(Clone, PartialEq, Eq, Hash)]
    struct HashableVertex {
        position: HashableVec2,
        iteration: u32,
    }

    // hashable alternative structs should have the exact same memory layout
    // so that we can use std::mem::transmute
    use std::mem::{align_of, size_of, transmute};
    assert_eq!( size_of::<Vertex>(),  size_of::<HashableVertex>());
    assert_eq!(align_of::<Vertex>(), align_of::<HashableVertex>());
    assert_eq!( size_of::<  Vec2>(),  size_of::<  HashableVec2>());
    assert_eq!(align_of::<  Vec2>(), align_of::<  HashableVec2>());

    let vertices_iter = vertices.iter()
        .map(|v| unsafe { transmute(*v) });
    let vertices = vertices_iter.clone().collect_vec();

    log::debug!("collecting unique vertices from iterator");
    let unique_vertices_iter = vertices_iter.unique();
    let unique_vertices = unique_vertices_iter.clone()
        .map(|v| unsafe { transmute::<HashableVertex, _>(v) })
        .collect_vec();

    // for O(1) lookups when building indices from vertices
    log::debug!("building vertex-index-map for {} unique vertices", unique_vertices.len());
    let vertex_index_map = unique_vertices_iter
        .enumerate()
        .map(|t| (t.1, t.0.try_into().unwrap()))
        .collect::<std::collections::HashMap<_, _>>();

    log::debug!("building indices from raw vertices and unique vertices");
    let indices = vertices.iter()
        .map(|v| *vertex_index_map.get(v).unwrap())
        .collect_vec();
    log::debug!("built {} indices ({} triangles)", indices.len(), indices.len() / 3);

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
