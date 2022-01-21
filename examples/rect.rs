use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(CoordinateSystemPlugin)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn_bundle(FlyCamera2dBundle::default());
    for i in 0..10 {
        for j in 0..10 {
            let x = 100.0 * i as f32;
            let y = 100.0 * j as f32;
            commands.spawn_bundle(SimpleMeshBundle {
                mesh: meshes.add(Mesh::from(Square::new(90.0, 90.0))),
                transform: Transform::from_xyz(x, y, 0.0),
                color: Color::RED,
                ..Default::default()
            });
        }
    }

    commands.spawn_bundle(SimpleMeshBundle {
        // mesh: meshes.add(Mesh::from(Square::new(100.0, 100.0))),
        mesh: meshes.add(Mesh::from(Circle::new(16, 50.0))),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        color: Color::GREEN,
        ..Default::default()
    });

}
