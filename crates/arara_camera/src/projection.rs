use std::f32::consts::FRAC_PI_3;

use arara_ecs::component::Component;
use arara_ecs::event::EventReader;
use arara_ecs::system::Query;
use arara_window::WindowResized;
use glam::Mat4;

use crate::Camera;

pub trait CameraProjection {
    fn calc_matrix(&self) -> Mat4;
    fn resize(&mut self, width: u32, height: u32);
}

#[derive(Debug, Component)]
pub struct Perspective {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Default for Perspective {
    fn default() -> Self {
        Self::new(1024, 768, FRAC_PI_3, 0.1, 1024.0)
    }
}

impl Perspective {
    pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }
}

impl CameraProjection for Perspective {
    fn calc_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fovy, self.aspect, self.znear, self.zfar)
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

pub fn process_resize<T: CameraProjection + Component>(
    mut event_reader: EventReader<WindowResized>,
    mut projections: Query<(&mut Camera, &mut T)>,
) {
    for ev in event_reader.iter().last() {
        for (mut camera, mut projection) in projections.iter_mut() {
            projection.resize(ev.width, ev.height);
            camera.projection = projection.calc_matrix();
        }
    }
}
