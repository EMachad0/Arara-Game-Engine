mod simple_mesh;
mod color;
mod renderer;
mod clear_color;

pub use simple_mesh::*;
pub use color::*;
pub use renderer::*;
pub use clear_color::*;

pub mod prelude {
    pub use crate::{
        RenderPlugin,
        simple_mesh::{SimpleMeshBundle, BPLight},
        color::Color,
        clear_color::ClearColor,
    };
}

#[macro_use]
extern crate glium;
extern crate arara_logger;

use bevy_ecs::prelude::*;
use arara_app::{
    AppBuilder,
    Plugin,
    CoreStage,
};

use arara_camera::CameraController;

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
        
        app_builder
            .init_resource::<ClearColor>()
            .init_resource::<BPLight>()
            .init_resource::<CameraController>()
            .add_system_to_stage(RenderStage::Draw, draw.system());
    }
}
