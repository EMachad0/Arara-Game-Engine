use arara::prelude::*;

/// This example illustrates how to use logs in arara
fn main() {
    // The log level defaults to Warning
    // Use the RUST_LOG enviroment variable to overide the default behavior
    App::new()
        // Uncomment this to change the log settings:
        // .insert_resource(arara::logger::LogSettings {
        //     level: arara::logger::Level::DEBUG,
        //     filter: "".to_string(),
        // })
        .add_plugins(DefaultPlugins)
        .add_startup_system(log_system.system())
        .run();
}

fn log_system() {
    // here is how you write new logs at each "log level"
    // (in "least important" to "most important" order)
    trace!("very noisy");
    debug!("helpful for debugging");
    info!("helpful information that is worth printing by default");
    warn!("something bad happened that isn't a failure, but thats worth calling out");
    error!("something failed");

    // by default, trace and debug logs are ignored because they are "noisy"
    // you can control what level is logged by adding the LogSettings resource
    // alternatively you can set the log level via the RUST_LOG=LEVEL environment variable
    // ex: RUST_LOG=trace, RUST_LOG=info,arara_ecs=warn
    // the format used here is super flexible. check out this documentation for more info:
    // https://docs.rs/tracing-subscriber/*/tracing_subscriber/filter/struct.EnvFilter.html
}
