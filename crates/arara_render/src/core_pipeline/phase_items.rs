use crate::render_phase::PhaseItem;
use arara_utils::FloatOrd;
use bevy_ecs::prelude::Entity;

pub struct Opaque {
    pub distance: f32,
    pub entity: Entity,
}

impl PhaseItem for Opaque {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }
}

pub struct Transparent {
    pub distance: f32,
    pub entity: Entity,
}

impl PhaseItem for Transparent {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }
}
