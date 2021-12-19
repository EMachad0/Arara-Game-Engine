use arara::prelude::*;
use arara_particle_system::*;
use cgmath::Deg;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CoordinateSystemPlugin)
        .add_plugin(ParticleSystemPlugin)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin {
            wait_duration: Duration::from_secs(3),
        })
        .add_startup_system(add_shapes)
        // .insert_resource(ClearColor(Color::RED))
        .insert_resource(BPLight {
            position: vec3(0.0, 5.0, 5.0),
        })
        .insert_resource(FlyCamera::from_camera(
            Camera::new((0.0, 5.0, 5.0), Deg(-90.0), Deg(-30.0)),
            20.0,
            0.5,
        ))
        .run()
}

fn add_shapes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let img0: Handle<Image> = asset_server.load("textures/joaozinho.png");

    commands
        .spawn_bundle(TransformBundle {
            transform: Transform {
                translation: vec3(0.5, 0.5, 0.0),
                rotation: Quat::from_euler(EulerRot::ZYX, PI, 0., FRAC_PI_2),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SimpleMeshBundle {
                mesh: meshes.add(Mesh::from(Square::default())),
                transform: Transform::from_xyz(0., 0., 0.),
                image: img0.clone(),
                ..Default::default()
            });
            parent.spawn_bundle(SimpleMeshBundle {
                mesh: meshes.add(Mesh::from(Square::default())),
                transform: Transform::from_xyz(1., 0., 0.),
                image: img0.clone(),
                ..Default::default()
            });
            parent.spawn_bundle(SimpleMeshBundle {
                mesh: meshes.add(Mesh::from(Square::default())),
                transform: Transform::from_xyz(0., 0., 1.),
                image: img0.clone(),
                ..Default::default()
            });
            parent.spawn_bundle(SimpleMeshBundle {
                mesh: meshes.add(Mesh::from(Square::default())),
                transform: Transform::from_xyz(1., 0., 1.),
                color: Color::rgba(0.5, 0.1, 0.5, 0.3),
                ..Default::default()
            });

            // panel
            parent.spawn_bundle(SimpleMeshBundle {
                mesh: meshes.add(Mesh::from(Square::new(5., 5.))),
                transform: Transform::from_xyz(0., -3., 0.),
                color: Color::rgba(0.1, 0.1, 0.5, 1.0),
                image: img0.clone(),
                ..Default::default()
            });
        });
}
