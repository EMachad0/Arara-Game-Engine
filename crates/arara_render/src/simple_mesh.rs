
use bevy_ecs::bundle::Bundle;

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

