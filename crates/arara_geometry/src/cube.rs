use crate::Vertex;
use crate::Shape;

pub struct Cube {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}
impl Shape for Cube {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

impl Default for Cube {
    fn default() -> Self {
        let vertices = vec![
            Vertex { position: [ 0.0, 1.0, 1.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 0.0, 0.0, 1.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 1.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 0.0, 1.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0,  1.0] },
            Vertex { position: [ 1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0,  1.0] },
            Vertex { position: [ 0.0, 0.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0,  1.0] },
            Vertex { position: [ 1.0, 0.0, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0,  1.0] },
        ];
        
        let indices = vec![
            2, 3, 1,
            2, 1, 0,
            2, 0, 4,
            2, 4, 6,
            2, 6, 7,
            2, 7, 3,
            5, 4, 6,
            5, 6, 7,
            5, 1, 0,
            5, 0, 4,
            5, 7, 3, 
            5, 3, 1,
        ];

        Self {
            vertices,
            indices,
        }
    }
}

/* faces:
      0--1
     /|  |
    4 2--3
    |/  /
    6--7
        
      0--1
     /  /|
    4--5 3
    |  |/
    6--7

    2 3 1 0
    2 0 4 6
    2 6 7 3
    5 4 6 7
    5 1 0 4
    5 7 3 1
*/