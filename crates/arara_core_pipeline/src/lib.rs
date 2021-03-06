mod coordinate_system;
mod core_pipeline_entities;
mod draw_functions;
mod extract_phase;
mod phase_items;
mod pipelines;
mod prepare_phase;
mod queue_phase;

use arara_app::{App, Plugin, StartupStage};
use arara_ecs::system::NonSend;
use arara_render::{
    clear_phase_system, DrawFunctions, RenderPhase, RenderPhases, RenderStage, SpecializedPipelines,
};
use arara_utils::tracing::info;
use arara_window::Window;
pub use coordinate_system::{CoordinateSystem, CoordinateSystemPlugin};
pub use core_pipeline_entities::{BPLight, SimpleMeshBundle};
use draw_functions::DrawSimpleMesh;
use extract_phase::{extract_core_pipeline_entities, ExtractedCorePipelineEntitys};
use glium::{Api, Profile, Version};
pub use phase_items::{Opaque3D, Transparent3D};
pub use pipelines::{CorePipeline, DefaultShader};
use prepare_phase::prepare_core_pipeline_phase;
use queue_phase::queue_core_pipeline_phase;

#[derive(Default)]
pub struct CorePipelinePlugin;

impl Plugin for CorePipelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BPLight>()
            .init_resource::<CorePipeline>()
            .init_resource::<SpecializedPipelines<CorePipeline>>()
            .init_resource::<DrawFunctions<Opaque3D>>()
            .init_resource::<DrawFunctions<Transparent3D>>()
            .init_resource::<RenderPhase<Opaque3D>>()
            .init_resource::<RenderPhase<Transparent3D>>()
            .init_resource::<ExtractedCorePipelineEntitys>()
            .add_startup_system_to_stage(StartupStage::PostStartup, debug_glium_backend_info)
            .add_system_to_stage(RenderStage::Extract, extract_core_pipeline_entities)
            .add_system_to_stage(RenderStage::Prepare, prepare_core_pipeline_phase)
            .add_system_to_stage(RenderStage::Queue, queue_core_pipeline_phase)
            .add_system_to_stage(RenderStage::Cleanup, clear_phase_system::<Opaque3D>)
            .add_system_to_stage(RenderStage::Cleanup, clear_phase_system::<Transparent3D>);

        let draw_simple_mesh = DrawSimpleMesh::new(&mut app.world);
        app.world
            .get_resource::<DrawFunctions<Opaque3D>>()
            .unwrap()
            .write()
            .add(draw_simple_mesh);
        let draw_simple_mesh = DrawSimpleMesh::new(&mut app.world);
        app.world
            .get_resource::<DrawFunctions<Transparent3D>>()
            .unwrap()
            .write()
            .add(draw_simple_mesh);

        app.world
            .get_resource_mut::<RenderPhases>()
            .unwrap()
            .add::<Opaque3D>()
            .add::<Transparent3D>();
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
