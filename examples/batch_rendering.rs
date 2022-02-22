use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(AssetCountDiagnosticsPlugin::<Mesh>::default())
        .add_plugin(LogDiagnosticPlugin::default())
        .add_startup_system(add_cubes)
        .add_startup_system(add_camera)
        .insert_resource(BPLight::new(-5.0, 5.0, -5.0))
        .insert_resource(ClearColor(Color::WHITE))
        .add_system(rotate_cubes)
        .run();
}

#[derive(Component)]
struct Cube;

fn rotate_cubes(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    let rotation = Quat::from_rotation_x(FRAC_PI_2 * time.delta_seconds());
    for mut transform in query.iter_mut() {
        transform.rotate(rotation);
    }
}

fn add_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cuboid = meshes.add(Mesh::from(Cuboid::default()));

    let space = 100.0;
    let cube_amount = 1 << 12;
    for _ in 0..cube_amount {
        let x = rand::random::<f32>() * space;
        let y = rand::random::<f32>() * space;
        let z = rand::random::<f32>() * space;
        commands
            .spawn_bundle(SimpleMeshBundle {
                mesh: cuboid.clone(),
                transform: Transform::from_xyz(x, y, z),
                color: Color::rgb(x / space, y / space, z / space),
                ..Default::default()
            })
            .insert(Cube);
    }
}

fn add_camera(mut commands: Commands) {
    // ------------ Camera -----------------
    commands.spawn_bundle(FlyCameraBundle {
        transform: Transform::from_xyz(-10.0, -10.0, -10.0).looking_at_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
