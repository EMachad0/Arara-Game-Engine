use arara::prelude::*;

fn main() {
    logger::init();

    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        // .add_plugin(EntityCountDiagnosticPlugin)
        // .add_plugin(LogDiagnosticPlugin { wait_duration: Duration::from_secs(1) })
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight {
        position: vec3(-2.0, 5.0, 3.0),
    })
        .build()
        .run()
}

fn add_shapes(mut commands: Commands) {

    // ------------- Floor ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.1f32, 4f32, 4f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new().rotate(Vector3::unit_x(), Deg(-90.0)).build(),
        color: Color::MIDNIGHT_BLUE,
    });

    // ------------- Body ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(2f32)
            .translate(0f32, 1.2f32, 0f32)
            .build(),
        color: Color::WHITE,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(1.2f32)
            .translate(0f32, 3.3f32, 0f32)
            .build(),
        color: Color::WHITE,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.75f32)
            .translate(0f32, 4.75f32, 0f32)
            .build(),
        color: Color::WHITE,
    });

    

    // ------------- Clothing ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.09f32)
            .translate(0f32, 3.8f32, 1.055f32)
            .build(),
        color: Color::BLACK,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.09f32)
            .translate(0f32, 3.4f32, 1.16f32)
            .build(),
        color: Color::BLACK,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.09f32)
            .translate(0f32, 2.4f32, 1.558f32)
            .build(),
        color: Color::BLACK,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.09f32)
            .translate(0f32, 1.9f32, 1.84f32)
            .build(),
        color: Color::BLACK,
    });

    // ------------- Eyes ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.1f32)
            .translate(-0.25f32, 4.77f32, 0.7f32)
            .build(),
        color: Color::BLACK,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.1f32)
            .translate(0.25f32, 4.77f32, 0.7f32)
            .build(),
        color: Color::BLACK,
    });

    // ------------- Nose ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.8f32, 0.15, 0.01)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(0f32, 4.70f32, 1f32)
            .build(),
        color: Color::ORANGE_RED,
    });

    // ------------- Hat ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.05f32, 1.1f32, 1.1f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .rotate(Vector3::unit_x(), Deg(90.0))
            .translate(0f32, 5.1f32, 0f32)
            .build(),
        color: Color::BLACK,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 2f32, 0.6f32, 0.6f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .rotate(Vector3::unit_x(), Deg(90.0))
            .translate(0f32, 5.1f32, 0f32)
            .build(),
        color: Color::BLACK,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.5f32, 0.655f32, 0.655f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .rotate(Vector3::unit_x(), Deg(90.0))
            .translate(0f32, 5.1f32, 0f32)
            .build(),
        color: Color::BLUE,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.5f32, 0.655f32, 0.655f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .rotate(Vector3::unit_x(), Deg(90.0))
            .translate(0f32, 5.1f32, 0f32)
            .build(),
        color: Color::BLUE,
    });

    // ------------- Arms ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 2.5f32, 0.08f32, 0.01f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .rotate(Vector3::unit_y(), Deg(90.0))
            .rotate(Vector3::unit_z(), Deg(30.0))
            .translate(2.2f32, 4.2f32, 0f32)
            .build(),
        color: Color::hex("2C1A0B").unwrap(),
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 2.5f32, 0.08f32, 0.01f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .rotate(Vector3::unit_y(), Deg(-90.0))
            .rotate(Vector3::unit_z(), Deg(-30.0))
            .translate(-2.2f32, 4.2f32, 0f32)
            .build(),
        color: Color::hex("2C1A0B").unwrap(),
    });

}
