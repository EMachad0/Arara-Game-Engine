use arara::prelude::*;

fn main() {
    logger::init();

    logger::test_logging_level();

    App::builder()
        .add_plugin(WindowPlugin::default())
        .add_startup_system(hello_world.system())
        .add_system(render.system())
        .build()
        .run();
}

fn hello_world() {
    println!("Hello World")
}

fn render() {
    println!("Renderizando...")
}