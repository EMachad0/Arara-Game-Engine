mod main_pass;
mod phase_items;
mod prepare_phase;
mod simple_mesh;

use arara_app::prelude::*;
use arara_window::Window;
use bevy_ecs::prelude::*;
use glium::{Api, Profile, Version};
pub use main_pass::main_pass;
pub use phase_items::*;
pub use prepare_phase::prepare_core_pass;
pub use simple_mesh::*;

use crate::{
    render_phase::{sort_phase_system, RenderPhase},
    ClearColor, RenderStage,
};

#[derive(Default)]
pub struct CorePipelinePlugin;

impl Plugin for CorePipelinePlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .init_resource::<ClearColor>()
            .init_resource::<BPLight>()
            .init_resource::<DefaultShader>()
            .add_startup_system_to_stage(StartupStage::PreStartup, load_default_shader.system())
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                debug_glium_backend_info.system(),
            )
            .add_system_to_stage(
                RenderStage::Extract,
                extract_core_pipeline_camera_phases.system(),
            )
            .add_system_to_stage(RenderStage::Prepare, prepare_core_pass.system())
            .add_system_to_stage(
                RenderStage::PhaseSort,
                sort_phase_system::<Transparent>.system(),
            );
        // .add_system_to_stage(RenderStage::PhaseSort, sort_phase_system::<Opaque>.system());
    }
}

fn extract_core_pipeline_camera_phases(mut commands: Commands) {
    commands.insert_resource(RenderPhase::<Opaque>::default());
    commands.insert_resource(RenderPhase::<Transparent>::default());
}

fn debug_glium_backend_info(window: NonSend<Window>) {
    let display = window.display();

    let version = *display.get_opengl_version();
    let api = match version {
        Version(Api::Gl, _, _) => "OpenGL",
        Version(Api::GlEs, _, _) => "OpenGL ES",
    };

    info!(
        "{} context version: {}",
        api,
        display.get_opengl_version_string()
    );

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
        info!(
            "{} profile mask: {}",
            api,
            match display.get_opengl_profile() {
                Some(Profile::Core) => "core",
                Some(Profile::Compatibility) => "compatibility",
                None => "unknown",
            }
        );
    }

    info!(
        "{} robustness strategy: {}",
        api,
        if display.is_context_loss_possible() {
            "lose"
        } else {
            "none"
        }
    );

    info!(
        "{} context vendor: {}",
        api,
        display.get_opengl_vendor_string()
    );
    info!(
        "{} context renderer: {}",
        api,
        display.get_opengl_renderer_string()
    );
}
