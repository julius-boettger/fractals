pub mod canopy;
pub mod koch_snowflake;
pub mod sierpinski_triangle;

use strum::IntoEnumIterator;

use crate::renderer::vertex::{Vertex, VertexFormat};

#[derive(Default, PartialEq, strum::EnumIter)]
pub enum Curves {
    #[default] // first rendered on program start
    Canopy,
    KochSnowflake,
    SierpinskiTriangle,
}
impl Curves {
    pub fn new_instance(&self) -> Box<dyn Curve> {
        match self {
            Self::Canopy => Box::new(canopy::Canopy::new()),
            Self::KochSnowflake => Box::new(koch_snowflake::KochSnowflake::new()),
            Self::SierpinskiTriangle => Box::new(sierpinski_triangle::SierpinskiTriangle::new()),
        }
    }

    pub fn next(&mut self) {
        let mut cycle = Self::iter().cycle();
        while cycle.next().unwrap() != *self {}
        *self = cycle.next().unwrap()
    }

    pub fn prev(&mut self) {
        let mut cycle = Self::iter().cycle();
        let mut next1 = cycle.next().unwrap();
        let mut next2 = cycle.next().unwrap();
        while next2 != *self {
            next1 = next2;
            next2 = cycle.next().unwrap();
        }
        *self = next1
    }
}

/// https://en.wikipedia.org/wiki/Fractal_curve
pub trait Curve: std::any::Any {
    fn new() -> Self
        where Self: Sized; // for dyn-compatability

    // technically dont need to be methods, but are easier to work with
    fn vertex_format(&self) -> VertexFormat;
    fn default_iteration(&self) -> usize;

    fn next_iteration(&self, last_vertices: &Vec<Vertex>, iteration: u32) -> Vec<Vertex>;

    /// one element for each iteration
    fn     data(&    self) -> &    Vec<Vec<Vertex>>;
    fn mut_data(&mut self) -> &mut Vec<Vec<Vertex>>;

    /// cast to current type of curve
    fn downcast(curve: &mut Box<dyn Curve>) -> &mut Self
        where Self: Sized // for dyn-compatability
    {
        (&mut **curve as &mut dyn std::any::Any).downcast_mut::<Self>().unwrap()
    }

    /// iteration 0 meaning initial state
    fn vertices(&mut self, iteration: usize) -> &Vec<Vertex> {
        // compute fractal iterations (if not done already)
        for current_iteration in self.data().len() ..= iteration {
            if current_iteration >= 9 {
                log::debug!("computing iteration {}", current_iteration + 1);
            }

            let vertices = self.next_iteration(
                &self.data()[current_iteration - 1],
                current_iteration.try_into().unwrap()
            );

            self.mut_data().push(vertices);
        }

        let fractal_name = std::any::type_name::<Self>()
            .rsplit_once("::")
            .unwrap()
            .1;
        log::info!("computed iteration {} of {}", iteration + 1, fractal_name);

        &self.data()[iteration]
    }
}
