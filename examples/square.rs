use arara::prelude::*;
use arara_utils::tracing::debug;
use cgmath::Deg;

fn main() {
    App::builder()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_cubes.system())
        .insert_resource(FlyCamera::from_camera(
            Camera::new((3.0, 0.0, 5.0), Deg(-125.0), Deg(0.0)),
            2.0,
            0.1,
        ))
        .insert_resource(BPLight {
            position: vec3(3.0, 2.0, 5.0),
        })
        .add_system(rotate_squares.system())
        .add_system(color_squares.system())
        .build()
        .run();
}

struct Pivot;
struct Square;

fn rotate_squares(time: Res<Time>, mut query: Query<(&mut Transform, With<Pivot>)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_x(FRAC_PI_2 * time.delta_seconds()));
    }
}

fn color_squares(mut query: Query<(&GlobalTransform, &mut Color, With<Square>)>) {
    for (transform, mut color, _) in query.iter_mut() {
        let z = (transform.translation.z + 2.) / 4.;
        debug!("{:?}", z);
        *color = Color::rgb(z, z, z);
    }
}

fn add_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cuboid = meshes.add(Mesh::from(Cuboid::default()));
    commands
        .spawn()
        .insert(Pivot)
        .insert_bundle(TransformBundle::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SimpleMeshBundle {
                    mesh: cuboid.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 2.0),
                    ..Default::default()
                })
                .insert(Square);
            parent
                .spawn_bundle(SimpleMeshBundle {
                    mesh: cuboid.clone(),
                    ..Default::default()
                })
                .insert(Square);
            parent
                .spawn_bundle(SimpleMeshBundle {
                    mesh: cuboid.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -2.0),
                    ..Default::default()
                })
                .insert(Square);
            parent
                .spawn_bundle(SimpleMeshBundle {
                    mesh: cuboid.clone(),
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..Default::default()
                })
                .insert(Square);
            parent
                .spawn_bundle(SimpleMeshBundle {
                    mesh: cuboid,
                    transform: Transform::from_xyz(0.0, -2.0, 0.0),
                    ..Default::default()
                })
                .insert(Square);
        });
}
