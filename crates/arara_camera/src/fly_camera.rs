use std::f32::consts::FRAC_PI_2;

use crate::camera::Camera;
use arara_ecs::prelude::*;
use arara_input::keyboard::KeyCode;
use arara_input::mouse::{MouseButton, MouseMotion, MouseScrollUnit, MouseWheel};
use arara_input::Input;
use arara_time::prelude::*;
use glam::{vec3, Vec3};

#[derive(Debug)]
pub struct FlyCamera {
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
    enabled: bool,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed: 5.0,
            sensitivity: 0.5,
            enabled: true,
        }
    }
}

impl FlyCamera {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            ..Default::default()
        }
    }
}

pub fn process_mouse_motion(
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut fly_camera: ResMut<FlyCamera>,
) {
    if !fly_camera.enabled {
        return;
    }
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    let mut mouse_dx = 0.0;
    let mut mouse_dy = 0.0;
    for event in mouse_motion_event_reader.iter() {
        let (x, y) = event.delta;
        mouse_dx += x;
        mouse_dy += y;
    }
    if mouse_dx == 0.0 && mouse_dy == 0.0 {
        return;
    }

    fly_camera.rotate_horizontal = mouse_dx as f32;
    fly_camera.rotate_vertical = mouse_dy as f32;
}

pub fn process_keyboard(keyboard_input: Res<Input<KeyCode>>, mut fly_camera: ResMut<FlyCamera>) {
    if !fly_camera.enabled {
        return;
    }
    let amount = if keyboard_input.pressed(KeyCode::LShift) {
        2.5
    } else {
        1.0
    };
    fly_camera.amount_forward = if keyboard_input.pressed(KeyCode::W) {
        amount
    } else {
        0.0
    };
    fly_camera.amount_backward = if keyboard_input.pressed(KeyCode::S) {
        amount
    } else {
        0.0
    };
    fly_camera.amount_left = if keyboard_input.pressed(KeyCode::A) {
        amount
    } else {
        0.0
    };
    fly_camera.amount_right = if keyboard_input.pressed(KeyCode::D) {
        amount
    } else {
        0.0
    };
    fly_camera.amount_up = if keyboard_input.pressed(KeyCode::Space) {
        amount
    } else {
        0.0
    };
    fly_camera.amount_down = if keyboard_input.pressed(KeyCode::LControl) {
        amount
    } else {
        0.0
    };
}

pub fn process_scroll(
    mut fly_camera: ResMut<FlyCamera>,
    mut mouse_wheel_event_reader: EventReader<MouseWheel>,
) {
    for ev in mouse_wheel_event_reader.iter() {
        fly_camera.scroll = match ev.unit {
            // I'm assuming a line is about 10 pixels
            MouseScrollUnit::Line => ev.y * 10.0,
            MouseScrollUnit::Pixel => ev.y,
        };
    }
}

pub fn update_camera(
    time: Res<Time>,
    mut fly_camera: ResMut<FlyCamera>,
    mut camera: ResMut<Camera>,
) {
    let dt = time.delta_seconds();

    let mut position = Vec3::ZERO;
    let mut yaw = 0.0;
    let mut pitch = 0.0;

    // Move forward/backward and left/right
    let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
    let forward = vec3(yaw_cos, 0.0, yaw_sin).normalize();
    let right = vec3(-yaw_sin, 0.0, yaw_cos).normalize();
    position +=
        forward * (fly_camera.amount_forward - fly_camera.amount_backward) * fly_camera.speed * dt;
    position += right * (fly_camera.amount_right - fly_camera.amount_left) * fly_camera.speed * dt;

    // Move in/out (aka. "zoom")
    // Note: this isn't an actual zoom. The camera's position
    // changes when zooming. I've added this to make it easier
    // to get closer to an object you want to focus on.
    let (pitch_sin, pitch_cos) = camera.pitch.sin_cos();
    let scrollward = vec3(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
    position += scrollward * fly_camera.scroll * fly_camera.speed * fly_camera.sensitivity * dt;
    fly_camera.scroll = 0.0;

    // Move up/down. Since we don't use roll, we can just
    // modify the y coordinate directly.
    position.y += (fly_camera.amount_up - fly_camera.amount_down) * fly_camera.speed * dt;

    // Rotate
    yaw += (fly_camera.rotate_horizontal) * fly_camera.sensitivity * dt;
    pitch += (-fly_camera.rotate_vertical) * fly_camera.sensitivity * dt;

    camera.position += position;
    camera.yaw += yaw;
    camera.pitch += pitch;

    // If process_mouse isn't called every frame, these values
    // will not get set to zero, and the camera will rotate
    // when moving in a non cardinal direction.
    fly_camera.rotate_horizontal = 0.0;
    fly_camera.rotate_vertical = 0.0;

    // Keep the self.camera's angle from going too high/low.
    if camera.pitch < -(FRAC_PI_2) {
        camera.pitch = -(FRAC_PI_2);
    } else if camera.pitch > (FRAC_PI_2) {
        camera.pitch = FRAC_PI_2;
    }
}
