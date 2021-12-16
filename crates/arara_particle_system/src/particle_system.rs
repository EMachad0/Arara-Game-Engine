use std::f32::consts::{PI};

use arara_asset::Handle;
use arara_camera::FlyCamera;
use arara_render::{Color, Mesh, SimpleMeshBundle, Visibility};
use arara_time::{Time, Timer};
use arara_transform::{BuildChildren, Children, GlobalTransform, Transform};
use bevy_ecs::prelude::{Bundle, Commands, Entity, Query, Res};
use glam::{vec3, vec4, Mat4, Quat, Vec3};

use crate::{Particle, Value};

use rand::{self, Rng};

pub struct ParticleSystem {
    pub lifetime: f32,
    pub timer: Timer,
    pub spawn_shape: SpawnShape,
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
            spawn_shape: SpawnShape::Circle(1.0),
            timer: Timer::default(),
            particle_mesh: Default::default(),
            particle_color: Color::WHITE,
            particle_velocity: Value::Constant(2.0),
        }
    }
}

#[derive(Bundle, Default)]
pub struct ParticleSystemBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub particle_system: ParticleSystem,
}

pub fn update_particles(
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
                        // rotation: Quat::from_rotation_x(FRAC_PI_2),
                        translation: particle_system.spawn_shape.gen_random_translation(),
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
                        transform.translation.y += time.delta_seconds() * particle.velocity;
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

pub fn init_particles(mut commands: Commands, query: Query<(Entity, &ParticleSystem)>) {
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

pub enum SpawnShape {
    Rectangle(f32, f32),
    Circle(f32),
}

impl SpawnShape {
    fn gen_random_translation(&self) -> Vec3 {
        let mut rng = rand::thread_rng();

        match self {
            Self::Rectangle(x, y) => {
                let x_size = x / 2.0;
                let y_size = y / 2.0;

                return vec3(rng.gen_range(-x_size..x_size), 0., rng.gen_range(-y_size..y_size));
            }
            Self::Circle(r) => {
                let rand_1: f32 = rng.gen();
                let rand_2: f32 = rng.gen();

                let radius = r * rand_1.sqrt();
                let theta: f32 = rand_2 * 2.0 * PI;

                let x = radius * theta.cos();
                let y = radius * theta.sin();
                return vec3(x, 2., y);
            }
        }
    }
}

pub fn target_to(eye: &Vec3, target: &Vec3, up: &Vec3) -> Mat4 {
    let eyex = eye[0];
    let eyey = eye[1];
    let eyez = eye[2];
    let upx = up[0];
    let upy = up[1];
    let upz = up[2];

    let mut z0 = eyex - target[0];
    let mut z1 = eyey - target[1];
    let mut z2 = eyez - target[2];

    let mut len = z0 * z0 + z1 * z1 + z2 * z2;
    if len > 0_f32 {
        len = 1. / f32::sqrt(len);
        z0 *= len;
        z1 *= len;
        z2 *= len;
    }

    let mut x0 = upy * z2 - upz * z1;
    let mut x1 = upz * z0 - upx * z2;
    let mut x2 = upx * z1 - upy * z0;

    len = x0 * x0 + x1 * x1 + x2 * x2;
    if len > 0_f32 {
        len = 1. / f32::sqrt(len);
        x0 *= len;
        x1 *= len;
        x2 *= len;
    }

    let out = Mat4::from_cols(
        vec4(x0, x1, x2, 0.),
        vec4(z1 * x2 - z2 * x1, z2 * x0 - z0 * x2, z0 * x1 - z1 * x0, 0.),
        vec4(z0, z1, z2, 0.),
        vec4(eyex, eyey, eyez, 1.),
    );

    out
}
