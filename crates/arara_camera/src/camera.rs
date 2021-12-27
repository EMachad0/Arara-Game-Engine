use std::f32::consts::FRAC_PI_2;

use glam::{vec3, Mat4, Vec3};

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(vec3(0.0, 5.0, 10.0), -FRAC_PI_2, (-20f32).to_radians())
    }
}

impl Camera {
    pub fn new(position: Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            position,
            yaw,
            pitch,
        }
    }

    pub fn calc_matrix(&self) -> Mat4 {
        let mut up = Vec3::Y;
        let normal = vec3(self.yaw.cos(), self.pitch.sin(), self.yaw.sin()).normalize();
        let right = normal.cross(up).normalize();
        up = right.cross(normal).normalize();
        Mat4::from_cols_array(&[
            right.x,
            up.x,
            -normal.x,
            0.0,
            right.y,
            up.y,
            -normal.y,
            0.0,
            right.z,
            up.z,
            -normal.z,
            0.0,
            -self.position.dot(right),
            -self.position.dot(up),
            self.position.dot(normal),
            1.0,
        ])
    }
}
