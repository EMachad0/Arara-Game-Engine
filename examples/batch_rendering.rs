use arara::prelude::*;
use cgmath::Deg;
use rand;

fn main() {
    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(AssetCountDiagnosticsPlugin::<Mesh>::default())
        .add_plugin(LogDiagnosticPlugin {
            wait_duration: Duration::from_secs(3),
        })
        .add_startup_system(add_cubes.system())
        .insert_resource(FlyCamera::from_camera(
            Camera::new((-10.0, -10.0, -10.0), Deg(45.0), Deg(45.0)),
            2.0,
            0.1,
        ))
        .insert_resource(BPLight {
            position: vec3(-5.0, 5.0, -5.0),
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_system(rotate_squares.system())
        .build()
        .run();
}

struct Square;

fn rotate_squares(time: Res<Time>, mut query: Query<(&mut Transform, With<Square>)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_x(FRAC_PI_2 * time.delta_seconds()));
    }
}

fn add_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cuboid = meshes.add(Mesh::from(Cuboid::default()));

    for _ in 0..(1 << 10) {
        let x = rand::random::<f32>() * 10.0;
        let y = rand::random::<f32>() * 10.0;
        let z = rand::random::<f32>() * 10.0;
        commands
            .spawn_bundle(SimpleMeshBundle {
                mesh: cuboid.clone(),
                transform: Transform::from_xyz(x, y, z),
                color: Color::rgb(x / 10.0, y / 10.0, z / 10.0),
                ..Default::default()
            })
            .insert(Square);
    }
}
