use arara::prelude::*;
use arara_app::*;
use arara_window;

fn main() {
    logger::init();

    logger::test_logging_level();

    let mut app_builder = App::builder();
    // arara_window::build_resource(&mut app_builder);
    app_builder.build().run();
}