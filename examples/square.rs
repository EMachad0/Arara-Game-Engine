use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BPLight {
            position: vec3(3.0, 2.0, 5.0),
        })
        .add_startup_system(add_camera)
        .add_startup_system(add_cubes)
        .add_system(rotate_squares)
        .add_system(color_squares)
        .run();
}

#[derive(Component)]
struct Pivot;
#[derive(Component)]
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

fn add_camera(mut commands: Commands) {
    // ------------ Camera -----------------
    commands.spawn_bundle(FlyCameraBundle {
        transform: Transform::from_xyz(3.0, 0.0, 5.0).looking_at_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
