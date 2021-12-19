mod components;
mod hierarchy;
mod transform_propagate_system;

pub use components::*;
pub use hierarchy::*;
pub use transform_propagate_system::*;

pub mod prelude {
    pub use crate::{components::*, hierarchy::*, TransformPlugin};
}

use arara_app::prelude::*;
use bevy_ecs::{
    schedule::{ParallelSystemDescriptorCoercion, SystemLabel},
    system::IntoSystem,
};

#[derive(Default)]
pub struct TransformPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum TransformSystem {
    TransformPropagate,
    ParentUpdate,
}

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // add transform systems to startup so the first update is "correct"
        app.add_startup_system_to_stage(
            StartupStage::PostStartup,
            parent_update_system
                .system()
                .label(TransformSystem::ParentUpdate),
        )
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            transform_propagate_system::transform_propagate_system
                .system()
                .label(TransformSystem::TransformPropagate)
                .after(TransformSystem::ParentUpdate),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            parent_update_system
                .system()
                .label(TransformSystem::ParentUpdate),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            transform_propagate_system::transform_propagate_system
                .system()
                .label(TransformSystem::TransformPropagate)
                .after(TransformSystem::ParentUpdate),
        );
    }
}
