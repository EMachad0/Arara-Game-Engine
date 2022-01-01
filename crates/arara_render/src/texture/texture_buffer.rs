use arara_asset::Handle;
use arara_ecs::system::NonSendMut;
use arara_utils::HashMap;
use glium::{implement_uniform_block, Display};
use glium::texture::{ResidentTexture, SrgbTexture2d, TextureHandle as GliumTextureHandle, RawImage2d};
use std::collections::VecDeque;

use crate::{Image, DEFAULT_IMAGE_HANDLE};

pub const TEXTURE_BUFFER_SIZE: usize = 5;

#[derive(Copy, Clone)]
pub struct TextureUniformBuffer<'a> {
    pub tex: [GliumTextureHandle<'a>; TEXTURE_BUFFER_SIZE],
}

implement_uniform_block!(TextureUniformBuffer<'a>, tex);

pub struct CachedTextureMeta {
    pub texture: ResidentTexture,
    pub uniform_buffer_position: usize,
    pub frames_since_last_use: usize,
    pub taken: bool,
}

pub struct TextureBuffer {
    available_positions: VecDeque<usize>,
    pub textures: HashMap<Handle<Image>, CachedTextureMeta>,
}

impl Default for TextureBuffer {
    fn default() -> Self {
        Self {
            available_positions: (0..TEXTURE_BUFFER_SIZE).collect(),
            textures: Default::default(),
        }
    }
}

impl TextureBuffer {
    /// Checks if the Handle is already on the buffer
    pub fn contains(&mut self, image_handle: &Handle<Image>) -> bool {
        if let Some(meta) = self.textures.get_mut(image_handle) {
            meta.frames_since_last_use = 0;
            meta.taken = true;
        };
        self.textures.contains_key(image_handle)
    }

    /// Inserts a texture that matches the `Handle`.
    /// If no matching one is found a new [`glium::TextureHandle`] is created.
    pub fn insert(&mut self, image_handle: Handle<Image>, display: &Display, image: &Image) {
        match self.textures.entry(image_handle) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let meta = entry.get_mut();
                meta.frames_since_last_use = 0;
                meta.taken = true;
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                assert!(
                    !self.available_positions.is_empty(),
                    "
                    No texture slot avaible in the Texture Buffer
                    You are probably using more than {} textures!
                    this value can be tweaked on [arara_render::texture::texture_buffer]
                    ",
                    TEXTURE_BUFFER_SIZE
                );
                let uniform_buffer_position = self.available_positions.pop_front().unwrap();

                let raw_image = RawImage2d::from_raw_rgba_reversed(&image.data, image.dimensions);
                let texture = SrgbTexture2d::new(display, raw_image).unwrap();

                entry.insert(CachedTextureMeta {
                    texture: texture.resident().unwrap(),
                    uniform_buffer_position,
                    frames_since_last_use: 0,
                    taken: true,
                });
            }
        }
    }

    pub fn get_position(&self, image_handle: &Handle<Image>) -> u32 {
        self.textures
            .get(image_handle)
            .unwrap()
            .uniform_buffer_position as u32
    }

    pub fn texture_uniform_buffer(&self) -> TextureUniformBuffer<'_> {
        let default_handle = GliumTextureHandle::new(
            &self
                .textures
                .get(&DEFAULT_IMAGE_HANDLE.typed())
                .unwrap()
                .texture,
            &Default::default(),
        );
        let mut tex = [default_handle; TEXTURE_BUFFER_SIZE];
        self.textures.values().for_each(|v| {
            tex[v.uniform_buffer_position] =
                GliumTextureHandle::new(&v.texture, &Default::default());
        });
        TextureUniformBuffer { tex }
    }

    /// Updates the cache and only retains recently used textures.
    pub fn update(&mut self) {
        for meta in self.textures.values_mut() {
            meta.frames_since_last_use += 1;
            meta.taken = false;
        }
        self.textures
            .retain(|_, meta| meta.frames_since_last_use < 3);
    }
}

/// Updates the [`TextureCache`] to only retains recently used textures.
pub fn update_texture_cache_system(mut texture_cache: NonSendMut<TextureBuffer>) {
    texture_cache.update();
}
