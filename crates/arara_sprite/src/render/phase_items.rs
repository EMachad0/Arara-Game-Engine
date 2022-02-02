use arara_ecs::prelude::Entity;
use arara_render::{
    CachedPipelineId, CachedPipelinePhaseItem, DrawFunctionId, EntityPhaseItem, PhaseItem,
};
use arara_utils::FloatOrd;

pub struct Transparent2D {
    pub distance: f32,
    pub entity: Entity,
    pub draw_function: DrawFunctionId,
    pub pipeline: CachedPipelineId,
}

impl PhaseItem for Transparent2D {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }

    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }
}

impl EntityPhaseItem for Transparent2D {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl CachedPipelinePhaseItem for Transparent2D {
    fn cached_pipeline(&self) -> CachedPipelineId {
        self.pipeline
    }
}
