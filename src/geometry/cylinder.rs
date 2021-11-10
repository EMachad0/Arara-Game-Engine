use super::vertex::Vertex;
use super::circle;

pub struct Cylinder {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Cylinder {
    pub fn new(sector_count: u32, height: f32, base_radius: f32, top_radius: f32) -> Self {
        let circle_points = circle::unit_circle_points(sector_count);

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        
        // Cylinder body
        for (i, vertice) in circle_points.iter().enumerate() {
            let position = [vertice.x * base_radius, vertice.y * base_radius, -height / 2.0];
            let normal = [vertice.x, vertice.y, 0.0];
            let tex_coords = [i as f32 / sector_count as f32, 0.0];
            vertices.push(Vertex {
                position, 
                normal,
                tex_coords,
            });
        }

        for (i, vertice) in circle_points.iter().enumerate() {
            let position = [vertice.x * top_radius, vertice.y * top_radius, height / 2.0];
            let normal = [vertice.x, vertice.y, 0.0];
            let tex_coords = [i as f32 / sector_count as f32, 1.0];
            vertices.push(Vertex {
                position, 
                normal,
                tex_coords,
            });
        }

        let mut k1 = 0;
        let mut k2 = sector_count + 1;

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

        // base circle
        let base_normal = [0.0, 0.0, -1.0];
        let base_center_index = vertices.len() as u32;
        vertices.push(Vertex {
            position: [0.0, 0.0, -height / 2.0],
            normal: base_normal,
            tex_coords: [0.5, 0.5],
        });

        for vertice in circle_points.iter() {
            let position = [vertice.x * base_radius, vertice.y * base_radius, -height / 2.0];
            let tex_coords = [(vertice.x + 1.0) / 2.0, (vertice.y + 1.0) / 2.0];
            vertices.push(Vertex {
                position, 
                normal: base_normal,
                tex_coords,
            });
        }

        for i in (base_center_index + 1)..(vertices.len() as u32) {
            indices.push(base_center_index);
            indices.push(if i+1 == vertices.len() as u32 { base_center_index } else { i+1 });
            indices.push(i);
        }

        // top circle
        let top_normal = [0.0, 0.0, 1.0];
        let top_center_index = vertices.len() as u32;
        vertices.push(Vertex {
            position: [0.0, 0.0, height / 2.0],
            normal: top_normal,
            tex_coords: [0.5, 0.5],
        });

        for vertice in circle_points.iter() {
            let position = [vertice.x * top_radius, vertice.y * top_radius, height / 2.0];
            let tex_coords = [(vertice.x + 1.0) / 2.0, (vertice.y + 1.0) / 2.0];
            vertices.push(Vertex {
                position, 
                normal: top_normal,
                tex_coords,
            });
        }

        for i in (top_center_index + 1)..(vertices.len() as u32) {
            indices.push(i);
            indices.push(if i+1 == vertices.len() as u32 { top_center_index } else { i+1 });
            indices.push(top_center_index);
        }

        Cylinder {
            vertices,
            indices,
        }
    }

}
