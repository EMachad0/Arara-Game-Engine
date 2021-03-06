mod app;
mod plugin;

pub use app::*;
pub use plugin::*;

pub mod prelude {
    pub use crate::{
        app::App,
        plugin::{Plugin, PluginGroup, PluginGroupBuilder},
        CoreStage, StartupStage,
    };
}

use arara_ecs::schedule::StageLabel;

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
