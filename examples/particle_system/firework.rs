use arara::prelude::*;
use arara_particle_system::{self, ParticleSystem, ParticleSystemPlugin, SpawnShape, Value};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticleSystemPlugin)
        .add_plugin(CoordinateSystemPlugin)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin::default())
        .add_plugin(LogDiagnosticPlugin {
            wait_duration: Duration::from_secs(3),
        })
        .add_startup_system(add_color_only_shader)
        .add_startup_system(add_shapes)
        .add_startup_system(add_camera)
        .insert_resource(BPLight {
            position: vec3(10.0, 10.0, 0.0),
        })
        .run()
}

fn add_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // ------------- Particle ------------------

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 1.0,
            buffer_quantity: 1000,
            spawn_quantity: 50,
            particle_velocity: Value::Range(3.0, 5.0),
            spawn_shape: SpawnShape::Sphere(0.1),
            particle_color: ColorOrGradient::Color(Color::RED),
            particle_mesh: meshes.add(Mesh::from(Square::new(0.2, 0.2))),
            timer: Timer::from_seconds(1., false),
            ..Default::default()
        });

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 1.0,
            buffer_quantity: 1000,
            spawn_quantity: 50,
            particle_velocity: Value::Range(3.0, 5.0),
            spawn_shape: SpawnShape::Sphere(0.1),
            particle_color: ColorOrGradient::Color(Color::RED),
            particle_mesh: meshes.add(Mesh::from(Square::new(0.1, 0.1))),
            timer: Timer::from_seconds(1., true),
            ..Default::default()
        });

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 1.1,
            buffer_quantity: 1000,
            spawn_quantity: 20,
            particle_velocity: Value::Range(3.0, 5.0),
            spawn_shape: SpawnShape::Sphere(0.1),
            particle_color: ColorOrGradient::Color(Color::GREEN),
            particle_mesh: meshes.add(Mesh::from(Square::new(0.1, 0.1))),
            timer: Timer::from_seconds(1.01, true),
            ..Default::default()
        });
}

fn add_color_only_shader(asset_server: Res<AssetServer>, mut pipeline: ResMut<CorePipeline>) {
    let fragment_shader = asset_server.load("shaders/fragment_shader_no_light_src.frag");
    pipeline.fragment_shader = fragment_shader.clone();
}

fn add_camera(mut commands: Commands) {
    // ------------ Camera -----------------
    commands.spawn_bundle(FlyCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
