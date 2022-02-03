use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_shapes)
        .add_startup_system(add_camera)
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(BPLight::new(-5.0, 10.0, 0.0))
        .run()
}

fn add_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load("textures/madeira.jpeg");
    let grass_image = asset_server.load("textures/grass.jpg");
    let mesa_mesh = asset_server.load("models/mesa.glb#Mesh0/Primitive0");

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: mesa_mesh,
        transform: Transform::from_xyz(0.0, 1.0, -1.0),
        image,
        ..Default::default()
    });

    // ------------- Floor ------------------
    let floor_height = 3.0;
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: meshes.add(Mesh::from(Cylinder::new(32, floor_height, 10., 13.))),
        transform: Transform {
            translation: vec3(0.0, -floor_height / 2.0, 0.0),
            rotation: Quat::from_rotation_x(FRAC_PI_2),
            ..Default::default()
        },
        // color: Color::DARK_GREEN,
        image: grass_image,
        ..Default::default()
    });
}

fn add_camera(mut commands: Commands) {
    // ------------ Camera -----------------
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
