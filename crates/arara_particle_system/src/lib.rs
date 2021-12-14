use std::f32::consts::FRAC_PI_2;

use arara_app::{AppBuilder, Plugin, StartupStage};
use arara_camera::FlyCamera;
use arara_geometry::{Shape, Square};
use arara_render::prelude::*;
use arara_transform::{BuildChildren, Children, GlobalTransform, Transform};
use bevy_ecs::prelude::*;
use glam::{vec3, Mat4, Quat, Vec3};
use rand::{self, Rng};

// #[macro_use]
// extern crate arara_logger;

pub struct ParticleSystemPlugin;

// #[derive(Default)]
pub struct ParticleSystem {
    pub lifetime: f32,
    pub quantity: u32,
    pub radius: f32,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            lifetime: Default::default(),
            quantity: Default::default(),
            radius: 5.0,
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
    mut commands: Commands,
    fly_camera: Res<FlyCamera>,
    particle_system_query: Query<(&ParticleSystem, Option<&Children>)>,
    mut query: Query<(&mut Particle, &mut Visibility, &mut Transform)>,
) {
    let mut rng = rand::thread_rng();
    
    let camera_position = fly_camera.camera.position;
    let camera_position = vec3(camera_position.x, camera_position.y, camera_position.z).normalize();

    for (particle_system, children) in particle_system_query.iter() {
        if let Some(children) = children {
            for child in children.iter() {
                let (mut particle, mut visibility, mut transform) = query.get_mut(*child).unwrap();

                if !visibility.active {
                    *transform = Transform {
                        rotation: Quat::from_rotation_x(FRAC_PI_2),
                        translation: vec3(
                            rng.gen_range(-particle_system.radius..particle_system.radius),
                            rng.gen_range(-particle_system.radius..particle_system.radius),
                            0.,
                        ),
                        ..Default::default()
                    };

                    particle.time_remaining = particle_system.lifetime;
                    visibility.active = true;
                } else {
                }
            }
        }
    }
}

fn init_particles(mut commands: Commands, query: Query<(Entity, &ParticleSystem)>) {
    for (entity, particle_system) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            for _ in 0..particle_system.quantity {
                parent
                    .spawn()
                    .insert(Particle {
                        time_remaining: 1.0,
                        velocity: 0.0,
                    })
                    .insert_bundle(SimpleMeshBundle {
                        mesh: Box::new(Square::default()),
                        color: Color::WHITE,
                        visibility: Visibility::inactive(),
                        ..Default::default()
                    });
            }
        });
    }
}
