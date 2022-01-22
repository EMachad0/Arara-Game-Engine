use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(FlyCamera2dBundle::default());

    let img0: Handle<Image> = asset_server.load("textures/joaozinho.png");
    
    for i in 0..2 {
        for j in 0..2 {
            let x = 100.0 * i as f32;
            let y = 100.0 * j as f32;
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, 0.0),
                    scale: vec3(60.0, 60.0, 1.0),
                    ..Default::default()
                },
                color: Color::BLUE,
                image: img0.clone_weak(),
                ..Default::default()
            });
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            let x = 100.0 * i as f32;
            let y = 100.0 * j as f32;
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, 0.0),
                    scale: vec3(90.0, 90.0, 0.0),
                    ..Default::default()
                },
                color: Color::RED,
                ..Default::default()
            });
        }
    }

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: meshes.add(Mesh::from(Circle::new(16, 50.0))),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        color: Color::GREEN,
        image: img0,
        ..Default::default()
    });
}
