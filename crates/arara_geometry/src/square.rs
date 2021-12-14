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

impl Default for Square {
    fn default() -> Self {
        Self::new(1., 1.)
    }
}

impl Square {
    pub fn new(x_lenght: f32, z_lenght: f32) -> Self {
        let hx = x_lenght / 2.;
        let hz = z_lenght / 2.;
        let normal = [0.0, 1.0, 0.0];

        let vertices = vec![
            Vertex { position: [-hx, 0.0, -hz], tex_coords: [0.0, 0.0], normal },
            Vertex { position: [-hx, 0.0,  hz], tex_coords: [0.0, 1.0], normal },
            Vertex { position: [ hx, 0.0,  hz], tex_coords: [1.0, 1.0], normal },
            Vertex { position: [ hx, 0.0, -hz], tex_coords: [1.0, 0.0], normal },
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
