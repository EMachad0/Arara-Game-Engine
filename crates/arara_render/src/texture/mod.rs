pub(crate) mod converters;
mod image;
mod image_texture_loader;

pub use self::image::*;
use arara_app::{App, Plugin};
use arara_asset::AddAsset;
pub use image_texture_loader::*;

/// Adds the [`Image`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<ImageTextureLoader>()
            .add_asset::<Image>();
    }
}
