use arara_asset::Handle;
use arara_ecs::bundle::Bundle;
use arara_render::{Color, Image, Visibility, DEFAULT_IMAGE_HANDLE};
use arara_transform::{GlobalTransform, Transform};

use crate::sprite::Sprite;

#[derive(Bundle, Clone)]
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub color: Color,
    pub image: Handle<Image>,
    pub visibility: Visibility,
}

impl Default for SpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            color: Default::default(),
            image: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
        }
    }
}
