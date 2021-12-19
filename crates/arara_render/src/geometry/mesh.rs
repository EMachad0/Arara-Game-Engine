use bevy_reflect::TypeUuid;

use crate::geometry::shape::Vertex;

#[derive(Debug, Clone, TypeUuid, Default)]
#[uuid = "8ecbac0f-f545-4473-ad43-e1f4243af51e"]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}
