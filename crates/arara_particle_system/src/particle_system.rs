use std::f32::consts::PI;

use arara_asset::Handle;
use arara_camera::FlyCamera;
use arara_render::{Color, Mesh, SimpleMeshBundle, Visibility, Image};
use arara_time::{Time, Timer};
use arara_transform::{BuildChildren, Children, GlobalTransform, Transform};
use bevy_ecs::prelude::{Bundle, Commands, Entity, Query, Res};
use glam::{vec3, Mat3, Quat, Vec3};

use crate::{Particle, Value};

use rand::{self, Rng};

pub struct ParticleSystem {
    pub lifetime: f32,
    pub timer: Timer,
    pub spawn_shape: SpawnShape,
    pub buffer_quantity: u32,
    pub spawn_quantity: u32,
    pub image: Option::<Handle<Image>>,
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
            image: None
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
    let _camera_position = vec3(camera_position.x, camera_position.y, camera_position.z);

    // view-plane billboard
    let u = Vec3::Y;
    let n = vec3(
        fly_camera.camera.yaw.0.cos(),
        fly_camera.camera.pitch.0.sin(),
        fly_camera.camera.yaw.0.sin(),
    );
    let r = u.cross(n).normalize();
    let u_linha = n.cross(r).normalize();
    let mat = Mat3::from_cols(r, u_linha, n);
    let rotation = Quat::from_mat3(&mat);

    // axial-locked view-plane billboard
    // let u = Vec3::Y;
    // let n = vec3(fly_camera.camera.yaw.0.cos(), fly_camera.camera.pitch.0.sin(), fly_camera.camera.yaw.0.sin());
    // let r = u.cross(n).normalize();
    // let n_linha = r.cross(u).normalize();
    // let mat = Mat3::from_cols(r, u, n_linha);
    // let rotation = Quat::from_mat3(&mat);

    for (mut particle_system, children) in particle_system_query.iter_mut() {
        particle_system.timer.tick(time.delta());
        let mut spawn_count = 0;

        if let Some(children) = children {
            for child in children.iter() {
                let (mut particle, mut visibility, mut transform, _global_transform) =
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
                    particle.direction = particle_system.spawn_shape.get_direction();
                    visibility.active = true;
                    spawn_count += 1;
                } else {
                    if particle.time_remaining <= 0.0 {
                        visibility.active = false;
                    } else {
                        particle.time_remaining -= time.delta_seconds();
                        transform.translation +=
                            particle.direction * time.delta_seconds() * particle.velocity;
                    }
                }

                // view-point billboard
                // let u = Vec3::Y;
                // let n = -(_camera_position - _global_transform.translation).normalize();
                // let r = u.cross(n).normalize();
                // let u_linha = n.cross(r).normalize();
                // let mat = Mat3::from_cols(r, u_linha, n);
                // let rotation = Quat::from_mat3(&mat);

                // axial-locked view-point billboard
                // let u = Vec3::Y;
                // let n = -(_camera_position - _global_transform.translation).normalize();
                // let r = u.cross(n).normalize();
                // let n_linha = r.cross(u).normalize();
                // let mat = Mat3::from_cols(r, u, n_linha);
                // let rotation = Quat::from_mat3(&mat);
                transform.rotation = rotation;
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
                        direction: vec3(0.0, 0.0, 0.0),
                    })
                    .insert_bundle(SimpleMeshBundle {
                        mesh: particle_system.particle_mesh.clone(),
                        color: particle_system.particle_color,
                        image: particle_system.image.clone(),
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
    Sphere(f32),
    Cone(f32),
}

impl SpawnShape {
    fn gen_random_translation(&self) -> Vec3 {
        let mut rng = rand::thread_rng();

        match self {
            Self::Rectangle(x, y) => {
                let x_size = x / 2.0;
                let y_size = y / 2.0;

                return vec3(
                    rng.gen_range(-x_size..x_size),
                    0.,
                    rng.gen_range(-y_size..y_size),
                );
            }
            Self::Circle(r) => {
                let radius = r * rng.gen::<f32>().sqrt();
                let theta= rng.gen::<f32>() * 2.0 * PI;

                let x = radius * theta.cos();
                let y = radius * theta.sin();
                return vec3(x, 0., y);
            },
            _ => vec3(0., 0., 0.),
            // Self::Cone(a, y) => {
            //     let radius = r * rng.gen::<f32>().sqrt();
            //     let theta= rng.gen::<f32>() * 2.0 * PI;

            //     let x = radius * theta.cos();
            //     let y = radius * theta.sin();
            //     return vec3(x, 0., y);
            // }
        }
    }

    fn get_direction(&self) -> Vec3 {        
        match self {
            Self::Rectangle(_x, _y) => vec3(0., 1., 0.),
            Self::Circle(_) => vec3(0., 1., 0.),
            Self::Sphere(_) => {
                let mut rng = rand::thread_rng();

                vec3(rng.gen::<f32>() * 2.0 - 1.0, rng.gen::<f32>() * 2.0 - 1.0, rng.gen::<f32>() * 2.0 - 1.0).normalize()
            },
            Self::Cone(r) => {
                let mut rng = rand::thread_rng();

                let radius = r * rng.gen::<f32>().sqrt();
                let theta= rng.gen::<f32>() * 2.0 * PI;

                let x = radius * theta.cos();
                let y = radius * theta.sin();
                return vec3(x, 1., y).normalize();
            }
        }
    }
}
