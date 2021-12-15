use crate::geometry::{Vertex, Mesh};

pub struct Cuboid {
    x_length: f32,
    y_length: f32,
    z_length: f32,
}

struct Cords {
    max_x: f32,
    min_x: f32,
    max_y: f32,
    min_y: f32,
    max_z: f32,
    min_z: f32,
}

impl Cuboid {
    pub fn new(x_length: f32, y_length: f32, z_length: f32) -> Self {
        Self {
            x_length,
            y_length,
            z_length,
        }
    }
}

impl Default for Cuboid {
    fn default() -> Self {
        Self::new(1., 1., 1.)
    }
}

impl From<Cuboid> for Mesh {
    fn from(cuboid: Cuboid) -> Self {
        let Cuboid {x_length, y_length, z_length} = cuboid;

        let sp = Cords {
            max_x: x_length / 2.0,
            min_x: -x_length / 2.0,
            max_y: y_length / 2.0,
            min_y: -y_length / 2.0,
            max_z: z_length / 2.0,
            min_z: -z_length / 2.0,
        };

        let vertices = vec![
            // Top
            Vertex {position: [sp.min_x, sp.min_y, sp.max_z], normal: [0., 0., 1.0],  tex_coords: [0., 0.]},
            Vertex {position: [sp.max_x, sp.min_y, sp.max_z], normal: [0., 0., 1.0],  tex_coords: [1.0, 0.]},
            Vertex {position: [sp.max_x, sp.max_y, sp.max_z], normal: [0., 0., 1.0],  tex_coords: [1.0, 1.0]},
            Vertex {position: [sp.min_x, sp.max_y, sp.max_z], normal: [0., 0., 1.0],  tex_coords: [0., 1.0]}, 
            Vertex {position: [sp.min_x, sp.max_y, sp.min_z], normal: [0., 0., -1.0], tex_coords:  [1.0, 0.]},
            Vertex {position: [sp.max_x, sp.max_y, sp.min_z], normal: [0., 0., -1.0], tex_coords:  [0., 0.]},
            Vertex {position: [sp.max_x, sp.min_y, sp.min_z], normal: [0., 0., -1.0], tex_coords:  [0., 1.0]},
            Vertex {position: [sp.min_x, sp.min_y, sp.min_z], normal: [0., 0., -1.0], tex_coords:  [1.0, 1.0]},
            Vertex {position: [sp.max_x, sp.min_y, sp.min_z], normal: [1.0, 0., 0.],  tex_coords: [0., 0.]},
            Vertex {position: [sp.max_x, sp.max_y, sp.min_z], normal: [1.0, 0., 0.],  tex_coords: [1.0, 0.]},
            Vertex {position: [sp.max_x, sp.max_y, sp.max_z], normal: [1.0, 0., 0.],  tex_coords: [1.0, 1.0]},
            Vertex {position: [sp.max_x, sp.min_y, sp.max_z], normal: [1.0, 0., 0.],  tex_coords: [0., 1.0]},
            Vertex {position: [sp.min_x, sp.min_y, sp.max_z], normal: [-1.0, 0., 0.], tex_coords:  [1.0, 0.]},
            Vertex {position: [sp.min_x, sp.max_y, sp.max_z], normal: [-1.0, 0., 0.], tex_coords:  [0., 0.]},
            Vertex {position: [sp.min_x, sp.max_y, sp.min_z], normal: [-1.0, 0., 0.], tex_coords:  [0., 1.0]},
            Vertex {position: [sp.min_x, sp.min_y, sp.min_z], normal: [-1.0, 0., 0.], tex_coords:  [1.0, 1.0]},
            Vertex {position: [sp.max_x, sp.max_y, sp.min_z], normal: [0., 1.0, 0.],  tex_coords: [1.0, 0.]},
            Vertex {position: [sp.min_x, sp.max_y, sp.min_z], normal: [0., 1.0, 0.],  tex_coords: [0., 0.]},
            Vertex {position: [sp.min_x, sp.max_y, sp.max_z], normal: [0., 1.0, 0.],  tex_coords: [0., 1.0]},
            Vertex {position: [sp.max_x, sp.max_y, sp.max_z], normal: [0., 1.0, 0.],  tex_coords: [1.0, 1.0]},
            Vertex {position: [sp.max_x, sp.min_y, sp.max_z], normal: [0., -1.0, 0.], tex_coords:  [0., 0.]},
            Vertex {position: [sp.min_x, sp.min_y, sp.max_z], normal: [0., -1.0, 0.], tex_coords:  [1.0, 0.]},
            Vertex {position: [sp.min_x, sp.min_y, sp.min_z], normal: [0., -1.0, 0.], tex_coords:  [1.0, 1.0]},
            Vertex {position: [sp.max_x, sp.min_y, sp.min_z], normal: [0., -1.0, 0.], tex_coords:  [0., 1.0]},
        ];
        
        let indices = vec![
            0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ];

        Self {
            vertices,
            indices,
        }
    }
}