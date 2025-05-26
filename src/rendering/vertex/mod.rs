pub mod vec2;

use rayon::prelude::*;

use vec2::Vec2;

// follow C's rules for the memory layout (e.g. dont reorder)
#[repr(C)]
#[derive(
    Clone, Copy, PartialEq, Debug,
    // allow bitwise casts with bytemuck
    bytemuck::Zeroable, bytemuck::Pod,
)]
/// a vertex to store in the vertex buffer
pub struct Vertex {
    pub position: Vec2,
    // need 32 bits to avoid padding and have a corresponding type in WGSL,
    // otherwise u8 would have had enough possible values
    /// fractal iteration this vertex was created in,
    /// 0 meaning the initial state
    pub iteration: u32,
}

impl Vertex {
    /// shape of each vertex for the buffer
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        // map shader locations to the data types
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Uint32];

    pub const fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    pub const fn new(position: Vec2, iteration: u32) -> Self {
        Self { position, iteration }
    }
}


/// describes how an array of vertices should be interpreted
pub enum VertexFormat {
    /// groups of two to form lines
    Lines,
    /// groups of three to form triangles
    Triangles,
}

/// transform ordered, partly duplicate vertices into unique vertices and indices 
pub fn index(vertices: &[Vertex]) -> (Vec<Vertex>, Vec<u32>) {
    // efficient handling of data is a lot simpler when you can e.g.
    // hash, order or compare it. rusts floating point primitives
    // (that are part of each Vertex) have some difficulties with that.
    // solution: map vertices to alternative struct,
    //           work with that, and finally map back

    // for bitwise casts to/from alternative struct
    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Zeroable, Pod, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct AltVec2 { x: u32, y: u32 }

    #[repr(C)]
    #[derive(Zeroable, Pod, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct AltVertex {
        position: AltVec2,
        iteration: u32,
    }

    // hashable alternative structs should have the exact same memory layout
    // so that we can use std::mem::transmute
    use std::mem::{align_of, size_of};
    assert_eq!( size_of::<Vertex>(),  size_of::<AltVertex>());
    assert_eq!(align_of::<Vertex>(), align_of::<AltVertex>());
    assert_eq!( size_of::<  Vec2>(),  size_of::<  AltVec2>());
    assert_eq!(align_of::<  Vec2>(), align_of::<  AltVec2>());

    let alt_vertices: Vec<AltVertex> = bytemuck::cast_slice(vertices).to_vec();

    log::debug!("determining unique vertices");
    let mut unique_alt_vertices = alt_vertices.clone();
    unique_alt_vertices.par_sort_unstable();
    unique_alt_vertices.dedup();

    let unique_vertices = bytemuck::cast_slice(&unique_alt_vertices).to_vec();

    // for O(1) lookups when building indices from vertices
    log::debug!("building vertex-index-map");
    let vertex_index_map = unique_alt_vertices.par_iter()
        .enumerate()
        .map(|t| (t.1, t.0.try_into().unwrap()))
        .collect::<std::collections::HashMap<_, _>>();

    log::debug!("determining indices");
    let indices = alt_vertices.par_iter()
        .map(|v| *vertex_index_map.get(&v).unwrap())
        .collect::<Vec<_>>();

    log::info!("determined {} triangles to render", indices.len() / 3);
    (unique_vertices, indices)
}

pub fn lines_as_triangles(vertices: &[Vertex], line_width: f32) -> Vec<Vertex> {
    log::debug!("representing lines as triangles");
    vertices.par_chunks(2).map(|line| {
        let (a, b) = (line[0].position, line[1].position);
        let (a_iter, b_iter) = (line[0].iteration, line[1].iteration);

        let a_to_b = b - a;

        // orthogonal to vector to make rectangle
        let offset1 = Vec2::new(-a_to_b.y,  a_to_b.x).set_len(line_width / 2.);
        let offset2 = Vec2::new( a_to_b.y, -a_to_b.x).set_len(line_width / 2.);

        [
            // line as rectangle of two triangles

            Vertex::new(a + offset1, a_iter),
            Vertex::new(a + offset2, a_iter),
            Vertex::new(b + offset1, b_iter),

            Vertex::new(b + offset1, b_iter),
            Vertex::new(a + offset2, a_iter),
            Vertex::new(b + offset2, b_iter),

            //// smoother ends of line
            // currently disabled, as this uses about 1.6x the memory per line
            // while being very hard to notice visually

            //let a_offset = (-a_to_b).set_len(line_width / 2.);
            //let b_offset = ( a_to_b).set_len(line_width / 2.);

            //Vertex::new(a +  offset1, a_iter),
            //Vertex::new(a + a_offset, a_iter),
            //Vertex::new(a +  offset2, a_iter),

            //Vertex::new(b +  offset1, b_iter),
            //Vertex::new(b +  offset2, b_iter),
            //Vertex::new(b + b_offset, b_iter),
        ]
    }).flatten().collect()
}
