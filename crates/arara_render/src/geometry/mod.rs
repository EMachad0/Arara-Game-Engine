mod shape;
mod mesh;

pub use shape::*;
pub use mesh::*;

use arara_app::{Plugin, AppBuilder};
use arara_asset::AddAsset;

/// Adds the [`Mesh`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Mesh>();
    }
}
