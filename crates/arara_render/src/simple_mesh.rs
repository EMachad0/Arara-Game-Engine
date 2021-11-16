
use bevy_ecs::bundle::Bundle;

use crate::Color;
use arara_shaders::Shaders;
use arara_transform::Transform;
use arara_geometry::Sphere;

#[derive(Bundle, Default)]
pub struct SimpleMeshBundle {
    pub mesh: Sphere,
    pub shaders: Shaders,
    pub transform: Transform,
    pub color: Color,
}

