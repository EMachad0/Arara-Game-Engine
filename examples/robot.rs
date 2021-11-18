use arara::prelude::*;
use arara_geometry::Cuboid;

fn main() {
    logger::init();

    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        // .add_plugin(EntityCountDiagnosticPlugin)
        // .add_plugin(LogDiagnosticPlugin { wait_duration: Duration::from_secs(1) })
        .add_startup_system(add_shapes.system())
        .add_startup_system(draw_cordinate_system.system())
        .insert_resource(BPLight {
            position: vec3(-2.0, 5.0, 3.0),
        })
        .insert_resource(CameraController::from_camera(
            Camera::new((0.0, 2.0, 10.0), Deg(-90.0), Deg(0.0)),
            1.0,
            0.5,
        ))
        .build()
        .run()
}

fn add_shapes(mut commands: Commands) {
    // ------------- Floor ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.1f32, 4f32, 4f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .rotate(Vector3::unit_x(), Deg(-90.0))
            .build(),
        color: Color::BLACK,
    });

    // ------------- Foot ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5f32, 0.3f32, 1f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new().translate(-1., 0.2f32, 0f32).build(),
        color: Color::SILVER,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5f32, 0.3f32, 1f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new().translate(1., 0.2f32, 0f32).build(),
        color: Color::SILVER,
    });

    // ------------- Legs ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5f32, 2f32, 0.5f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(-1., 1f32, -0.25f32)
            .build(),
        color: Color::SILVER,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5f32, 2f32, 0.5f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(1., 1f32, -0.25f32)
            .build(),
        color: Color::SILVER,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.45)
            .translate(1., 2.1f32, -0.25f32)
            .build(),
        color: Color::DARK_GRAY,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.45)
            .translate(-1., 2.1f32, -0.25f32)
            .build(),
        color: Color::DARK_GRAY,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5f32, 1.8f32, 0.5f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(-1., 3f32, -0.25f32)
            .build(),
        color: Color::SILVER,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5f32, 1.8f32, 0.5f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(1., 3f32, -0.25f32)
            .build(),
        color: Color::SILVER,
    });

    // ------------- Body ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(2.55f32, 3f32, 1f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(0., 5.4f32, -0.25f32)
            .build(),
        color: Color::SILVER,
    });

    // ------------- Arms ------------------
    
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5f32, 2f32, 0.5f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(1.6, 5.6, -0.25f32)
            .build(),
        color: Color::SILVER,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.45)
            .translate(1.6, 6.6, -0.25)
            .build(),
        color: Color::DARK_GRAY,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(0.5f32, 2f32, 0.5f32)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(-1.6, 5.6, -0.25f32)
            .build(),
        color: Color::SILVER,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .scale(0.45)
            .translate(-1.6, 6.6, -0.25)
            .build(),
        color: Color::DARK_GRAY,
    });

}

fn draw_cordinate_system(mut commands: Commands) {
    let radius = 0.05;
    for i in 0..5 {
        let h = i as f32;
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cylinder::new(4, 0.9, radius, radius)),
            shaders: Shaders::default(),
            transform: TransformBuilder::new()
                .rotate(Vector3::unit_y(), Deg(90.0))
                .translate(h + 0.5, 0.0, 0.0)
                .build(),
            color: Color::RED,
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cylinder::new(4, 0.9, radius, radius)),
            shaders: Shaders::default(),
            transform: TransformBuilder::new()
                .rotate(Vector3::unit_x(), Deg(90.0))
                .translate(0.0, h + 0.5, 0.0)
                .build(),
            color: Color::GREEN,
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cylinder::new(4, 0.9, radius, radius)),
            shaders: Shaders::default(),
            transform: TransformBuilder::new().translate(0.0, 0.0, h + 0.5).build(),
            color: Color::BLUE,
        });
    }
}
