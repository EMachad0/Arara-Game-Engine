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
        .add_system(steer)
        .add_system(move_speedy)
        .run();
}

#[derive(Component)]
struct FollowMouse;

#[derive(Component, Default, Debug)]
struct Speedy {
    pub speed: Vec2,
    pub max_speed: f32,
    pub acceleration: Vec2,
    pub max_force: f32,
}

impl Speedy {
    pub fn add_force(&mut self, force: &Vec2) {
        self.acceleration += *force;
    }
}

fn move_speedy(mut query: Query<(&mut Transform, &mut Speedy)>) {
    for (mut transform, mut speedy) in query.iter_mut() {
        let acceleration = speedy.acceleration.clamp_length_max(speedy.max_force);
        speedy.speed += acceleration;
        speedy.speed = speedy.speed.clamp_length_max(speedy.max_speed);
        transform.translation += Vec3::from((speedy.speed, 0.0));
        speedy.acceleration = Vec2::ZERO;
    }
}

fn steer(
    mouse_query: Query<&WorldMouse2d>,
    mut query: Query<(&Transform, &mut Speedy), With<FollowMouse>>,
) {
    let mouse = mouse_query.single();
    if let Some(pos) = mouse.pos() {
        for (transform, mut speedy) in query.iter_mut() {
            let mut desire = pos - transform.translation.xy();
            desire = desire.normalize() * speedy.max_speed;
            let steer = desire - speedy.speed;
            speedy.add_force(&steer);
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: NonSend<Window>) {
    let img0: Handle<Image> = asset_server.load("textures/joaozinho.png");

    for _ in 0..4 {
        let x = rand::random::<f32>() * window.width() as f32;
        let y = rand::random::<f32>() * window.height() as f32;
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, 0.0),
                    scale: vec3(90.0, 90.0, 1.0),
                    ..Default::default()
                },
                color: Color::rgba(rand::random(), rand::random(), rand::random(), 0.8),
                image: img0.clone(),
                ..Default::default()
            })
            .insert(FollowMouse)
            .insert(Speedy {
                max_speed: 5.0,
                max_force: 0.05,
                ..Default::default()
            });
    }
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(FlyCamera2dBundle::default());
}
