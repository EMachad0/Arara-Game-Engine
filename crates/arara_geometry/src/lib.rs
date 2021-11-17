mod vertex;
mod square;
mod circle;
mod cube;
mod sphere;
mod cylinder;

pub use vertex::*;
pub use square::*;
pub use circle::*;
pub use cube::*;
pub use sphere::*;
pub use cylinder::*;

pub mod prelude {
    pub use crate::{
        vertex::Vertex,
        square::Square,
        circle::Circle,
        cube::Cube,
        sphere::Sphere,
        cylinder::Cylinder,
    };
}

pub trait Shape: Sync + Send {
    fn get_vertices(&self) -> &Vec<Vertex>;
    fn get_indices(&self) -> &Vec<u32>;
}