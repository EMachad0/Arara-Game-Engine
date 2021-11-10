use super::vertex::Vertex;

pub struct Cuboid {
    pub vertices: [Vertex; 8],
    pub indices: [u16; 36],
}

impl Default for Cuboid {
    fn default() -> Self {
        let vertices = [
            Vertex { position: [ 0.0, 1.0, 1.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 0.0, 0.0, 1.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 1.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 0.0, 1.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0,  1.0] },
            Vertex { position: [ 1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0,  1.0] },
            Vertex { position: [ 0.0, 0.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0,  1.0] },
            Vertex { position: [ 1.0, 0.0, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0,  1.0] },
        ];
        
        let indices = [
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