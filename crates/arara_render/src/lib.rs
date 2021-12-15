mod simple_mesh;
mod color;
mod clear_color;
mod coordinate_system;
mod texture;
mod shaders;
mod visibility;
mod geometry;
mod draw_phase;
// mod prepare_phase;

pub use simple_mesh::*;
pub use color::*;
pub use clear_color::*;
pub use coordinate_system::*;
pub use texture::*;
pub use shaders::*;
pub use visibility::*;
pub use geometry::*;
use draw_phase::*;
// use prepare_phase::*;

pub mod prelude {
    pub use crate::{
        RenderPlugin,
        simple_mesh::{SimpleMeshBundle, BPLight},
        geometry::*,
        color::Color,
        clear_color::ClearColor,
        coordinate_system::{CoordinateSystem, CoordinateSystemPlugin},
        texture::Image,
        visibility::Visibility,
    };
}

#[macro_use]
extern crate arara_logger;
use glium::*;
use arara_window::Window;

use bevy_ecs::prelude::*;
use arara_app::{AppBuilder, CoreStage, Plugin, StartupStage};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    Prepare,
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
            .add_plugin(geometry::MeshPlugin)
            .add_plugin(texture::ImagePlugin)
            .add_startup_system_to_stage(StartupStage::PostStartup, debug_glium_backend_info.system())
            .add_system_to_stage(RenderStage::Draw, main_pass.system().label("MainPass"));
            // .add_system_to_stage(RenderStage::Draw, translucent_pass.system().after("MainPass"));
    }
}


fn debug_glium_backend_info(window: NonSend<Window>) {
    let display = window.display();

    let version = *display.get_opengl_version();
    let api = match version {
        Version(Api::Gl, _, _) => "OpenGL",
        Version(Api::GlEs, _, _) => "OpenGL ES"
    };

    info!("{} context version: {}", api, display.get_opengl_version_string());

    info!("{} context flags:", api);
    if display.is_forward_compatible() {
        info!("\tforward-compatible");
    }
    if display.is_debug() {
        info!("\tdebug");
    }
    if display.is_robust() {
        info!("\trobustness");
    }

    if version >= Version(Api::Gl, 3, 2) {
        info!("{} profile mask: {}", api,
            match display.get_opengl_profile() {
                Some(Profile::Core) => "core",
                Some(Profile::Compatibility) => "compatibility",
                None => "unknown"
            });
    }

    info!("{} robustness strategy: {}", api,
        if display.is_context_loss_possible() {
            "lose"
        } else {
            "none"
        });

    info!("{} context vendor: {}", api, display.get_opengl_vendor_string());
    info!("{} context renderer: {}", api, display.get_opengl_renderer_string());
}
