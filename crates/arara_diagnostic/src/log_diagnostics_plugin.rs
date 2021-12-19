use super::{Diagnostic, Diagnostics};
use arara_app::prelude::*;
use arara_ecs::prelude::*;
use arara_time::{Duration, Time, Timer};
use arara_utils::tracing::info;

/// An App Plugin that logs diagnostics to the console
pub struct LogDiagnosticPlugin {
    pub wait_duration: Duration,
}

/// State used by the [LogDiagnosticPlugin]
struct LogDiagnosticsTimer {
    timer: Timer,
}

impl Default for LogDiagnosticPlugin {
    fn default() -> Self {
        LogDiagnosticPlugin {
            wait_duration: Duration::from_secs(5),
        }
    }
}

impl Plugin for LogDiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LogDiagnosticsTimer {
            timer: Timer::new(self.wait_duration, true),
        })
        .add_system_to_stage(CoreStage::PostUpdate, Self::log_diagnostics_system.system());
    }
}

impl LogDiagnosticPlugin {
    fn log_diagnostic(diagnostic: &Diagnostic) {
        let name_width = 15;
        if let Some(value) = diagnostic.value() {
            if let Some(average) = diagnostic.average() {
                info!(
                    target: "Diagnostic",
                    "{:<name_width$}: {:>12} (avg {:>})",
                    diagnostic.name,
                    format!("{:.6}", value),
                    format!("{:.6}", average),
                    name_width = name_width,
                );
            } else {
                info!(
                    target: "Diagnostic",
                    "{:<name_width$}: {:>}",
                    diagnostic.name,
                    format!("{:.6}", value),
                    name_width = name_width,
                );
            }
        }
    }

    fn log_diagnostics_system(
        mut state: ResMut<LogDiagnosticsTimer>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
    ) {
        if state.timer.tick(time.delta()).finished() {
            for diagnostic in diagnostics.iter() {
                Self::log_diagnostic(diagnostic);
            }
        }
    }
}
