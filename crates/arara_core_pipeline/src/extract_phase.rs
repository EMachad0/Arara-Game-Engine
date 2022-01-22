use arara_asset::{Assets, Handle};
use arara_ecs::{
    query::With,
    system::{Query, Res, ResMut},
};
use arara_render::{Color, Image, Mesh, Visibility};
use arara_transform::GlobalTransform;

use crate::core_pipeline_entities::{CorePipelineEntity, ExtractedCorePipelineEntity};

#[derive(Default)]
pub struct ExtractedCorePipelineEntitys {
    pub items: Vec<ExtractedCorePipelineEntity>,
}

pub(crate) fn extract_core_pipeline_entities(
    mut extracts: ResMut<ExtractedCorePipelineEntitys>,
    meshes: Res<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    query: Query<
        (
            &Handle<Mesh>,
            &Handle<Image>,
            &GlobalTransform,
            &Color,
            &Visibility,
        ),
        With<CorePipelineEntity>,
    >,
) {
    extracts.items.clear();
    for (mesh, image, global_transform, color, visibility) in query.iter() {
        if !visibility.active || !visibility.visible {
            continue;
        }
        if meshes.get(mesh).is_none() || images.get(image).is_none() {
            continue;
        }
        extracts.items.push(ExtractedCorePipelineEntity {
            mesh: mesh.clone_weak(),
            image: image.clone_weak(),
            transform: global_transform.compute_matrix(),
            color: *color,
        });
    }
}
