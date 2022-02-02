use std::f32::consts::PI;

use glam::{vec2, Vec2};

use crate::{geometry::Vertex, Mesh};

pub struct Circle {
    sector_count: u32,
    radius: f32,
}

impl Circle {
    pub fn new(sector_count: u32, radius: f32) -> Self {
        Self {
            sector_count,
            radius,
        }
    }
}

impl From<Circle> for Mesh {
    fn from(circle: Circle) -> Mesh {
        let Circle {
            sector_count,
            radius,
        } = circle;

        let circle = unit_circle_points(sector_count);
        let normal = [0.0, 0.0, 1.0];

        let mut vertices: Vec<Vertex> = Vec::new();

        for vertice in circle.iter() {
            let position = [vertice.x * radius, vertice.y * radius, 0.0];
            let tex_coords = [(vertice.x + 1.0) / 2.0, (vertice.y + 1.0) / 2.0];
            vertices.push(Vertex {
                position,
                normal,
                tex_coords,
            });
        }

        let mut indices: Vec<u32> = Vec::new();
        let center_index = vertices.len() as u32;
        for i in 0..center_index {
            indices.push(center_index);
            indices.push(i);
            indices.push(if i + 1 == center_index { 0 } else { i + 1 });
        }

        vertices.push(Vertex {
            position: [0.0, 0.0, 0.0],
            normal,
            tex_coords: [0.5, 0.5],
        });

        Self { vertices, indices }
    }
}

pub fn unit_circle_points(sector_count: u32) -> Vec<Vec2> {
    let sector_step = 2.0 * PI / sector_count as f32;

    let mut circle_vertices = Vec::new();
    for i in 0..=sector_count {
        let sector_angle = i as f32 * sector_step;
        circle_vertices.push(vec2(sector_angle.cos(), sector_angle.sin()));
    }
    circle_vertices
}
