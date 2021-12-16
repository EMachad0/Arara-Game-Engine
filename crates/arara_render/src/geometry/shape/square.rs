use crate::geometry::{Vertex, Mesh};

pub struct Square {
    x_lenght: f32,
    y_lenght: f32,
}

impl Default for Square {
    fn default() -> Self {
        Self::new(1., 1.)
    }
}

impl Square {
    pub fn new(x_lenght: f32, y_lenght: f32) -> Self {
        Self {
            x_lenght,
            y_lenght,
        }
    }
}

impl From<Square> for Mesh {
    fn from(square: Square) -> Self {
        let hx = square.x_lenght / 2.;
        let hy = square.y_lenght / 2.;
        let normal = [0.0, 0.0, -1.0];

        let vertices = vec![
            Vertex { position: [-hx, -hy, 0.0], tex_coords: [0.0, 0.0], normal },
            Vertex { position: [-hx,  hy, 0.0], tex_coords: [0.0, 1.0], normal },
            Vertex { position: [ hx,  hy, 0.0], tex_coords: [1.0, 1.0], normal },
            Vertex { position: [ hx, -hy, 0.0], tex_coords: [1.0, 0.0], normal },
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
