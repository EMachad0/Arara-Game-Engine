use arara_ecs::{
    event::Events,
    prelude::*,
    schedule::{IntoSystemDescriptor, RunOnce},
    system::Resource,
};
use arara_utils::tracing::debug;

use crate::{
    app::App,
    plugin::{Plugin, PluginGroup, PluginGroupBuilder},
    AppExit, CoreStage, StartupStage,
};

pub struct AppBuilder {
    pub app: App,
}

impl Default for AppBuilder {
    fn default() -> Self {
        let mut app_builder = Self {
            app: App::default(),
        };
        app_builder.add_core_stages().add_event::<AppExit>();
        app_builder
    }
}

impl AppBuilder {
    pub fn build(&mut self) -> App {
        std::mem::take(&mut self.app)
    }

    
}
