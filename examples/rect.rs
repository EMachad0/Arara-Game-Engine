use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(AssetCountDiagnosticsPlugin::<Image>::default())
        .add_plugin(LogDiagnosticPlugin {
            wait_duration: Duration::from_millis(3000),
        })
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(FlyCamera2dBundle::default());

    let img0: Handle<Image> = asset_server.load("textures/joaozinho.png");
    
    for i in 0..100 {
        for j in 0..100 {
            let x = 10.0 * i as f32;
            let y = 10.0 * j as f32;
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, 0.0),
                    scale: vec3(9.0, 9.0, 1.0),
                    ..Default::default()
                },
                color: Color::rgba(rand::random(), rand::random(), rand::random(), 0.8),
                image: img0.clone(),
                ..Default::default()
            });
        }
    }
}
