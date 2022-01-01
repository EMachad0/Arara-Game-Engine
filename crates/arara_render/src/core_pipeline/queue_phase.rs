use arara_ecs::prelude::*;

use crate::{
    core_pipeline::{
        draw_functions::DrawSimpleMesh, pipelines::CorePipelineKey,
        prepare_phase::CorePipelineBatch,
    },
    render_phase::{DrawFunctions, RenderPhase},
    CorePipeline, Opaque3D, RenderPipelineCache, SpecializedPipelines,
};

pub(crate) fn queue_core_pipeline_phase(
    mut opaques: ResMut<RenderPhase<Opaque3D>>,
    query: Query<(Entity, With<CorePipelineBatch>)>,
    mut render_pipeline_cache: NonSendMut<RenderPipelineCache>,
    pipeline: Res<CorePipeline>,
    mut pipelines: ResMut<SpecializedPipelines<CorePipeline>>,
    opaque_draw_functions: Res<DrawFunctions<Opaque3D>>,
) {
    let opaque_pipeline = pipelines.specialize(
        &mut render_pipeline_cache,
        &pipeline,
        CorePipelineKey { transparent: false },
    );

    for (entity, _) in query.iter() {
        let draw_opaque_function = opaque_draw_functions
            .read()
            .get_id::<DrawSimpleMesh>()
            .unwrap();

        opaques.add(Opaque3D {
            distance: 0.0,
            entity,
            draw_function: draw_opaque_function,
            pipeline: opaque_pipeline,
        });
    }
}
