mod color;
mod clear_color;
mod coordinate_system;
mod texture;
mod shader;
mod shaders;
mod visibility;
mod geometry;
mod render_phase;
mod core_pipeline;
mod billboard;

pub use color::*;
pub use clear_color::*;
pub use coordinate_system::*;
pub use texture::*;
pub use shader::*;
pub use visibility::*;
pub use geometry::*;
pub use core_pipeline::*;
pub use billboard::*;

pub mod prelude {
    pub use crate::{
        RenderPlugin,
        core_pipeline::{SimpleMeshBundle, BPLight},
        geometry::*,
        color::Color,
        clear_color::ClearColor,
        coordinate_system::{CoordinateSystem, CoordinateSystemPlugin},
        texture::Image,
        visibility::Visibility,
        billboard::Billboard,
    };
}

#[macro_use]
extern crate arara_logger;

use bevy_ecs::prelude::*;
use arara_app::{AppBuilder, CoreStage, Plugin};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    /// Extract data from the "app world" and insert it into the "render world".
    /// This step should be kept as short as possible to increase the "pipelining potential" for
    /// running the next frame while rendering the current frame.
    Extract,

    /// Prepare render resources from the extracted data for the GPU.
    Prepare,

    /// Create [`BindGroups`](crate::render_resource::BindGroup) that depend on
    /// [`Prepare`](RenderStage::Prepare) data and queue up draw calls to run during the
    /// [`Render`](RenderStage::Render) stage.
    Queue,

    /// Sort the [`RenderPhases`](crate::render_phase::RenderPhase) here.
    PhaseSort,

    /// Actual rendering happens here.
    /// In most cases, only the render backend should insert resources here.
    Render,

    /// Cleanup render resources here.
    Cleanup,
}

#[derive(Default)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.app.schedule
            .add_stage_before(CoreStage::PreUpdate, RenderStage::Extract, SystemStage::parallel())
            .add_stage_after(RenderStage::Extract, RenderStage::Prepare, SystemStage::parallel())
            .add_stage_after(RenderStage::Prepare, RenderStage::Queue, SystemStage::parallel())
            .add_stage_after(RenderStage::Queue, RenderStage::PhaseSort, SystemStage::parallel())
            .add_stage_after(
                RenderStage::PhaseSort, RenderStage::Render,
                SystemStage::parallel().with_system(main_pass.exclusive_system().at_end().label("MainPass"))
            )
            .add_stage_after(RenderStage::Render, RenderStage::Cleanup, SystemStage::parallel());
        
        app_builder
            .add_plugin(shader::ShaderPlugin)
            .add_plugin(geometry::MeshPlugin)
            .add_plugin(texture::ImagePlugin)
            .add_plugin(core_pipeline::CorePipelinePlugin)
            .add_plugin(billboard::BillboardPlugin);
    }
}
