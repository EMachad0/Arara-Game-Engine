use arara_ecs::prelude::*;
use arara_render::{Color, Image, Mesh, Visibility, DEFAULT_IMAGE_HANDLE};
use glam::{vec3, Mat4, Vec3};

use arara_asset::Handle;
use arara_transform::{GlobalTransform, Transform};

#[derive(Default, Component)]
pub struct CorePipelineEntity;

#[derive(Bundle)]
pub struct SimpleMeshBundle {
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub color: Color,
    pub image: Handle<Image>,
    pub visibility: Visibility,
    pub core_pipeline_entity: CorePipelineEntity,
}

impl Default for SimpleMeshBundle {
    fn default() -> Self {
        Self {
            mesh: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            color: Default::default(),
            image: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            core_pipeline_entity: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct ExtractedCorePipelineEntity {
    pub mesh: Handle<Mesh>,
    pub image: Handle<Image>,
    pub transform: Mat4,
    pub color: Color,
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
        Self {
            position: vec3(x, y, z),
        }
    }
}
