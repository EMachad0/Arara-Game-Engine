// use crate::render_phase::draw::DrawFunctionId;
use arara_ecs::{prelude::ResMut, entity::Entity};

use crate::CachedPipelineId;

/// An item which will be drawn to the screen. A phase item should be queued up for rendering
/// during the [`RenderStage::Queue`](crate::RenderStage::Queue) stage.
/// Afterwards it will be sorted and rendered automatically  in the
/// [`RenderStage::PhaseSort`](crate::RenderStage::PhaseSort) stage and
/// [`RenderStage::Render`](crate::RenderStage::Render) stage, respectively.
pub trait PhaseItem: Send + Sync + 'static {
    /// The type used for ordering the items. The smallest values are drawn first.
    type SortKey: Ord;
    /// Determines the order in which the items are drawn during the corresponding [`RenderPhase`].
    fn sort_key(&self) -> Self::SortKey;
    // Specifies the [`Draw`] function used to render the item.
    // fn draw_function(&self) -> DrawFunctionId;
}

pub trait EntityPhaseItem: PhaseItem {
    fn entity(&self) -> Entity;
}

pub trait CachedPipelinePhaseItem: PhaseItem {
    fn cached_pipeline(&self) -> CachedPipelineId;
}

/// A resource to collect and sort draw requests for specific [`PhaseItems`](PhaseItem).
pub struct RenderPhase<I: PhaseItem> {
    pub items: Vec<I>,
}

impl<I: PhaseItem> Default for RenderPhase<I> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<I: PhaseItem> RenderPhase<I> {
    /// Adds a [`PhaseItem`] to this render phase.
    #[inline]
    pub fn add(&mut self, item: I) {
        self.items.push(item);
    }

    /// Sorts all of its [`PhaseItems`](PhaseItem).
    pub fn sort(&mut self) {
        self.items.sort_by_key(|d| d.sort_key());
    }
}

/// This system sorts all [`RenderPhases`](RenderPhase) for the [`PhaseItem`] type.
pub fn sort_phase_system<I: PhaseItem>(mut render_phase: ResMut<RenderPhase<I>>) {
    render_phase.sort();
}
