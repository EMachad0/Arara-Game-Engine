use std::f32::consts::PI;

use crate::Vertex;
use crate::Shape;

pub struct Sphere {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}
impl Shape for Sphere {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }
}


impl Default for Sphere {
    fn default() -> Self {
        Self::new(36, 18, 1.0)
    }
}

impl Sphere {
    pub fn new(sector_count: u32, stack_count: u32, radius: f32) -> Self {
        let sector_step = 2.0 * PI / sector_count as f32;
        let stack_step = PI / stack_count as f32;

        let mut vertices: Vec<Vertex> = Vec::new();

        for i in 0..(stack_count+1) {
            let stack_angle = PI / 2.0 - (i as f32) * stack_step;
            let xy = stack_angle.cos();
            let z = stack_angle.sin();

            for j in 0..(sector_count+1) {
                let sector_angle = j as f32 * sector_step;

                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();

                let position = [x * radius, y * radius, z * radius];
                let normal = position;

                let s = (j as f32) / (sector_count as f32);
                let t = (i as f32) / (stack_count as f32);
                let tex_coords = [s, t];

                vertices.push(Vertex {
                    position,
                    normal,
                    tex_coords,
                });
            }
        }

        let mut indices: Vec<u32> = Vec::new();
        for i in 0..stack_count {
            let mut k1 = i * (sector_count + 1);
            let mut k2 = k1 + sector_count + 1;

            for _ in 0..sector_count {
                if i != 0 {
                    indices.push(k1);
                    indices.push(k2);
                    indices.push(k1+1);
                }
                if i != stack_count-1 {
                    indices.push(k1+1);
                    indices.push(k2);
                    indices.push(k2+1);
                }

                k1 += 1;
                k2 += 1;
            }
        }

        Self {
            indices,
            vertices,
        }
    }
}