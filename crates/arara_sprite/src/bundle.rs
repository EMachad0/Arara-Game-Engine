use arara_asset::Handle;
use arara_ecs::bundle::Bundle;
use arara_render::{Color, Image, Visibility, DEFAULT_IMAGE_HANDLE};
use arara_transform::{GlobalTransform, Transform};

use crate::{
    sprite::Sprite,
    texture_atlas::{TextureAtlas, TextureAtlasSprite},
};

#[derive(Bundle, Clone)]
pub struct SpriteBundle {
    /// Sprite marker
    pub sprite: Sprite,
    /// Handle of the sprite's texture
    pub image: Handle<Image>,
    /// Data pertaining to how the sprite is drawn on the screen
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// Sprite Color
    pub color: Color,
    /// Indication of whether an entity is visible
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

/// A Bundle of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`)
#[derive(Bundle, Clone, Default)]
pub struct SpriteSheetBundle {
    /// The specific sprite from the texture atlas to be drawn
    pub sprite: TextureAtlasSprite,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,
    /// Data pertaining to how the sprite is drawn on the screen
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// Sprite Color
    pub color: Color,
    /// Indication of whether an entity is visible
    pub visibility: Visibility,
}
