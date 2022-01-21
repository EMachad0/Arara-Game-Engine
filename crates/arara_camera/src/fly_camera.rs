//! A simple plugin and components for 2d/3d flying cameras in Arara.
//!	Copy from https://github.com/mcpar-land/bevy_fly_camera
//!
//! # 3D
//!
//! Movement system is based on Minecraft, flying along the horizontal plane no matter the mouse's vertical angle, with two extra buttons for moving vertically.
//!
//! Default keybinds are:
//! - <kbd>W</kbd> / <kbd>A</kbd> / <kbd>S</kbd> / <kbd>D</kbd> - Move along the horizontal plane
//! - Shift - Move downward
//! - Space - Move upward
//!
//! ## Example
//! ```no_compile
//! use Arara::prelude::*;
//! use Arara_fly_camera::{FlyCamera, FlyCameraPlugin};
//!
//! fn setup(commands: &mut Commands) {
//!	  commands
//!     .spawn(Camera3dBundle::default())
//!     .with(FlyCamera::default());
//! }
//!
//! fn main() {
//!	  App::build()
//!     .add_plugins(DefaultPlugins)
//!     .add_startup_system(setup.system())
//!     .add_plugin(FlyCameraPlugin)
//!     .run();
//! }
//! ```
//!
//! There's also a basic piece of example code included in `/examples/basic.rs`
//!
//! # 2D
//! Movement system only uses the keyboard to move in all four directions across the 2d plane.
//!
//! The default keybinds are:
//! - <kbd>W</kbd> / <kbd>A</kbd> / <kbd>S</kbd> / <kbd>D</kbd> - Move along the 2d plane
//!
//! ## Example
//! ```no_compile
//! use Arara::prelude::*;
//! use Arara_fly_camera::{FlyCamera2d, FlyCameraPlugin};
//! ```
//! ```no_compile
//!	commands
//!   .spawn(Camera2dBundle::default())
//!   .with(FlyCamera2d::default());
//! ```
//!
//! There's also a basic piece of example code included in `/examples/2d.rs`

use arara_ecs::{
    component::Component,
    event::EventReader,
    query::Added,
    system::{Query, Res},
};
use arara_input::{
    keyboard::KeyCode,
    mouse::{MouseButton, MouseMotion},
    Input,
};
use arara_time::Time;
use arara_transform::{GlobalTransform, Transform};
use glam::{Quat, Vec3};

pub fn movement_axis(input: &Res<Input<KeyCode>>, plus: KeyCode, minus: KeyCode) -> f32 {
    let mut axis = 0.0;
    if input.pressed(plus) {
        axis += 1.0;
    }
    if input.pressed(minus) {
        axis -= 1.0;
    }
    axis
}

/// A set of options for initializing a FlyCamera.
/// Attach this component to a [`Camera3dBundle`] bundle to control it with your mouse and keyboard.
/// # Example
/// ```no_compile
/// fn setup(mut commands: Commands) {
///	  commands
///     .spawn(Camera3dBundle::default())
///     .with(FlyCamera::default());
/// }

#[derive(Component)]
pub struct FlyCamera {
    /// The speed the FlyCamera accelerates at. Defaults to `1.0`
    pub accel: f32,
    /// The maximum speed the FlyCamera can move at. Defaults to `0.5`
    pub max_speed: f32,
    /// The sensitivity of the FlyCamera's motion based on mouse movement. Defaults to `3.0`
    pub sensitivity: f32,
    /// The amount of deceleration to apply to the camera's motion. Defaults to `1.0`
    pub friction: f32,
    /// The current pitch of the FlyCamera in degrees. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
    pub pitch: f32,
    /// The current pitch of the FlyCamera in degrees. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
    pub yaw: f32,
    /// The current velocity of the FlyCamera. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
    pub velocity: Vec3,
    /// Key used to move forward. Defaults to <kbd>W</kbd>
    pub key_forward: KeyCode,
    /// Key used to move backward. Defaults to <kbd>S</kbd>
    pub key_backward: KeyCode,
    /// Key used to move left. Defaults to <kbd>A</kbd>
    pub key_left: KeyCode,
    /// Key used to move right. Defaults to <kbd>D</kbd>
    pub key_right: KeyCode,
    /// Key used to move up. Defaults to <kbd>Space</kbd>
    pub key_up: KeyCode,
    /// Key used to move forward. Defaults to <kbd>LShift</kbd>
    pub key_down: KeyCode,
    /// If `false`, disable keyboard control of the camera. Defaults to `true`
    pub enabled: bool,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            accel: 1.5,
            max_speed: 0.5,
            sensitivity: 3.0,
            friction: 1.0,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            key_forward: KeyCode::W,
            key_backward: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::Space,
            key_down: KeyCode::LControl,
            enabled: true,
        }
    }
}

fn forward_vector(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(Vec3::Z).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
    let f = forward_vector(rotation);
    let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
    f_flattened
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
    // Rotate it 90 degrees to get the strafe direction
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_walk_vector(rotation))
        .normalize()
}

pub(crate) fn fly_camera_creation(
    mut query: Query<(&mut FlyCamera, &GlobalTransform), Added<FlyCamera>>,
) {
    for (mut fly_camera, transform) in query.iter_mut() {
        let (yaw, pitch, _) = transform.rotation.to_euler(glam::EulerRot::YXZ);
        fly_camera.yaw = yaw.to_degrees();
        fly_camera.pitch = -pitch.to_degrees();
    }
}

pub(crate) fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
    for (mut options, mut transform) in query.iter_mut() {
        let (axis_h, axis_v, axis_float) = if options.enabled {
            (
                movement_axis(&keyboard_input, options.key_right, options.key_left),
                movement_axis(&keyboard_input, options.key_backward, options.key_forward),
                movement_axis(&keyboard_input, options.key_up, options.key_down),
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        let rotation = transform.rotation;
        let accel: Vec3 = (strafe_vector(&rotation) * axis_h)
            + (forward_walk_vector(&rotation) * axis_v)
            + (Vec3::Y * axis_float);
        let accel: Vec3 = if accel.length() != 0.0 {
            accel.normalize() * options.accel
        } else {
            Vec3::ZERO
        };

        let friction: Vec3 = if options.velocity.length() != 0.0 {
            options.velocity.normalize() * -1.0 * options.friction
        } else {
            Vec3::ZERO
        };

        options.velocity += accel * time.delta_seconds();

        // clamp within max speed
        if options.velocity.length() > options.max_speed {
            options.velocity = options.velocity.normalize() * options.max_speed;
        }

        let delta_friction = friction * time.delta_seconds();

        options.velocity =
            if (options.velocity + delta_friction).signum() != options.velocity.signum() {
                Vec3::ZERO
            } else {
                options.velocity + delta_friction
            };

        transform.translation += options.velocity;
    }
}

pub(crate) fn mouse_motion_system(
    time: Res<Time>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
    let mut dx = 0.0;
    let mut dy = 0.0;
    for event in mouse_motion_event_reader.iter() {
        let (x, y) = event.delta;
        dx += x;
        dy += y;
    }

    for (mut options, mut transform) in query.iter_mut() {
        if !options.enabled {
            continue;
        }

        if mouse_input.pressed(MouseButton::Left) {
            options.yaw -= dx * options.sensitivity * time.delta_seconds();
            options.pitch += dy * options.sensitivity * time.delta_seconds();
        }

        options.pitch = options.pitch.clamp(-89.0, 89.9);

        let yaw_radians = options.yaw.to_radians();
        let pitch_radians = options.pitch.to_radians();

        transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians)
            * Quat::from_axis_angle(-Vec3::X, pitch_radians);
    }
}
