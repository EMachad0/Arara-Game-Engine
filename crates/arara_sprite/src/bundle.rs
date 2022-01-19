use arara_asset::Handle;
use arara_ecs::{bundle::Bundle, component::Component};
use arara_render::{Image, Visibility, DEFAULT_IMAGE_HANDLE};
use arara_transform::{GlobalTransform, Transform};

#[derive(Component, Default, Clone)]
pub struct Sprite;

#[derive(Bundle, Clone)]
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
}

impl Default for SpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
        }
    }
}
