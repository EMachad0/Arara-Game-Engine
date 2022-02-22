use arara::prelude::*;

mod path;
mod vehicle;

use crate::{path::*, vehicle::*};

fn main() {
    App::new()
        .insert_resource(WindowProps {
            vsync: true,
            title: "Arara Stering Behaviors".to_string(),
            mode: arara_window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(AssetCountDiagnosticsPlugin::<Image>::default())
        .add_plugin(LogDiagnosticPlugin::default())
        .insert_resource(ClearColor(Color::WHITE))
        .add_startup_system(add_camera)
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, draw_paths)
        .add_system(steer)
        .add_system(move_speedy)
        .add_system(spawn_vehicle_on_mouse_click)
        .run();
}

#[derive(Component)]
struct FollowPath(Entity);

fn steer(path_query: Query<&Path>, mut query: Query<(&Transform, &mut Speedy, &FollowPath)>) {
    let future_constant = 50.0; // just an arbitrary distance in pixels
    for (transform, mut speedy, follow_path) in query.iter_mut() {
        let position = transform.translation.xy();
        let speed = speedy.speed.normalize_or_zero();
        let future_position = position + speed * future_constant;

        let path = path_query.get(follow_path.0).unwrap();
        let mut minimum_distance = 1000000000.0;
        let mut target = vec2(0.0, 0.0);
        for i in 1..path.points.len() {
            let p1 = path.points[i - 1];
            let p2 = path.points[i];
            let path_lenght = (p2 - p1).length();
            let path_dir = (p2 - p1).normalize();

            let path_to_position = future_position - p1;
            let projection = path_to_position.dot(path_dir).clamp(0.0, path_lenght * 0.8);
            let closest_point = p1 + path_dir * projection;
            let distance = future_position.distance(closest_point);
            if distance < minimum_distance {
                minimum_distance = distance;
                target = closest_point + path_dir.normalize() * future_constant;
            }
        }

        let mut desire = target - transform.translation.xy();
        desire = desire.normalize() * speedy.max_speed;
        let steer = desire - speedy.speed;
        speedy.add_force(&steer);
    }
}

// pub const CAR_IMAGE_HANDLE: HandleUntyped =
//     HandleUntyped::weak_from_u64(Image::TYPE_UUID, 14324239193847198473);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: NonSend<Window>) {
    let img0: Handle<Image> = asset_server.load("textures/car_40x20.png");

    let path = commands
        .spawn()
        .insert(Path {
            points: vec![
                vec2(100.0, 100.0),
                vec2(1000.0, 100.0),
                vec2(1000.0, 1000.0),
                vec2(500.0, 500.0),
                vec2(100.0, 1000.0),
                vec2(100.0, 100.0),
            ],
            radius: 50.0,
        })
        .id();

    for _ in 0..4 {
        let x = rand::random::<f32>() * window.width() as f32;
        let y = rand::random::<f32>() * window.height() as f32;
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, 0.0),
                    scale: vec3(40.0, 20.0, 1.0),
                    ..Default::default()
                },
                color: Color::rgb(rand::random(), rand::random(), rand::random()),
                image: img0.clone(),
                ..Default::default()
            })
            .insert(FollowPath(path))
            .insert(Speedy {
                max_speed: 2.0 + rand::random::<f32>() * 8.0,
                max_force: rand::random(),
                ..Default::default()
            });
    }
}

fn spawn_vehicle_on_mouse_click(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    path_query: Query<Entity, With<Path>>,
    asset_server: Res<AssetServer>,
    mouse_query: Query<&WorldMouse2d>,
) {
    let path = path_query.single();
    let img0: Handle<Image> = asset_server.load("textures/car_40x20.png");
    for event in mouse_button_input_events.iter() {
        if let MouseButtonInput {
            button: MouseButton::Left,
            state: ElementState::Pressed,
        } = event
        {
            let mouse = mouse_query.single();
            if let Some(pos) = mouse.pos() {
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: vec3(pos.x, pos.y, 0.0),
                            scale: vec3(40.0, 20.0, 1.0),
                            ..Default::default()
                        },
                        color: Color::rgb(rand::random(), rand::random(), rand::random()),
                        image: img0.clone(),
                        ..Default::default()
                    })
                    .insert(FollowPath(path))
                    .insert(Speedy {
                        max_speed: 2.0 + rand::random::<f32>() * 8.0,
                        max_force: rand::random(),
                        ..Default::default()
                    });
            }
        }
    }
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(FlyCamera2dBundle::default());
}
