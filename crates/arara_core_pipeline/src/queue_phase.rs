use arara_ecs::prelude::*;
use arara_render::{DrawFunctions, RenderPhase, RenderPipelineCache};

use crate::{
    draw_functions::DrawSimpleMesh, pipelines::CorePipelineKey, prepare_phase::CorePipelineBatch,
    CorePipeline, Opaque3D, SpecializedPipelines, Transparent3D,
};

pub(crate) fn queue_core_pipeline_phase(
    mut opaques: ResMut<RenderPhase<Opaque3D>>,
    mut transparents: ResMut<RenderPhase<Transparent3D>>,
    query: Query<(Entity, &CorePipelineBatch)>,
    mut render_pipeline_cache: NonSendMut<RenderPipelineCache>,
    pipeline: Res<CorePipeline>,
    mut pipelines: ResMut<SpecializedPipelines<CorePipeline>>,
    opaque_draw_functions: Res<DrawFunctions<Opaque3D>>,
    transparent_draw_functions: Res<DrawFunctions<Transparent3D>>,
) {
    if query.is_empty() {
        return;
    }

    let opaque_pipeline = pipelines.specialize(
        &mut render_pipeline_cache,
        &pipeline,
        CorePipelineKey { transparent: false },
    );
    let transparent_pipeline = pipelines.specialize(
        &mut render_pipeline_cache,
        &pipeline,
        CorePipelineKey { transparent: true },
    );

    let draw_opaque_function = opaque_draw_functions
        .read()
        .get_id::<DrawSimpleMesh>()
        .unwrap();
    let draw_transparent_function = transparent_draw_functions
        .read()
        .get_id::<DrawSimpleMesh>()
        .unwrap();

    for (entity, batch) in query.iter() {
        if batch.transparent {
            transparents.add(Transparent3D {
                distance: 0.0,
                entity,
                draw_function: draw_transparent_function,
                pipeline: transparent_pipeline,
            });
        } else {
            opaques.add(Opaque3D {
                distance: 0.0,
                entity,
                draw_function: draw_opaque_function,
                pipeline: opaque_pipeline,
            });
        }
    }
}
