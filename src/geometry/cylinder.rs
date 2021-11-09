
use std::f32::consts::PI;

use super::vertex::Vertex;
use super::circle;

pub struct Cylinder {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Cylinder {
    pub fn new(sector_count: u32) -> Self {
        let circle_points = circle::circle_points(sector_count);

        let mut vertices: Vec<Vertex> = Vec::new();
        
        for (i, vertice) in circle_points.iter().enumerate() {
            let position = [vertice.x, vertice.y, 0.0];
            let normal = [vertice.x, vertice.y, 0.0];
            let tex_coords = [i as f32 / sector_count as f32, 0.0];

            vertices.push(Vertex {
                position, 
                normal,
                tex_coords,
            });
        }

        for (i, vertice) in circle_points.iter().enumerate() {
            let position = [vertice.x, vertice.y, 1.0];
            let normal = [vertice.x, vertice.y, 0.0];
            let tex_coords = [i as f32 / sector_count as f32, 1.0];

            vertices.push(Vertex {
                position, 
                normal,
                tex_coords,
            });
        }

        // let base_center_index = vertices.len() as u32;
        // let top_center_index = base_center_index + sector_count + 1;

        let mut indices: Vec<u32> = Vec::new();
        let mut k1 = 0;
        let mut k2 = sector_count;

        for _ in 0..sector_count {
            indices.push(k1);
            indices.push(k1+1);
            indices.push(k2);

            indices.push(k2);
            indices.push(k1+1);
            indices.push(k2+1);

            k1 += 1;
            k2 += 1;
        }

        Cylinder {
            vertices,
            indices,
        }
    }

}
