use arara::prelude::*;
use arara_render::*;

fn main() {
    logger::init();

    App::builder()
        .add_plugin(WindowPlugin)
        .add_plugin(RenderPlugin)
        .init_resource::<CameraController>()
        .add_startup_system(add_spheres.system())
        .build()
        .run()
}

fn add_spheres(mut commands: Commands) {
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new().build(),
        color: Color::PURPLE,
    });
}