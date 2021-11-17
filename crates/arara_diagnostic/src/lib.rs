mod diagnostic;
mod entity_count_diagnostics_plugin;
mod frame_time_diagnostics_plugin;
mod log_diagnostics_plugin;

pub use diagnostic::*;
pub use entity_count_diagnostics_plugin::EntityCountDiagnosticPlugin;
pub use frame_time_diagnostics_plugin::FrameTimeDiagnosticPlugin;
pub use log_diagnostics_plugin::LogDiagnosticPlugin;

pub mod prelude {
    pub use crate::{
        DiagnosticsPlugin,
        EntityCountDiagnosticPlugin,
        FrameTimeDiagnosticPlugin,
        LogDiagnosticPlugin,
    };
}

use arara_app::prelude::*;

#[macro_use]
extern crate arara_logger;

/// Adds core diagnostics resources to an App.
#[derive(Default)]
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Diagnostics>();
    }
}
