use arara_asset::Handle;
use arara_ecs::component::Component;
use arara_render::{Color, Image};
use glam::Mat4;

use crate::texture_atlas::TextureAtlasCoord;

#[derive(Component, Default, Clone)]
pub struct Sprite;

#[derive(Component)]
pub struct ExtractedSprite {
    pub image_handle: Handle<Image>,
    pub transform: Mat4,
    pub uv_coord: Option<TextureAtlasCoord>,
    pub color: Color,
    pub z: f32,
}
