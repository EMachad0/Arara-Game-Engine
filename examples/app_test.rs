use arara::prelude::*;
use arara_app::*;
use arara_window;

fn main() {
    logger::init();

    logger::test_logging_level();

    App::builder()
        .add_plugin(arara_window::WindowPlugin::default())
        .build()
        .run();
}