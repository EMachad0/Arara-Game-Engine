pub mod app;
pub mod app_builder;
pub mod plugin;

pub use app::*;
pub use app_builder::*;
pub use plugin::*;

pub mod prelude {
    pub use crate::{
        app::App,
        app_builder::AppBuilder,
        plugin::Plugin,
        CoreStage, StartupStage,
    };
}

use bevy_ecs::schedule::StageLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    First,
    Startup,
    Update,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum StartupStage {
    Startup,
}