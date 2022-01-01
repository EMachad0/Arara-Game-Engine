use arara_asset::{Assets, Handle};
use arara_ecs::{
    entity::Entity,
    system::{Commands, Query, Res},
};
use arara_transform::GlobalTransform;

use crate::{
    core_pipeline::{
        core_pipeline_entities::ExtractedCorePipelineEntity,
        phase_items::{Opaque3D, Transparent3D},
    },
    render_phase::RenderPhase,
    Color, Image, Mesh, Visibility,
};

pub(crate) fn extract_core_pipeline_phases(mut commands: Commands) {
    commands.insert_resource(RenderPhase::<Opaque3D>::default());
    commands.insert_resource(RenderPhase::<Transparent3D>::default());
}

pub(crate) fn extract_core_pipeline_entities(
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
        commands.entity(entity).insert(ExtractedCorePipelineEntity {
            mesh: mesh.clone_weak(),
            image: image.clone_weak(),
            transform: global_transform.compute_matrix(),
            color: *color,
        });
    }
}
