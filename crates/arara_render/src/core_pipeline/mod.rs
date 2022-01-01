mod core_pipeline_entities;
mod draw_functions;
mod extract_phase;
mod phase_items;
mod pipelines;
mod prepare_phase;
mod queue_phase;

use arara_app::{App, Plugin, StartupStage};
use arara_ecs::system::{NonSend, IntoExclusiveSystem, Commands, Res};
use arara_utils::tracing::info;
use arara_window::Window;
pub use core_pipeline_entities::{BPLight, SimpleMeshBundle};
use draw_functions::DrawSimpleMesh;
use extract_phase::{extract_core_pipeline_entities, extract_core_pipeline_phases};
use glium::{Api, Profile, Version};
pub use phase_items::{Opaque3D, Transparent3D};
pub use pipelines::{CorePipeline, DefaultShader};
use prepare_phase::{prepare_bindless_textures, prepare_core_pipeline_phase};
use queue_phase::queue_core_pipeline_phase;

use crate::{DrawFunctions, RenderStage, SpecializedPipelines, RenderPhase};

#[derive(Default)]
pub struct CorePipelinePlugin;

impl Plugin for CorePipelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BPLight>()
            .init_resource::<CorePipeline>()
            .init_resource::<SpecializedPipelines<CorePipeline>>()
            .init_resource::<DrawFunctions<Opaque3D>>()
            .init_resource::<DrawFunctions<Transparent3D>>()
            .add_startup_system_to_stage(StartupStage::PostStartup, debug_glium_backend_info)
            .add_system_to_stage(RenderStage::Extract, extract_core_pipeline_phases)
            .add_system_to_stage(RenderStage::Extract, extract_core_pipeline_entities)
            .add_system_to_stage(RenderStage::Prepare, prepare_bindless_textures.exclusive_system())
            .add_system_to_stage(RenderStage::Prepare, prepare_core_pipeline_phase)
            .add_system_to_stage(RenderStage::Queue, queue_core_pipeline_phase)
            .add_system_to_stage(RenderStage::Cleanup, clear_core_pipeline_entities);

        let draw_simple_mesh = DrawSimpleMesh::new(&mut app.world);
        app.world
            .get_resource::<DrawFunctions<Opaque3D>>()
            .unwrap()
            .write()
            .add(draw_simple_mesh);
    }
}

fn clear_core_pipeline_entities(mut commands: Commands, phase: Res<RenderPhase<Opaque3D>>) {
    for item in phase.items.iter() {
        commands.entity(item.entity).despawn();
    }
}

fn debug_glium_backend_info(window: NonSend<Window>) {
    let glium_display = window.display();

    let version = *glium_display.get_opengl_version();
    let api = match version {
        Version(Api::Gl, _, _) => "OpenGL",
        Version(Api::GlEs, _, _) => "OpenGL ES",
    };

    info!(
        "{} context version: {}",
        api,
        glium_display.get_opengl_version_string()
    );

    info!("{} context flags:", api);
    if glium_display.is_forward_compatible() {
        info!("\tforward-compatible");
    }
    if glium_display.is_debug() {
        info!("\tdebug");
    }
    if glium_display.is_robust() {
        info!("\trobustness");
    }

    if version >= Version(Api::Gl, 3, 2) {
        info!(
            "{} profile mask: {}",
            api,
            match glium_display.get_opengl_profile() {
                Some(Profile::Core) => "core",
                Some(Profile::Compatibility) => "compatibility",
                None => "unknown",
            }
        );
    }

    info!(
        "{} robustness strategy: {}",
        api,
        if glium_display.is_context_loss_possible() {
            "lose"
        } else {
            "none"
        }
    );

    info!(
        "{} context vendor: {}",
        api,
        glium_display.get_opengl_vendor_string()
    );
    info!(
        "{} context renderer: {}",
        api,
        glium_display.get_opengl_renderer_string()
    );
}
