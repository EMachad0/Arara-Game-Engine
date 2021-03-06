use arara_app::{App, Plugin};
use arara_ecs::{
    system::{IntoExclusiveSystem, ResMut},
    world::World,
};
use arara_utils::tracing::warn;

use crate::{Diagnostic, Diagnostics};

/// Adds "entity count" diagnostic to an App
#[derive(Default)]
pub struct EntityCountDiagnosticPlugin;

impl Plugin for EntityCountDiagnosticPlugin {
    fn build(&self, app: &mut App) {
        if !app.world.contains_resource::<Diagnostics>() {
            warn!("Tring to add a DiagnosticsPlugin without the [Diagnostics] resource!");
            return;
        };
        app.add_startup_system(Self::setup_system)
            .add_system(Self::diagnostic_system.exclusive_system());
    }
}

impl EntityCountDiagnosticPlugin {
    pub const ENTITY_COUNT: &'static str = "entity_count";

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::ENTITY_COUNT, 20));
    }

    pub fn diagnostic_system(world: &mut World) {
        let entity_count = world.entities().len();
        let mut diagnostics = world.get_resource_mut::<Diagnostics>().unwrap();
        diagnostics.add_measurement(Self::ENTITY_COUNT, entity_count as f64);
    }
}
