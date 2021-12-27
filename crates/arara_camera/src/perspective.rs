use std::f32::consts::FRAC_PI_3;

use arara_ecs::{event::EventReader, system::ResMut};
use arara_window::WindowResized;
use glam::Mat4;

#[derive(Debug)]
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

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

pub fn process_resize(
    mut mouse_motion_event_reader: EventReader<WindowResized>,
    mut perspective: ResMut<Perspective>,
) {
    for ev in mouse_motion_event_reader.iter().last() {
        perspective.resize(ev.width, ev.height);
    }
}
