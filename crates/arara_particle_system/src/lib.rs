use std::f32::consts::FRAC_PI_2;

use arara_app::{AppBuilder, Plugin, StartupStage};
use arara_geometry::{Shape, Square};
use arara_render::prelude::*;
use arara_transform::{BuildChildren, GlobalTransform, Transform};
use bevy_ecs::prelude::*;
use glam::{vec3, Quat, Mat4, Vec3};
use arara_camera::{FlyCamera};

// #[macro_use]
// extern crate arara_logger;

pub struct ParticleSystemPlugin;

// #[derive(Default)]
pub struct ParticleSystem {
    pub lifetime: f32,
    pub quantity: u32,
    pub shape: Box<dyn Shape>,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            lifetime: Default::default(),
            quantity: Default::default(),
            shape: Box::new(Square::default()),
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
    pub lifetime: f32,
}

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_startup_system_to_stage(StartupStage::PostStartup, init_particles.system())
            .add_system(update_particles.system());
    }
}

fn update_particles(mut commands: Commands, fly_camera: Res<FlyCamera>, mut query: Query<(&mut Particle, &mut Visibility, &mut Transform)>) {
    
    for (mut particle, mut visibility, mut transform) in query.iter_mut() {
        if !visibility.active {
            *transform = Transform {
                rotation: Quat::from_rotation_x(FRAC_PI_2),
                ..Default::default()
            };
            
            visibility.active = true;
        } else {
            let camera_position = fly_camera.camera.position;
            let camera_position = vec3(camera_position.x, camera_position.y, camera_position.z).normalize();
            let rotation = Mat4::look_at_rh(transform.translation,camera_position, Vec3::Y);
        
            *transform = Transform {
                rotation: Quat::from_mat4(&rotation),
                ..Default::default()
            };
        }



    }
}

fn init_particles(mut commands: Commands, query: Query<(Entity, &ParticleSystem)>) {
    for (entity, particle_system) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            for _ in 0..particle_system.quantity {
                parent
                    .spawn()
                    .insert(Particle { lifetime: 1.0 })
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
