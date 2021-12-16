use glam::Vec3;
use rand::{self, Rng};

mod particle_plugin;
mod particle_system;

pub use particle_plugin::*;
pub use particle_system::*;

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
            }
        }
    }
}

pub struct Particle {
    pub time_remaining: f32,
    pub velocity: f32,
    pub direction: Vec3,
}
