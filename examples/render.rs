use std::f32::consts::{FRAC_PI_2, PI};

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
        .build()
        .run()
}

fn add_shapes(mut commands: Commands) {
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 3.0, 2.0, 0.5)),
        shaders: Shaders::default(),
        transform: Transform::from_translation(Vec3::new(2.0, 1.5, 0.0)),
        color: Color::PURPLE,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(4, 1.0, 1.0, 1.0)),
        shaders: Shaders::default(),
        transform: Transform::from_translation(Vec3::new(-1.0, 1.0, -1.0)),
        color: Color::PURPLE,
        ..Default::default()
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 1.0, 10.0, 10.0)),
        shaders: Shaders::default(),
        transform: Transform::from_rotation(Quat::from_rotation_x(75.0 / 180.0 * PI)),
        color: Color::DARK_GREEN,
        ..Default::default()
    });
}

fn draw_cordinate_system(mut commands: Commands) {
    let radius = 0.05;
    for i in 0..5 {
        let h = i as f32;
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cylinder::new(4, 0.9, radius, radius)),
            shaders: Shaders::default(),
            transform: Transform {
                translation: Vec3::new(h+0.5, 0.0, 0.0),
                rotation: Quat::from_rotation_y(FRAC_PI_2),
                ..Default::default()
            },
            color: Color::RED,
            ..Default::default()
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cylinder::new(4, 0.9, radius, radius)),
            shaders: Shaders::default(),
            transform: Transform {
                translation: Vec3::new(0.0, h+0.5, 0.0),
                rotation: Quat::from_rotation_x(FRAC_PI_2),
                ..Default::default()
            },
            color: Color::GREEN,
            ..Default::default()
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cylinder::new(4, 0.9, radius, radius)),
            shaders: Shaders::default(),
            transform: Transform::from_xyz(0.0, 0.0, h+0.5),
            color: Color::BLUE,
            ..Default::default()
        });
    }
}