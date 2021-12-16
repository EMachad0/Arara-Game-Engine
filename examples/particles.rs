use arara::prelude::*;
use arara_particle_system::{self, Value, ParticleSystem, ParticleSystemPlugin, SpawnShape};
use arara_render::DefaultShader;
use cgmath::Deg;

fn main() {
    logger::init();
    App::builder()
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
        .insert_resource(FlyCamera::from_camera(
            Camera::new((0.0, 5.0, 5.0), Deg(-90.0), Deg(-30.0)),
            20.0,
            0.5,
        ))
        .build()
        .run()
}

fn add_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>) {
    // ------------- Particle ------------------
    let img0: Handle<Image> = asset_server.load("textures/joaozinho.png");

    commands
        .spawn_bundle(SimpleMeshBundle {
            mesh: meshes.add(Mesh::from(Icosphere::new(6, 0.1))),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            color: Color::PURPLE,
            ..Default::default()
        })
        .insert(ParticleSystem {
            lifetime: 5.0,
            buffer_quantity: 100,
            spawn_quantity: 5,
            image: Some(img0),
            particle_velocity: Value::Constant(1.0),
            spawn_shape: SpawnShape::Cone(0.5),
            particle_color: Color::RED,
            particle_mesh: meshes.add(Mesh::from(Square::new(0.2, 0.5))),
            timer: Timer::from_seconds( 0.5, true),
            billboard: Some(Billboard::ViewPlane),
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
