mod mesh;
mod shape;

pub use mesh::*;
pub use shape::*;

use arara_app::{AppBuilder, Plugin};
use arara_asset::AddAsset;

/// Adds the [`Mesh`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Mesh>();
    }
}
