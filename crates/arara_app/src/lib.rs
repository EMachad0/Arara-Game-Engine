mod app;
mod app_builder;
mod plugin;
mod event;

pub use app::*;
pub use app_builder::*;
pub use plugin::*;
pub use event::*;

pub mod prelude {
    pub use crate::{
        app::App,
        app_builder::AppBuilder,
        plugin::{Plugin, PluginGroup, PluginGroupBuilder},
        event::*,
        CoreStage, StartupStage,
    };
}

#[macro_use]
extern crate arara_logger;

use bevy_ecs::schedule::StageLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    First,
    EventUpdateStage,
    Startup,
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum StartupStage {
    PreStartup,
    Startup,
    PostStartup,
}

/// An event that indicates the app should exit. This will fully exit the app process.
#[derive(Debug, Clone)]
pub struct AppExit;
