mod app;
mod app_builder;
mod plugin;

pub use app::*;
pub use app_builder::*;
pub use plugin::*;

pub mod prelude {
    pub use crate::{
        app::App,
        app_builder::AppBuilder,
        plugin::{Plugin, PluginGroup, PluginGroupBuilder},
        CoreStage, StartupStage,
    };
}

#[macro_use]
extern crate arara_logger;

use bevy_ecs::schedule::StageLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    First,
    Startup,
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum StartupStage {
    Startup,
}