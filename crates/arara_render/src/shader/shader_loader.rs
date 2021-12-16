use arara_asset::{AssetLoader, LoadContext, BoxedFuture, LoadedAsset};
use glium::program::ShaderStage;

use crate::Shader;

#[derive(Default)]
pub struct ShaderLoader;

impl AssetLoader for ShaderLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let ext = load_context.path().extension().unwrap().to_str().unwrap();

            let shader = match ext {
                "vert" => Shader::from_glsl(
                    String::from_utf8(Vec::from(bytes))?,
                    ShaderStage::Vertex,
                ),
                "frag" => Shader::from_glsl(
                    String::from_utf8(Vec::from(bytes))?,
                    ShaderStage::Fragment,
                ),
                _ => panic!("unhandled extension: {}", ext),
            };

            load_context.set_default_asset(LoadedAsset::new(shader));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vert", "frag"]
    }
}