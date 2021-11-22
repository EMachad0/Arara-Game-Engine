use arara::prelude::*;
use cgmath::Deg;

fn main() {
    logger::init();

    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin { wait_duration: Duration::from_secs(3) })
        .add_startup_system(add_shapes.system())
        .add_startup_system(draw_cordinate_system.system())
        .insert_resource(BPLight {
            position: vec3(0.0, 50.0, 30.0),
        })
        .insert_resource(FlyCamera::from_camera(
            Camera::new((0.0, 30.0, 70.0), Deg(-90.0), Deg(0.0)),
            20.0,
            0.5,
        ))
        .build()
        .run()
}

struct Tower;

fn add_shapes(mut commands: Commands) {
    // ------------- Floor ------------------
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 0.1, 50., 50.)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        color: Color::DARK_GREEN,
        ..Default::default()
    });

    // ------------- Tower ------------------
    let tower_radius = 5.0;
    let tower_distance = 50.0;
    let tower_height = 20.0;
    for i in 0..2 {
        for j in 0..2 {
            let x = i as f32 * tower_distance - tower_distance / 2.0;
            let z = j as f32 * tower_distance - tower_distance / 2.0;
            commands.spawn().insert(Tower)
            .insert_bundle(SimpleMeshBundle {
                mesh: Box::new(Cylinder::new(16, tower_height, tower_radius, tower_radius)),
                transform: Transform {
                    translation: vec3(x, tower_height / 2.0, z),
                    rotation: Quat::from_rotation_x(FRAC_PI_2),
                    ..Default::default()
                },
                color: Color::ORANGE,
                ..Default::default()
            });
        }
    }
}

fn draw_cordinate_system(mut commands: Commands) {
    let count = 5;
    let lenght = 10.0;
    let radius = 0.05;
    for i in 0..count {
        let pos = (i as f32 + 0.5) * lenght;
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(0.9 * lenght, radius, radius)),
            transform: Transform::from_xyz(pos, 0.0, 0.0),
            color: Color::RED,
            ..Default::default()
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(radius, 0.9 * lenght, radius)),
            transform: Transform::from_xyz(0.0, pos, 0.0),
            color: Color::GREEN,
            ..Default::default()
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(radius, radius, 0.9 * lenght)),
            transform: Transform::from_xyz(0.0, 0.0, pos),
            color: Color::BLUE,
            ..Default::default()
        });
    }
}
