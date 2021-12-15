use bevy_ecs::bundle::Bundle;
use glam::{Vec3, vec3};

use crate::{Color, Image, prelude::Visibility, geometry::Mesh};
use arara_asset::Handle;
use arara_transform::{GlobalTransform, Transform};

#[derive(Bundle, Default)]
pub struct SimpleMeshBundle {
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub color: Color,
    pub image: Option::<Handle<Image>>,
    pub visibility: Visibility,
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

