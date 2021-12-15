use arara::prelude::*;
use arara_particle_system::*;
use cgmath::Deg;

fn main() {
    logger::init();
    App::builder()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticleSystemPlugin)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin {
            wait_duration: Duration::from_secs(3),
        })
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight {
            position: vec3(5.0, 10.0, 0.0),
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
    // ------------- Particle ------------------

    commands
        .spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Icosphere::new(6, 0.1)),
            transform: Transform::from_xyz(5.0, 5.0, 1.0),
            color: Color::PURPLE,
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 5.0,
            buffer_quantity: 50,
            spawn_quantity: 5,
            particle_velocity: Value::Range(1.0, 2.0),
            particle_color: Color::BLUE,
            particle_shape: Box::new(Circle::new(8, 1)),
            timer: Timer::from_seconds( 1.0, true),
            ..Default::default()
        });
}
