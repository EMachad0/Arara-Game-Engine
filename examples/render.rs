use arara::prelude::*;

fn main() {
    logger::init();
    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin { wait_duration: Duration::from_secs(1) })
        .add_startup_system(draw_cordinate_system.system())
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight::new(-5.0, 10.0, 0.0))
        .build()
        .run()
}

fn add_shapes(mut commands: Commands) {
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cuboid::new(2.0, 2.0, 2.0)),
        transform: Transform::from_translation(Vec3::new(2.0, 1.5, -1.0)),
        color: Color::PURPLE,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Sphere::new(32, 16, 1.0)),
        transform: Transform::from_translation(Vec3::new(-2.0, 1.5, -1.0)),
        color: Color::PURPLE,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Icosphere::new(4, 1.0)),
        transform: Transform::from_translation(Vec3::new(0.0, 3.0, 3.0)),
        color: Color::PURPLE,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 1.0, 10.0, 8.0)),
        transform: Transform::from_rotation(Quat::from_rotation_x(75.0 / 180.0 * PI)),
        color: Color::DARK_GREEN,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Thorus::new(32, 24, 1.0, 0.5)),
        transform: Transform {
            translation: vec3(-3.0, 3.0, 3.0),
            // rotation: Quat::from_rotation_x(FRAC_PI_2),
            ..Default::default()
        },
        color: Color::MIDNIGHT_BLUE,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Capsule::new(0.5, 1.0, 0, 16, 32, CapsuleUvProfile::Aspect)),
        transform: Transform {
            translation: vec3(3.0, 3.0, 3.0),
            // rotation: Quat::from_rotation_x(FRAC_PI_2),
            ..Default::default()
        },
        color: Color::MIDNIGHT_BLUE,
        ..Default::default()
    });
}

fn draw_cordinate_system(mut commands: Commands) {
    let radius = 0.05;
    for i in 0..5 {
        let h = i as f32;
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(0.9, radius, radius)),
            transform: Transform::from_xyz(h+0.5, 0.0, 0.0),
            color: Color::RED,
            ..Default::default()
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(radius, 0.9, radius)),
            transform: Transform::from_xyz(0.0, h+0.5, 0.0),
            color: Color::GREEN,
            ..Default::default()
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(radius, radius, 0.9)),
            transform: Transform::from_xyz(0.0, 0.0, h+0.5),
            color: Color::BLUE,
            ..Default::default()
        });
    }
}