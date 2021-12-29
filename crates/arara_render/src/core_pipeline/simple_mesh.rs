use arara_ecs::prelude::*;
use glam::{vec3, Mat4, Vec3};

use crate::{geometry::Mesh, prelude::Visibility, Color, Image, Shader, DEFAULT_IMAGE_HANDLE};
use arara_asset::{AssetServer, Handle};
use arara_transform::{GlobalTransform, Transform};

#[derive(Bundle)]
pub struct SimpleMeshBundle {
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub color: Color,
    pub image: Handle<Image>,
    pub visibility: Visibility,
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
        }
    }
}

#[derive(Component)]
pub struct ExtractedCPE {
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

#[derive(Default)]
pub struct DefaultShader {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
}

pub fn load_default_shader(mut commands: Commands, asset_server: Res<AssetServer>) {
    let vertex_shader = asset_server.load("shaders/vertex_shader_src.vert");
    let fragment_shader = asset_server.load("shaders/fragment_shader_src.frag");
    commands.insert_resource(DefaultShader {
        vertex_shader,
        fragment_shader,
    });
}
