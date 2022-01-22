use arara_asset::{Assets, Handle};
use arara_ecs::system::{NonSend, NonSendMut, Res};
use arara_utils::tracing::trace;
use arara_utils::HashMap;
use arara_window::Window;
use glium::texture::{
    RawImage2d, ResidentTexture, SrgbTexture2d, TextureHandle as GliumTextureHandle,
};
use glium::implement_uniform_block;
use std::collections::VecDeque;

use crate::{Image, DEFAULT_IMAGE_HANDLE};

pub const TEXTURE_BUFFER_SIZE: usize = 5;

#[derive(Copy, Clone)]
pub struct TextureUniformBuffer<'a> {
    pub tex: [GliumTextureHandle<'a>; TEXTURE_BUFFER_SIZE],
}

implement_uniform_block!(TextureUniformBuffer<'a>, tex);

pub struct CachedTextureMeta {
    pub texture: Option<ResidentTexture>,
    pub uniform_buffer_position: usize,
    pub frames_since_last_use: usize,
    pub taken: bool,
}

pub struct TextureBuffer {
    available_positions: VecDeque<usize>,
    pub textures: HashMap<Handle<Image>, CachedTextureMeta>,
    queue: Vec<Handle<Image>>,
}

impl Default for TextureBuffer {
    fn default() -> Self {
        Self {
            available_positions: (0..TEXTURE_BUFFER_SIZE).collect(),
            textures: Default::default(),
            queue: Default::default(),
        }
    }
}

impl TextureBuffer {
    /// Inserts a texture that matches the `Handle`.
    /// If no matching one is found a new [`glium::TextureHandle`] is created.
    pub fn get_or_insert(&mut self, image_handle: Handle<Image>) -> usize {
        match self.textures.entry(image_handle) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let meta = entry.get_mut();
                meta.frames_since_last_use = 0;
                meta.taken = true;
                meta.uniform_buffer_position
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                trace!("inserting in texture buffer {:?}", entry);
                assert!(
                    !self.available_positions.is_empty(),
                    "
                    No texture slot avaible in the Texture Buffer
                    You are probably using more than {} textures!
                    this value can be tweaked on [arara_render::texture::texture_buffer]
                    ",
                    TEXTURE_BUFFER_SIZE
                );
                self.queue.push(entry.key().clone_weak());
                let uniform_buffer_position = self.available_positions.pop_front().unwrap();
                entry.insert(CachedTextureMeta {
                    texture: None,
                    uniform_buffer_position,
                    frames_since_last_use: 0,
                    taken: true,
                });
                uniform_buffer_position
            }
        }
    }

    pub fn texture_uniform_buffer(&self) -> TextureUniformBuffer<'_> {
        let default_handle = GliumTextureHandle::new(
            self
                .textures
                .get(&DEFAULT_IMAGE_HANDLE.typed())
                .unwrap_or_else(|| self.textures.iter().next().expect("No texture in buffer").1)
                .texture
                .as_ref()
                .unwrap(),
            &Default::default(),
        );
        let mut tex = [default_handle; TEXTURE_BUFFER_SIZE];
        self.textures.values().for_each(|v| {
            tex[v.uniform_buffer_position] =
                GliumTextureHandle::new(v.texture.as_ref().unwrap(), &Default::default());
        });
        TextureUniformBuffer { tex }
    }

    /// Updates the cache and only retains recently used textures.
    pub fn update(&mut self) {
        for meta in self.textures.values_mut() {
            meta.frames_since_last_use += 1;
            meta.taken = false;
            if meta.frames_since_last_use > 3 {
                self.available_positions.push_back(meta.uniform_buffer_position);
            }
        }
        self.textures
            .retain(|_, meta| meta.frames_since_last_use <= 3);
    }
}

/// Add queued to the gpus
pub fn process_queue_to_gpu(
    mut texture_cache: NonSendMut<TextureBuffer>,
    window: NonSend<Window>,
    images: Res<Assets<Image>>,
) {
    let display = window.display();
    let handles: Vec<_> = texture_cache.queue.drain(..).collect();
    for image_handle in handles {
        let image = images.get(&image_handle).unwrap();
        let raw_image = RawImage2d::from_raw_rgba_reversed(&image.data, image.dimensions);
        let texture = SrgbTexture2d::new(display, raw_image).unwrap();
        let texture = texture.resident().unwrap();

        let meta = texture_cache.textures.get_mut(&image_handle).unwrap();
        meta.texture = Some(texture);
    }
}

/// Updates the [`TextureCache`] to only retains recently used textures.
pub fn update_texture_cache_system(mut texture_cache: NonSendMut<TextureBuffer>) {
    texture_cache.update();
}
