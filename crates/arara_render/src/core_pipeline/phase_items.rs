use crate::{render_phase::{PhaseItem, EntityPhaseItem, CachedPipelinePhaseItem}, CachedPipelineId};
use arara_ecs::prelude::Entity;
use arara_utils::FloatOrd;

pub struct Opaque {
    pub distance: f32,
    pub entity: Entity,
    pub pipeline: CachedPipelineId,
}

impl PhaseItem for Opaque {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }
}

impl EntityPhaseItem for Opaque {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl CachedPipelinePhaseItem for Opaque {
    fn cached_pipeline(&self) -> CachedPipelineId {
        self.pipeline
    }
}

pub struct Transparent {
    pub distance: f32,
    pub entity: Entity,
    pub pipeline: CachedPipelineId,
}

impl PhaseItem for Transparent {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }
}

impl EntityPhaseItem for Transparent {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl CachedPipelinePhaseItem for Transparent {
    fn cached_pipeline(&self) -> CachedPipelineId {
        self.pipeline
    }
}
