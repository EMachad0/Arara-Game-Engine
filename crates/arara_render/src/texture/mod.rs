mod image;
mod image_texture_loader;
pub(crate) mod converters;

pub use self::image::*;
pub use image_texture_loader::*;
use arara_app::{AppBuilder, Plugin};
use arara_asset::AddAsset;

/// Adds the [`Image`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_asset_loader::<ImageTextureLoader>()
            .add_asset::<Image>();
    }
}