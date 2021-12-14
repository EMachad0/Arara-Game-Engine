use arara_app::{AppBuilder, Plugin, StartupStage};
use arara_transform::{Transform, GlobalTransform, BuildChildren};
use arara_render::prelude::*;
use arara_geometry::Square;
use bevy_ecs::prelude::*;

// #[macro_use]
// extern crate arara_logger;

pub struct ParticleSystemPlugin;

#[derive(Default)]
pub struct ParticleSystem {
    pub lifetime: f32,
    pub quantity: u32,
}

#[derive(Bundle, Default)]
pub struct ParticleSystemBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub particle_system: ParticleSystem,
}

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.add_startup_system_to_stage(StartupStage::PostStartup, init_particles.system());
    }
}

fn init_particles(
    mut commands: Commands,
    query: Query<(Entity, &ParticleSystem)>,
) {
    for (entity, particle_system) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            for i in 0..particle_system.quantity {
                parent.spawn_bundle(SimpleMeshBundle {
                    mesh: Box::new(Square::new()),
                    transform: Transform::from_xyz(0.0, i as f32, 0.0),
                    color: Color::RED,
                    ..Default::default()
                });
            }
        });
    }
}