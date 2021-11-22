use arara::prelude::*;

fn main() {
    logger::init();
    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin::default())
        .add_plugin(CoordinateSystemPlugin::default())
        .add_startup_system(add_shapes.system())
        .init_resource::<Timer>()
        .insert_resource(BPLight::new(-2.0, 5.0, 3.0))
        .add_system(move_snowman.system())
        .build()
        .run()
}

struct SnowMan;
struct Body;
struct Clothing;
struct Face;
struct Hat;
struct Arms;

fn move_snowman(
    time: Res<Time>,
    mut query: Query<(&mut Transform, With<SnowMan>)>,
) {
    for transform in query.iter_mut() {
        let mut tr = transform.0;
        tr.rotate(Quat::from_rotation_y(FRAC_PI_2 * time.delta_seconds()));
    }
}

fn add_shapes(mut commands: Commands) {
    // ------------- Floor ------------------
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.1, 4.0, 4.0)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        color: Color::MIDNIGHT_BLUE,
        ..Default::default()
    });

    // ------------- SnowMan ------------------
    commands.spawn()
    .insert(SnowMan)
    .insert_bundle(TransformBundle {
        transform: Transform::default(),
        global_transform: GlobalTransform::default(),
    })
    .with_children(|snow_man| {
        // ------------- Body ------------------
        snow_man.spawn().insert(Body)
        .insert_bundle(TransformBundle::default())
        .with_children(|body| {
            body.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::default()),
                transform: Transform {
                    scale: vec3(2.0, 2.0, 2.0),
                    translation: vec3(0.0, 1.2, 0.0),
                    ..Default::default()
                },
                color: Color::WHITE,
                ..Default::default()
            });
            body.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::default()),
                transform: Transform {
                    scale: vec3(1.2, 1.2, 1.2),
                    translation: vec3(0.0, 3.3, 0.0),
                    ..Default::default()
                },
                color: Color::WHITE,
                ..Default::default()
            });
            body.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::default()),
                transform: Transform {
                    scale: vec3(0.75, 0.75, 0.75),
                    translation: vec3(0.0, 4.75, 0.0),
                    ..Default::default()
                },
                color: Color::WHITE,
                ..Default::default()
            });
        });
    })
    .with_children(|snow_man| {
        // ------------- Clothing ------------------
        snow_man.spawn().insert(Clothing)
        .insert_bundle(TransformBundle::default())
        .with_children(|clothing| {
            clothing.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::new(32, 16, 0.09)),
                transform: Transform::from_xyz(0.0, 3.8, 1.055),
                color: Color::BLACK,
                ..Default::default()
            });
            clothing.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::new(32, 16, 0.09)),
                transform: Transform::from_xyz(0.0, 2.4, 1.558),
                color: Color::BLACK,
                ..Default::default()
            });
            clothing.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::new(32, 16, 0.09)),
                transform: Transform::from_xyz(0.0, 3.4, 1.16),
                color: Color::BLACK,
                ..Default::default()
            });
            clothing.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::new(32, 16, 0.09)),
                transform: Transform::from_xyz(0.0, 1.9, 1.84),
                color: Color::BLACK,
                ..Default::default()
            });
        });
    })
    .with_children(|snow_man| {
        // ------------- Face ------------------
        snow_man.spawn().insert(Face)
        .insert_bundle(TransformBundle {
            transform: Transform::from_xyz(0.0, 4.72, 0.0),
            ..Default::default()
        })
        // ------------- Eyes ------------------
        .with_children(|face| {
            face.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::new(32, 16, 0.1)),
                transform: Transform::from_xyz(-0.25, 0.07, 0.7),
                color: Color::BLACK,
                ..Default::default()
            });
            face.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Sphere::new(32, 16, 0.1)),
                transform: Transform::from_xyz(0.25, 0.07, 0.7),
                color: Color::BLACK,
                ..Default::default()
            });
            face.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cylinder::new(32, 0.8, 0.15, 0.01)),
                transform: Transform::from_xyz(0.0, 0.00, 1.0),
                color: Color::ORANGE_RED,
                ..Default::default()
            });
        });
    })
    .with_children(|snow_man| {
        // ------------- Hat ------------------
        snow_man.spawn().insert(Hat)
        .insert_bundle(TransformBundle {
            transform: Transform {
                translation: vec3(0.0, 5.1, 0.0),
                rotation: Quat::from_rotation_x(FRAC_PI_2),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|hat| {
            hat.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cylinder::new(32, 0.05, 1.1, 1.1)),
                color: Color::BLACK,
                ..Default::default()
            });
            hat.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cylinder::new(32, 2.0, 0.6, 0.6)),
                color: Color::BLACK,
                ..Default::default()
            });
            hat.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cylinder::new(32, 0.5, 0.655, 0.655)),
                color: Color::BLUE,
                ..Default::default()
            });
            hat.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cylinder::new(32, 0.5, 0.655, 0.655)),
                color: Color::BLUE,
                ..Default::default()
            });
        });
    })
    .with_children(|snow_man| {
        // ------------- Arms ------------------
        snow_man.spawn().insert(Arms)
        .insert_bundle(TransformBundle {
            transform: Transform { 
                translation: vec3(0.0, 4.2, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|arms| {
            arms.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cylinder::new(32, 2.5, 0.08, 0.01)),
                transform: Transform {
                    translation: vec3(2.2, 0.0, 0.0),
                    rotation: Quat::from_euler(EulerRot::ZYX, FRAC_PI_6, FRAC_PI_2, 0.0),
                    scale: Vec3::ONE,
                },
                color: Color::hex("2C1A0B").unwrap(),
                ..Default::default()
            });
            arms.spawn_bundle(SimpleMeshBundle {
                mesh: Box::new(Cylinder::new(32, 2.5, 0.08, 0.01)),
                transform: Transform {
                    translation: vec3(-2.2, 0.0, 0.0),
                    rotation: Quat::from_euler(EulerRot::ZYX, -FRAC_PI_6, -FRAC_PI_2, 0.0),
                    scale: Vec3::ONE,
                },
                color: Color::hex("2C1A0B").unwrap(),
                ..Default::default()
            });
        });
    });
}