pub mod prelude {
    pub use arara_utils::tracing::{
        debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
    };
}
pub use arara_utils::tracing::{
    debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
    Level,
};

use arara_app::{AppBuilder, Plugin};
use tracing_log::LogTracer;
#[cfg(feature = "tracing-chrome")]
use tracing_subscriber::fmt::{format::DefaultFields, FormattedFields};
use tracing_subscriber::{prelude::*, registry::Registry, EnvFilter};

/// Adds logging to Apps. This plugin is part of the `DefaultPlugins`. Adding
/// this plugin will setup a collector appropriate to your target platform:
/// * Using [`tracing-subscriber`](https://crates.io/crates/tracing-subscriber) by default,
/// logging to `stdout`.
///
/// You can configure this plugin using the resource [`LogSettings`].
/// ```no_run
/// # use arara::DefaultPlugins;
/// # use arara_app::App;
/// # use arara_logger::LogSettings;
/// # use arara_utils::tracing::Level;
/// fn main() {
///     App::new()
///         .insert_resource(LogSettings {
///             level: Level::DEBUG,
///             filter: "wgpu=error,bevy_render=info".to_string(),
///         })
///         .add_plugins(DefaultPlugins)
///         .run();
/// }
/// ```
///
/// Log level can also be changed using the `RUST_LOG` environment variable.
/// It has the same syntax has the field [`LogSettings::filter`], see [`EnvFilter`].
/// If you define the `RUST_LOG` environment variable, the [`LogSettings`] resource
/// will be ignored.
///
/// If you want to setup your own tracing collector, you should disable this
/// plugin from `DefaultPlugins` with [`App::add_plugins_with`]:
/// ```no_run
/// # use bevy_internal::DefaultPlugins;
/// # use bevy_app::App;
/// # use bevy_log::LoggerPlugin;
/// fn main() {
///     App::new()
///         .add_plugins_with(DefaultPlugins, |group| group.disable::<LoggerPlugin>())
///         .run();
/// }
/// ```
#[derive(Default)]
pub struct LoggerPlugin;

/// LoggerPlugin settings
pub struct LogSettings {
    /// Filters logs using the [`EnvFilter`] format
    pub filter: String,

    /// Filters out logs that are "less than" the given level.
    /// This can be further filtered using the `filter` setting.
    pub level: Level,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            filter: "".to_string(),
            level: Level::WARN,
        }
    }
}

impl LogSettings {
    fn get_filter(&self) -> String {
        if self.filter == "" {
            self.level.to_string()
        } else {
            format!("{},{}", self.level, self.filter)
        }
    }
}

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        LogTracer::init().unwrap();
        let filter_layer = match app.world_mut().get_resource::<LogSettings>() {
            Some(settings) => EnvFilter::try_new(&settings.get_filter())
                .expect("Failed on parsing [`LogSettings`]"),
            None => EnvFilter::try_from_default_env()
                .or_else(|_| {
                    let settings = app
                        .world_mut()
                        .get_resource_or_insert_with(LogSettings::default);
                    EnvFilter::try_new(&settings.get_filter())
                })
                .unwrap(),
        };

        let subscriber = Registry::default().with(filter_layer);

        #[cfg(feature = "tracing-chrome")]
        let chrome_layer = {
            let (chrome_layer, guard) = tracing_chrome::ChromeLayerBuilder::new()
                .name_fn(Box::new(|event_or_span| match event_or_span {
                    tracing_chrome::EventOrSpan::Event(event) => event.metadata().name().into(),
                    tracing_chrome::EventOrSpan::Span(span) => {
                        if let Some(fields) =
                            span.extensions().get::<FormattedFields<DefaultFields>>()
                        {
                            format!("{}: {}", span.metadata().name(), fields.fields.as_str())
                        } else {
                            span.metadata().name().into()
                        }
                    }
                }))
                .build();
            app.world_mut().insert_non_send(guard);
            chrome_layer
        };

        let fmt_layer = tracing_subscriber::fmt::Layer::default();
        let subscriber = subscriber.with(fmt_layer);

        #[cfg(feature = "tracing-chrome")]
        let subscriber = subscriber.with(chrome_layer);

        arara_utils::tracing::subscriber::set_global_default(subscriber)
            .expect("Could not set global default tracing subscriber. If you've already set up a tracing subscriber, please disable LoggerPlugin from Bevy's DefaultPlugins");
    }
}

pub fn debug_logging_level() {
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
