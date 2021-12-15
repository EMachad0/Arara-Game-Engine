use arara_app::{AppBuilder, Plugin, StartupStage};
use arara_asset::{Assets, Handle};
use arara_camera::FlyCamera;
use arara_render::{Visibility, Mesh, SimpleMeshBundle, Color, Square};
use arara_time::{Time, Timer};
use arara_transform::{BuildChildren, Children, GlobalTransform, Transform};
use bevy_ecs::prelude::*;
use glam::{vec3, Quat, Vec3};
use rand::{self, Rng};

// #[macro_use]
// extern crate arara_logger;

pub struct ParticleSystemPlugin;

// #[derive(Default)]
pub struct ParticleSystem {
    pub lifetime: f32,
    pub radius: f32,
    pub timer: Timer,
    pub buffer_quantity: u32,
    pub spawn_quantity: u32,
    pub particle_mesh: Handle<Mesh>,
    pub particle_color: Color,
    pub particle_velocity: Value,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            lifetime: 10.0,
            buffer_quantity: 100,
            spawn_quantity: 1,
            radius: 5.0,
            timer: Timer::default(),
            particle_mesh: Default::default(),
            particle_color: Color::WHITE,
            particle_velocity: Value::Constant(2.0)
        }
    }
}

pub enum Value {
    Constant(f32),
    Range(f32, f32),
}

impl Value {
    fn get(&self) -> f32 {
        match self {
            Value::Constant(c) => *c,
            Value::Range(a, b) => {
                let mut rng = rand::thread_rng();

                rng.gen_range(*a..*b)
            },
        }
    }
}

#[derive(Bundle, Default)]
pub struct ParticleSystemBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub particle_system: ParticleSystem,
}

pub struct Particle {
    pub time_remaining: f32,
    pub velocity: f32,
}

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_startup_system_to_stage(StartupStage::PostStartup, init_particles.system())
            .add_system(update_particles.system());
    }
}

fn update_particles(
    // mut commands: Commands,
    fly_camera: Res<FlyCamera>,
    time: Res<Time>,
    mut particle_system_query: Query<(&mut ParticleSystem, Option<&Children>)>,
    mut query: Query<(
        &mut Particle,
        &mut Visibility,
        &mut Transform,
        &GlobalTransform,
    )>,
) {
    let mut rng = rand::thread_rng();

    let camera_position = fly_camera.camera.position;
    let camera_position = vec3(camera_position.x, camera_position.y, camera_position.z);

    for (mut particle_system, children) in particle_system_query.iter_mut() {
        particle_system.timer.tick(time.delta());
        let mut spawn_count = 0;

        if let Some(children) = children {
            for child in children.iter() {
                let (mut particle, mut visibility, mut transform, global_transform) =
                    query.get_mut(*child).unwrap();

                if !visibility.active
                    && spawn_count < particle_system.spawn_quantity
                    && particle_system.timer.finished()
                {
                    *transform = Transform {
                        // rotation: ,
                        translation: vec3(
                            rng.gen_range(-particle_system.radius..particle_system.radius),
                            0.,
                            rng.gen_range(-particle_system.radius..particle_system.radius),
                        ),
                        ..Default::default()
                    };

                    particle.time_remaining = particle_system.lifetime;
                    particle.velocity = particle_system.particle_velocity.get();
                    visibility.active = true;
                    spawn_count += 1;
                } else {
                    if particle.time_remaining <= 0.0 {
                        visibility.active = false;
                    } else {
                        particle.time_remaining -= time.delta_seconds();

                        transform.translation.y -= time.delta_seconds() * particle.velocity;
                    }
                }
                transform.rotation = Quat::from_rotation_arc(
                    Vec3::Y,
                    (camera_position - global_transform.translation).normalize(),
                )
            }
        }
    }
}

fn init_particles(mut commands: Commands, query: Query<(Entity, &ParticleSystem)>, mut meshes: ResMut<Assets<Mesh>>) {
    
    for (entity, particle_system) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            for _ in 0..particle_system.buffer_quantity {
                parent
                    .spawn()
                    .insert(Particle {
                        time_remaining: particle_system.lifetime,
                        velocity: 0.0,
                    })
                    .insert_bundle(SimpleMeshBundle {
                        mesh: particle_system.particle_mesh.clone(),
                        color: Color::WHITE,
                        visibility: Visibility::inactive(),
                        ..Default::default()
                    });
            }
        });
    }
}
