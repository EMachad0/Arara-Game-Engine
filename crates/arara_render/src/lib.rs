mod simple_mesh;
mod color;
mod renderer;

pub use simple_mesh::*;
pub use color::*;
pub use renderer::*;

pub mod prelude {
    pub use crate::{
        simple_mesh::SimpleMeshBundle,
        color::Color,
    };
}

#[macro_use]
extern crate glium;
extern crate arara_logger;

use bevy_ecs::prelude::*;
use arara_app::{
    app_builder::AppBuilder,
    plugin::Plugin,
    CoreStage,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    Draw,
}

#[derive(Default)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.app.schedule.add_stage_before(
            CoreStage::Update,
            RenderStage::Draw,
            SystemStage::parallel(),
        );
        
        app_builder.add_system_to_stage(RenderStage::Draw, draw.system());
    }
}
