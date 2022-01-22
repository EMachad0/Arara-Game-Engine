use arara_ecs::prelude::*;
use arara_render::{DrawFunctions, RenderPhase, RenderPipelineCache, SpecializedPipelines};

use crate::render::{
    draw_function::DrawSprite,
    phase_items::Transparent2D,
    pipelines::{SpritePipeline, SpritePipelineKey},
    prepare_phase::SpriteBatch,
};

pub(crate) fn queue_sprite_phase(
    mut phase: ResMut<RenderPhase<Transparent2D>>,
    query: Query<Entity, With<SpriteBatch>>,
    mut render_pipeline_cache: NonSendMut<RenderPipelineCache>,
    pipeline: Res<SpritePipeline>,
    mut pipelines: ResMut<SpecializedPipelines<SpritePipeline>>,
    draw_functions: Res<DrawFunctions<Transparent2D>>,
) {
    if query.is_empty() {
        return;
    }

    let sprite_pipeline =
        pipelines.specialize(&mut render_pipeline_cache, &pipeline, SpritePipelineKey);

    let draw_sprite_function = draw_functions.read().get_id::<DrawSprite>().unwrap();

    for entity in query.iter() {
        phase.add(Transparent2D {
            distance: 0.0,
            entity,
            draw_function: draw_sprite_function,
            pipeline: sprite_pipeline,
        });
    }
}
