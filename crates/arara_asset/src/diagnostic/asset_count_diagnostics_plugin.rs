use crate::{Asset, Assets};
use arara_app::prelude::*;
use arara_diagnostic::{Diagnostic, Diagnostics};
use arara_ecs::system::{Res, ResMut};

/// Adds "asset count" diagnostic to an App
pub struct AssetCountDiagnosticsPlugin<T: Asset> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Asset> Default for AssetCountDiagnosticsPlugin<T> {
    fn default() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Asset> Plugin for AssetCountDiagnosticsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_system)
            .add_system(Self::diagnostic_system);
    }
}

impl<T: Asset> AssetCountDiagnosticsPlugin<T> {
    pub const ASSET_COUNT: &'static str = "asset_count";

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::ASSET_COUNT, 20));
    }

    pub fn diagnostic_system(mut diagnostics: ResMut<Diagnostics>, assets: Res<Assets<T>>) {
        diagnostics.add_measurement(Self::ASSET_COUNT, assets.len() as f64);
    }
}
