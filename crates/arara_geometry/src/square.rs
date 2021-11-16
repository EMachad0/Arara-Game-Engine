use crate::Vertex;
use crate::Shape;

pub struct Square {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}
impl Shape for Square {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

impl Square {
    pub fn new() -> Self {
        let normal = [0.0, 1.0, 0.0];

        let vertices = vec![
            Vertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], normal },
            Vertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0], normal },
            Vertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0], normal },
            Vertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0], normal },
        ];

        let indices = vec![
            0, 1, 2,
            0, 2, 3,
        ];

        Self {
            vertices,
            indices,
        }
    }
}
