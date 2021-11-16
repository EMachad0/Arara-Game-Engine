use crate::{
    camera::Camera,
    perspective::Perspective,
};

use cgmath::*;
use glium::glutin::event::*;
use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};
use std::time::Duration;
use std::f32::consts::FRAC_PI_2;


#[derive(Debug)]
pub struct CameraController {
    camera: Camera,
    perspective: Perspective,
    changed: bool,
    cached_matrix: [[f32; 4]; 4],
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new(1.0, 0.5)
    }
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            camera: Default::default(),
            perspective: Default::default(),
            changed: true,
            cached_matrix: Default::default(),
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        self.changed = true;
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LControl => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.changed = true;
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.changed = true;
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                y: scroll,
                ..
            }) => *scroll as f32,
        };
    }

    pub fn update_camera(&mut self, dt: Duration) {
        self.changed = true;
        let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = self.camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        self.camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        self.camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = self.camera.pitch.0.sin_cos();
        let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        self.camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        self.camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        self.camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        self.camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the self.camera's angle from going too high/low.
        if self.camera.pitch < -Rad(FRAC_PI_2) {
            self.camera.pitch = -Rad(FRAC_PI_2);
        } else if self.camera.pitch > Rad(FRAC_PI_2) {
            self.camera.pitch = Rad(FRAC_PI_2);
        }
    }

    pub fn resize_from_size(&mut self, new_size: PhysicalSize<u32>) {
        self.resize(new_size.width, new_size.height);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.changed = true;
        self.perspective.resize(width, height);
    }

    pub fn calc_matrix(&mut self) -> [[f32; 4]; 4] {
        if self.changed {
            let pv_matrix = self.perspective.calc_matrix() * self.camera.calc_matrix();
            self.cached_matrix = pv_matrix.into();
        }
        self.cached_matrix
    }
}
