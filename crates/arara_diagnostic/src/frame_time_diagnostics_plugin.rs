use crate::{Diagnostic, Diagnostics};
use arara_app::prelude::*;
use arara_ecs::system::{IntoSystem, Res, ResMut};
use arara_time::Time;

/// Adds "frame time" diagnostics to the App
/// specifically "frame time", "fps" and "frame count"
#[derive(Default)]
pub struct FrameTimeDiagnosticPlugin;

pub struct FrameTimeDiagnosticsCounter {
    frame_count: f64,
}

impl Plugin for FrameTimeDiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_system.system())
            .insert_resource(FrameTimeDiagnosticsCounter { frame_count: 0.0 })
            .add_system(Self::diagnostic_system.system());
    }
}

impl FrameTimeDiagnosticPlugin {
    pub const FPS: &'static str = "fps";
    pub const FRAME_COUNT: &'static str = "frame_time";
    pub const FRAME_TIME: &'static str = "frame_count";

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::FRAME_TIME, 20));
        diagnostics.add(Diagnostic::new(Self::FPS, 20));
        diagnostics.add(Diagnostic::new(Self::FRAME_COUNT, 1));
    }

    pub fn diagnostic_system(
        mut diagnostics: ResMut<Diagnostics>,
        time: Res<Time>,
        mut state: ResMut<FrameTimeDiagnosticsCounter>,
    ) {
        state.frame_count += 1.0;
        diagnostics.add_measurement(Self::FRAME_COUNT, state.frame_count);

        if time.delta_seconds_f64() == 0.0 {
            return;
        }

        diagnostics.add_measurement(Self::FRAME_TIME, time.delta_seconds_f64());
        if let Some(fps) = diagnostics
            .get(Self::FRAME_TIME)
            .and_then(|frame_time_diagnostic| {
                frame_time_diagnostic
                    .average()
                    .and_then(|frame_time_average| {
                        if frame_time_average > 0.0 {
                            Some(1.0 / frame_time_average)
                        } else {
                            None
                        }
                    })
            })
        {
            diagnostics.add_measurement(Self::FPS, fps);
        }
    }
}
