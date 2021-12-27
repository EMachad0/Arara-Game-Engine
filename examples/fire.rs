use arara::prelude::*;
use arara_particle_system::{self, ParticleSystem, ParticleSystemPlugin, SpawnShape, Value};

use arara_render::DefaultShader;

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
        .add_startup_system(add_color_only_shader.system())
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight {
            position: vec3(10.0, 10.0, 0.0),
        })
        .insert_resource(Camera::new(vec3(0.0, 5.0, 5.0), -FRAC_PI_2, -FRAC_PI_6))
        .run()
}

fn add_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // ------------- Particle ------------------

    let fire1: Handle<Image> = asset_server.load("textures/fire/fire1.png");
    let fire2: Handle<Image> = asset_server.load("textures/fire/fire2.png");
    let fire3: Handle<Image> = asset_server.load("textures/fire/fire3.png");
    let fire4: Handle<Image> = asset_server.load("textures/fire/fire4.png");

    let fire_gradient = gradient::CustomGradient::new()
        .colors(&[
            Color::hex("69696900").unwrap(),
            Color::hex("FFD800FF").unwrap(),
            Color::hex("FF5033FF").unwrap(),
            Color::hex("82000000").unwrap(),
        ])
        .domain(&[0., 0.15, 0.30, 1.])
        .build()
        .unwrap();

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 3.,
            buffer_quantity: 1000,
            spawn_quantity: 20,
            image: fire1.clone(),
            particle_velocity: Value::Range(1., 2.),
            spawn_shape: SpawnShape::Cone(0.2),
            billboard: Some(Billboard::AxialViewPlane),
            particle_color: ColorOrGradient::Gradient(fire_gradient.clone()),
            particle_mesh: meshes.add(Mesh::from(Square::new(0.5, 0.5))),
            timer: Timer::from_seconds(0.3, true),
            ..Default::default()
        });

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 3.,
            buffer_quantity: 1000,
            spawn_quantity: 20,
            image: fire2.clone(),
            particle_velocity: Value::Range(1., 2.),
            spawn_shape: SpawnShape::Cone(0.2),
            billboard: Some(Billboard::AxialViewPlane),
            particle_color: ColorOrGradient::Gradient(fire_gradient.clone()),
            particle_mesh: meshes.add(Mesh::from(Square::new(0.5, 0.5))),
            timer: Timer::from_seconds(0.3, true),
            ..Default::default()
        });

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 3.,
            buffer_quantity: 1000,
            spawn_quantity: 20,
            image: fire3.clone(),
            particle_velocity: Value::Range(1., 2.),
            spawn_shape: SpawnShape::Cone(0.2),
            billboard: Some(Billboard::AxialViewPlane),
            particle_color: ColorOrGradient::Gradient(fire_gradient),
            particle_mesh: meshes.add(Mesh::from(Square::new(0.5, 0.5))),
            timer: Timer::from_seconds(0.3, true),
            ..Default::default()
        });

    // ------------------------ smoke -----------------------

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 0.7,
            buffer_quantity: 5,
            spawn_quantity: 2,
            image: fire4.clone(),
            particle_velocity: Value::Range(0.5, 1.0),
            spawn_shape: SpawnShape::Cone(0.7),
            particle_color: ColorOrGradient::Color(Color::rgba(0., 0., 0., 0.4)),
            particle_mesh: meshes.add(Mesh::from(Square::new(1., 1.))),
            timer: Timer::from_seconds(1., true),
            ..Default::default()
        });

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 0.5,
            buffer_quantity: 5,
            spawn_quantity: 2,
            image: fire1.clone(),
            particle_velocity: Value::Range(0.5, 1.0),
            spawn_shape: SpawnShape::Cone(0.7),
            particle_color: ColorOrGradient::Color(Color::rgba(0., 0., 0., 0.4)),
            particle_mesh: meshes.add(Mesh::from(Square::new(1., 1.))),
            timer: Timer::from_seconds(1.4, true),
            ..Default::default()
        });

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 0.7,
            buffer_quantity: 5,
            spawn_quantity: 2,
            image: fire3.clone(),
            particle_velocity: Value::Range(0.5, 1.0),
            spawn_shape: SpawnShape::Cone(0.7),
            particle_color: ColorOrGradient::Color(Color::rgba(0., 0., 0., 0.4)),
            particle_mesh: meshes.add(Mesh::from(Square::new(1., 1.))),
            timer: Timer::from_seconds(1.3, true),
            ..Default::default()
        });

    commands
        .spawn_bundle(SimpleMeshBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 0.7,
            buffer_quantity: 5,
            spawn_quantity: 2,
            image: fire2.clone(),
            particle_velocity: Value::Range(0.5, 1.0),
            spawn_shape: SpawnShape::Cone(0.5),
            particle_color: ColorOrGradient::Color(Color::rgba(0., 0., 0., 0.4)),
            particle_mesh: meshes.add(Mesh::from(Square::new(1., 1.))),
            timer: Timer::from_seconds(1.4, true),
            ..Default::default()
        });
}

fn add_color_only_shader(mut commands: Commands, asset_server: Res<AssetServer>) {
    let vertex_shader = asset_server.load("shaders/vertex_shader_src.vert");
    let fragment_shader = asset_server.load("shaders/fragment_shader_no_light_src.frag");
    commands.insert_resource(DefaultShader {
        vertex_shader,
        fragment_shader,
    });
}
