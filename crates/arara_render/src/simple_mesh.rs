
use bevy_ecs::bundle::Bundle;
use glam::{Vec3, vec3};

use crate::Color;
use arara_shaders::Shaders;
use arara_transform::{GlobalTransform, Transform};
use arara_geometry::{Shape, Sphere};

#[derive(Bundle)]
pub struct SimpleMeshBundle {
    pub mesh: Box<dyn Shape>,
    pub shaders: Shaders,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub color: Color,
}

impl Default for SimpleMeshBundle {
    fn default() -> Self {
        Self { 
            mesh: Box::new(Sphere::default()),
            shaders: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            color: Default::default()
        }
    }
}

pub struct BPLight {
    pub position: Vec3,
}

impl Default for BPLight {
    fn default() -> Self {
        Self::new(0.0, 10.0, 0.0)
    }
}

impl BPLight {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {position: vec3(x, y, z)}
    }
}

