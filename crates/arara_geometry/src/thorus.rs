use std::f32::consts::PI;

use glam::vec3;

use crate::Vertex;
use crate::Shape;

pub struct Thorus {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}
impl Shape for Thorus {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

impl Thorus {
    pub fn new(subdivisions_segments: u32, subdivisions_sides: u32, radius: f32, ring_radius: f32) -> Self {
        let n_vertices = (subdivisions_segments + 1) * (subdivisions_sides + 1);
        let mut vertices = Vec::with_capacity(n_vertices as usize); 

        let segment_stride = 2.0 * PI / subdivisions_segments as f32;
        let side_stride = 2.0 * PI / subdivisions_sides as f32;

        for segment in 0..=subdivisions_segments {
            let theta = segment_stride * segment as f32;
            // let segment_pos = vec3(theta.cos(), 0.0, theta.sin() * radius);
            let tang = vec3(-theta.sin(), theta.cos(), 0.0);
            
            for side in 0..=subdivisions_sides {
                let phi = side_stride * side as f32;
                
                let x = theta.cos() * (radius + ring_radius * phi.cos());
                let z = theta.sin() * (radius + ring_radius * phi.cos());
                let y = ring_radius * phi.sin();
                
                let stang = vec3(theta.cos() * -phi.sin(), theta.sin() * -phi.sin(), phi.cos());
                let normal = tang.cross(stang).normalize();
                let tex_coords = [
                    segment as f32 / subdivisions_segments as f32,
                    side as f32 / subdivisions_sides as f32,
                ];

                vertices.push(Vertex {
                    position: [x, y, z],
                    normal: normal.into(),
                    tex_coords,
                });
            }
        }

        let n_faces = (subdivisions_segments) * (subdivisions_sides);
        let n_triangles = n_faces * 2;
        let n_indices = n_triangles * 3;

        let mut indices: Vec<u32> = Vec::with_capacity(n_indices as usize);

        let n_vertices_per_row = subdivisions_sides + 1;
        for segment in 0..subdivisions_segments {
            for side in 0..subdivisions_sides {
                let lt = side + segment * n_vertices_per_row;
                let rt = (side + 1) + segment * n_vertices_per_row;

                let lb = side + (segment + 1) * n_vertices_per_row;
                let rb = (side + 1) + (segment + 1) * n_vertices_per_row;

                indices.push(lt as u32);
                indices.push(rt as u32);
                indices.push(lb as u32);

                indices.push(rt as u32);
                indices.push(rb as u32);
                indices.push(lb as u32);
            }
        }

        Self {
            vertices,
            indices,
        }
    }
}
