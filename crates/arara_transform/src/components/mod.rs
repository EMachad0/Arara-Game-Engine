mod children;
mod global_transform;
mod parent;
mod transform;

use arara_ecs::prelude::Bundle;
pub use children::Children;
pub use global_transform::*;
pub use parent::{Parent, PreviousParent};
pub use transform::*;

/// Simple Bundle for making entities visible
/// See [Transform]
#[derive(Bundle, Debug, Default)]
pub struct TransformBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl TransformBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            global_transform: Default::default(),
        }
    }
}
