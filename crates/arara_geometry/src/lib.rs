mod vertex;
mod square;
mod circle;
mod cuboid;
mod sphere;
mod cylinder;
mod thorus;
mod capsule;
mod icosphere;

pub use vertex::*;
pub use square::*;
pub use circle::*;
pub use cuboid::*;
pub use sphere::*;
pub use cylinder::*;
pub use thorus::*;
pub use capsule::*;
pub use icosphere::*;

pub mod prelude {
    pub use crate::{
        vertex::Vertex,
        square::Square,
        circle::Circle,
        cuboid::Cuboid,
        sphere::Sphere,
        cylinder::Cylinder,
        thorus::Thorus,
        capsule::{Capsule, CapsuleUvProfile},
        icosphere::Icosphere,
    };
}

pub trait Shape: Sync + Send {
    fn get_vertices(&self) -> &Vec<Vertex>;
    fn get_indices(&self) -> &Vec<u32>;
}