use arara_asset::Handle;
use arara_ecs::component::Component;
use arara_render::Image;
use arara_utils::HashMap;
use bevy_reflect::{Reflect, TypeUuid};
use glam::Vec2;

/// Used for Texture Atlas Uv Mapping
#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct TextureAtlasCoord {
    /// The beginning point of the rect
    pub point: Vec2,
    /// The size of the tile
    pub size: Vec2,
}

/// An atlas containing multiple textures (like a spritesheet or a tilemap).
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "7233c597-ccfa-411f-bd59-9af349432ada"]
pub struct TextureAtlas {
    /// The handle to the texture in which the sprites are stored
    pub texture: Handle<Image>,
    // TODO: add support to Uniforms derive to write dimensions and sprites to the same buffer
    pub size: Vec2,
    /// The specific areas of the atlas where each texture can be found
    pub textures: Vec<TextureAtlasCoord>,
    pub texture_handles: Option<HashMap<Handle<Image>, usize>>,
}

#[derive(Component, Debug, Clone, Default, TypeUuid, Reflect)]
#[uuid = "7233c597-ccfa-411f-bd59-9af349432ada"]
pub struct TextureAtlasSprite {
    pub index: usize,
}

impl TextureAtlasSprite {
    pub fn new(index: usize) -> TextureAtlasSprite {
        Self {
            index,
            ..Default::default()
        }
    }
}

impl TextureAtlas {
    /// Create a new `TextureAtlas` that has a texture, but does not have
    /// any individual sprites specified
    pub fn new_empty(texture: Handle<Image>, dimensions: Vec2) -> Self {
        Self {
            texture,
            size: dimensions,
            texture_handles: None,
            textures: Vec::new(),
        }
    }

    /// Generate a `TextureAtlas` by splitting a texture into a grid where each
    /// cell of the grid  of `tile_size` is one of the textures in the atlas
    pub fn from_grid(
        texture: Handle<Image>,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
    ) -> TextureAtlas {
        Self::from_grid_with_padding(texture, tile_size, columns, rows, Vec2::ZERO)
    }

    /// Generate a `TextureAtlas` by splitting a texture into a grid where each
    /// cell of the grid of `tile_size` is one of the textures in the atlas and is separated by
    /// some `padding` in the texture
    pub fn from_grid_with_padding(
        texture: Handle<Image>,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        padding: Vec2,
    ) -> TextureAtlas {
        let mut textures = Vec::new();
        let mut x_padding = 0.0;
        let mut y_padding = 0.0;
        let size = Vec2::new(
            ((tile_size.x + x_padding) * columns as f32) - x_padding,
            ((tile_size.y + y_padding) * rows as f32) - y_padding,
        );

        for y in 0..rows {
            if y > 0 {
                y_padding = padding.y;
            }
            for x in 0..columns {
                if x > 0 {
                    x_padding = padding.x;
                }
                let point = Vec2::new(
                    (tile_size.x + x_padding) * x as f32,
                    (tile_size.y + y_padding) * y as f32 - y_padding,
                );
                textures.push(TextureAtlasCoord {
                    point,
                    size: tile_size,
                });
            }
        }

        TextureAtlas {
            size,
            textures,
            texture,
            texture_handles: None,
        }
    }

    /// Add a sprite to the list of textures in the `TextureAtlas`
    /// returns an index to the texture which can be used with `TextureAtlasSprite`
    ///
    /// # Arguments
    ///
    /// * `texture` - The section of the atlas that contains the texture to be added,
    /// from the top-left corner of the texture to the bottom-right corner
    pub fn add_texture(&mut self, texture: TextureAtlasCoord) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }

    /// How many textures are in the `TextureAtlas`
    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    pub fn get_texture_index(&self, texture: &Handle<Image>) -> Option<usize> {
        self.texture_handles
            .as_ref()
            .and_then(|texture_handles| texture_handles.get(texture).cloned())
    }
}
