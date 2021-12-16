mod shader;
mod shader_loader;

use arara_app::{AppBuilder, Plugin};
use arara_asset::AddAsset;
pub use shader::Shader;
pub use shader_loader::ShaderLoader;

/// Adds the [`Shader`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_asset_loader::<ShaderLoader>()
            .add_asset::<Shader>();
    }
}