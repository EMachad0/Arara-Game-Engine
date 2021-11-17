use arara::prelude::*;

fn main() {
    logger::init();

    App::builder()
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticPlugin)
    // .add_plugin(EntityCountDiagnosticPlugin)
    // .add_plugin(LogDiagnosticPlugin { wait_duration: Duration::from_secs(1) })
    .add_system(add_shapes.system())
    .build()
    .run()
}

fn add_shapes(mut commands: Commands) {
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Circle::new(32, 5f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::default().build(),
        color: Color::PURPLE,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(16, 8)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(2f32)
            .translate(0f32, 1.2f32, 0f32)
            .build(),
        color: Color::WHITE,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(16, 8)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(1.2f32)
            .translate(0f32, 3.4f32, 0f32)
            .build(),
        color: Color::WHITE,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(16, 8)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.75f32)
            .translate(0f32, 4.75f32, 0f32)
            .build(),
        color: Color::WHITE,
    });


    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.3, 0.2, 0.01)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.75f32)
            .translate(0f32, 4.75f32, 1f32)
            .build(),
        color: Color::ORANGE,
    });

}
