use crate::{
    render_phase::{CachedPipelinePhaseItem, DrawFunctionId, EntityPhaseItem, PhaseItem},
    CachedPipelineId,
};
use arara_ecs::prelude::Entity;
use arara_utils::FloatOrd;

pub struct Opaque3D {
    pub distance: f32,
    pub entity: Entity,
    pub draw_function: DrawFunctionId,
    pub pipeline: CachedPipelineId,
}

impl PhaseItem for Opaque3D {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }

    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }
}

impl EntityPhaseItem for Opaque3D {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl CachedPipelinePhaseItem for Opaque3D {
    fn cached_pipeline(&self) -> CachedPipelineId {
        self.pipeline
    }
}

pub struct Transparent3D {
    pub distance: f32,
    pub entity: Entity,
    pub draw_function: DrawFunctionId,
    pub pipeline: CachedPipelineId,
}

impl PhaseItem for Transparent3D {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }

    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }
}

impl EntityPhaseItem for Transparent3D {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl CachedPipelinePhaseItem for Transparent3D {
    fn cached_pipeline(&self) -> CachedPipelineId {
        self.pipeline
    }
}
