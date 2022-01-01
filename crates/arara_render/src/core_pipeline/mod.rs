mod main_pass;
mod phase_items;
mod pipelines;
mod prepare_phase;
mod simple_mesh;

use arara_app::prelude::*;
use arara_asset::{Assets, Handle};
use arara_ecs::prelude::*;
use arara_transform::GlobalTransform;
use arara_utils::tracing::info;
use arara_window::Window;
use glium::{Api, Profile, Version};
pub use main_pass::main_pass;
pub use phase_items::*;
pub use prepare_phase::{prepare_bindless_textures, prepare_split_render_phase};
pub use simple_mesh::*;
pub use pipelines::*;

use crate::{
    render_phase::{sort_phase_system, RenderPhase},
    ClearColor, Color, Image, Mesh, RenderStage, SpecializedPipelines, Visibility,
};

#[derive(Default)]
pub struct CorePipelinePlugin;

impl Plugin for CorePipelinePlugin {
    fn build(&self, app_builder: &mut App) {
        app_builder
            .init_resource::<ClearColor>()
            .init_resource::<BPLight>()
            .init_resource::<DefaultShader>()
            .init_resource::<OpaquePipeline>()
            .init_resource::<SpecializedPipelines<OpaquePipeline>>()
            .init_resource::<TransparentPipeline>()
            .init_resource::<SpecializedPipelines<TransparentPipeline>>()
            .add_startup_system_to_stage(StartupStage::PostStartup, debug_glium_backend_info)
            .add_system_to_stage(RenderStage::Extract, extract_core_pipeline_camera_phases)
            .add_system_to_stage(RenderStage::Extract, extract_core_pipeline_entities)
            .add_system_to_stage(RenderStage::Prepare, prepare_bindless_textures)
            .add_system_to_stage(RenderStage::Prepare, prepare_split_render_phase)
            .add_system_to_stage(RenderStage::PhaseSort, sort_phase_system::<Transparent>);
        // .add_system_to_stage(RenderStage::PhaseSort, sort_phase_system::<Opaque>);
    }
}

fn extract_core_pipeline_camera_phases(mut commands: Commands) {
    commands.insert_resource(RenderPhase::<Opaque>::default());
    commands.insert_resource(RenderPhase::<Transparent>::default());
}

fn extract_core_pipeline_entities(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    query: Query<(
        Entity,
        &Handle<Mesh>,
        &Handle<Image>,
        &GlobalTransform,
        &Color,
        &Visibility,
    )>,
) {
    for (entity, mesh, image, global_transform, color, visibility) in query.iter() {
        if !visibility.active || !visibility.visible {
            continue;
        }
        if meshes.get(mesh).is_none() || images.get(image).is_none() {
            continue;
        }
        commands.entity(entity).insert(ExtractedCPE {
            mesh: mesh.clone_weak(),
            image: image.clone_weak(),
            transform: global_transform.compute_matrix(),
            color: *color,
        });
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
