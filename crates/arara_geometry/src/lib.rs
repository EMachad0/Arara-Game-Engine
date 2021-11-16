pub mod vertex;
pub mod square;
pub mod circle;
pub mod cube;
pub mod sphere;
pub mod cylinder;

pub use vertex::*;
pub use square::*;
pub use circle::*;
pub use cube::*;
pub use sphere::*;
pub use cylinder::*;

pub trait Shape: Sync + Send {
    fn get_vertices(&self) -> &Vec<Vertex>;
    fn get_indices(&self) -> &Vec<u32>;
}

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