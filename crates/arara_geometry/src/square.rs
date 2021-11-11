use super::vertex::Vertex;

pub struct Square {
    pub vertices: [Vertex; 4],
    pub indices: [u32; 6],
}

impl Square {
    pub fn new() -> Self {
        let normal = [0.0, 1.0, 0.0];

        let vertices = [
            Vertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], normal },
            Vertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0], normal },
            Vertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0], normal },
            Vertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0], normal },
        ];

        let indices = [
            0, 1, 2,
            0, 2, 3,
        ];

        Self {
            vertices,
            indices,
        }
    }
}
