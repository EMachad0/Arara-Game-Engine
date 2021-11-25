use arara::prelude::*;

fn main() {
    logger::init();
    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(AssetCountDiagnosticsPlugin::<Image>::default())
        .add_plugin(LogDiagnosticPlugin { wait_duration: Duration::from_secs(1) })
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight::new(-5.0, 10.0, 0.0))
        .build()
        .run()
}

fn add_shapes(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {

    let img0: Handle<Image> = asset_server.load("textures/joaozinho.png");
    let img1: Handle<Image> = asset_server.load("textures/white-snow.jpg");

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
        image: Some(img0.clone()),
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Icosphere::new(4, 1.0)),
        transform: Transform::from_translation(Vec3::new(0.0, 3.0, 3.0)),
        color: Color::PURPLE,
        image: Some(img1.clone()),
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 1.0, 10.0, 8.0)),
        transform: Transform::from_rotation(Quat::from_rotation_x(75.0 / 180.0 * PI)),
        color: Color::DARK_GREEN,
        image: Some(img0.clone()),
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Thorus::new(32, 24, 1.0, 0.5)),
        transform: Transform {
            translation: vec3(-3.0, 3.0, 3.0),
            ..Default::default()
        },
        color: Color::MIDNIGHT_BLUE,
        image: Some(img1.clone()),
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Capsule::new(0.5, 1.0, 0, 16, 32, CapsuleUvProfile::Aspect)),
        transform: Transform {
            translation: vec3(3.0, 3.0, 3.0),
            ..Default::default()
        },
        color: Color::MIDNIGHT_BLUE,
        ..Default::default()
    });
}
