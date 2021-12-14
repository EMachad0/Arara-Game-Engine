use arara::prelude::*;
use arara_particle_system::*;
use cgmath::Deg;

fn main() {
    logger::init();
    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(CoordinateSystemPlugin)
        .add_plugin(ParticleSystemPlugin)
        .insert_resource(CoordinateSystem::default())
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin { wait_duration: Duration::from_secs(3) })
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight {
            position: vec3(0.0, 10.0, 0.0),
        })
        .insert_resource(FlyCamera::from_camera(
            Camera::new((0.0, 5.0, 5.0), Deg(-90.0), Deg(-30.0)),
            20.0,
            0.5,
        ))
        .build()
        .run()
}

fn add_shapes(mut commands: Commands) {
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Square::default()),
        transform: Transform::from_xyz(1.0, 1.0, 1.0),
        color: Color::PURPLE,
        ..Default::default()
    });
}