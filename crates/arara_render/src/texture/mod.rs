pub(crate) mod converters;
mod image;
mod image_texture_loader;
mod texture_buffer;

use crate::RenderStage;

pub use self::image::*;
use arara_app::{App, Plugin};
use arara_asset::{AddAsset, Assets, HandleUntyped};
use bevy_reflect::TypeUuid;
pub use image_texture_loader::*;
pub use texture_buffer::*;

pub const DEFAULT_IMAGE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 13148262314052771789);

/// Adds the [`Image`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<ImageTextureLoader>()
            .add_asset::<Image>();
        app.world
            .get_resource_mut::<Assets<Image>>()
            .unwrap()
            .set_untracked(DEFAULT_IMAGE_HANDLE, Image::default());
        app.init_non_send_resource::<TextureBuffer>()
            .add_system_to_stage(RenderStage::Cleanup, update_texture_cache_system);
    }
}
