use arara_app::{AppBuilder, Plugin, StartupStage};
use arara_geometry::{Shape, Square};
use arara_render::prelude::*;
use arara_transform::{BuildChildren, GlobalTransform, Transform};
use bevy_ecs::prelude::*;
use glam::vec3;

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
            shape: Box::new(Square::new()),
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
        app_builder.add_startup_system_to_stage(StartupStage::PostStartup, init_particles.system());
    }
}

fn init_particles(mut commands: Commands, query: Query<(Entity, &ParticleSystem)>) {
    for (entity, particle_system) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            for _ in 0..particle_system.quantity {
                parent
                    .spawn()
                    .insert(Particle {
                        lifetime: 1.0,
                    })
                    .insert_bundle(SimpleMeshBundle {
                        mesh: Box::new(Square::new()),
                        color: Color::RED,
                        visibility: Visibility::inactive(),
                        ..Default::default()
                    });
            }
        });
    }
}
