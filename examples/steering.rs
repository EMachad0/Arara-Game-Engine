use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(AssetCountDiagnosticsPlugin::<Image>::default())
        .add_plugin(LogDiagnosticPlugin::default())
        .add_startup_system(add_camera)
        .add_startup_system(setup)
        .add_system(mouse_debug)
        .run();
}

#[derive(Component)]
struct FollowMouse;

fn mouse_debug(mouse_query: Query<&WorldMouse2d>, mut query: Query<(&mut Transform, &mut Visibility), With<FollowMouse>>) {
    let mouse = mouse_query.single();
    for (mut transform, mut visibility) in query.iter_mut() {
        if let Some(pos) = mouse.pos() {
            visibility.visible = true;
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        } else {
            visibility.visible = false;
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let img0: Handle<Image> = asset_server.load("textures/joaozinho.png");
    
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            scale: vec3(50.0, 50.0, 10.0),
            ..Default::default()
        },
        image: img0.clone(),
        ..Default::default()
    }).insert(FollowMouse);

    for i in 0..10 {
        for j in 0..10 {
            let x = 100.0 * i as f32;
            let y = 100.0 * j as f32;
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, 0.0),
                    scale: vec3(90.0, 90.0, 1.0),
                    ..Default::default()
                },
                color: Color::rgba(rand::random(), rand::random(), rand::random(), 0.8),
                image: img0.clone(),
                ..Default::default()
            });
        }
    }
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(FlyCamera2dBundle::default());
}
