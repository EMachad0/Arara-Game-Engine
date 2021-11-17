
use bevy_ecs::bundle::Bundle;
use cgmath::{vec3, Vector3};

use crate::Color;
use arara_shaders::Shaders;
use arara_transform::Transform;
use arara_geometry::Shape;

#[derive(Bundle)]
pub struct SimpleMeshBundle {
    pub mesh: Box<dyn Shape>,
    pub shaders: Shaders,
    pub transform: Transform,
    pub color: Color,
}

pub struct BPLight {
    pub position: Vector3<f32>,
}

impl Default for BPLight {
    fn default() -> Self {
        Self {
            position: vec3(0.0, 10.0, 0.0),
        }
    }
}

