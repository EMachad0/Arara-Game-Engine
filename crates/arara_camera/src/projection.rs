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
pub struct PerspectiveProjection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        Self::new(1024, 768, FRAC_PI_3, 0.1, 1024.0)
    }
}

impl PerspectiveProjection {
    pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }
}

impl CameraProjection for PerspectiveProjection {
    fn calc_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fovy, self.aspect, self.znear, self.zfar)
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

// TODO: make this a component instead of a property
#[derive(Debug, Clone)]
pub enum WindowOrigin {
    Center,
    BottomLeft,
}

#[derive(Debug, Clone)]
pub enum ScalingMode {
    /// Manually specify left/right/top/bottom values.
    /// Ignore window resizing; the image will stretch.
    None,
    /// Match the window size. 1 world unit = 1 pixel.
    WindowSize,
    /// Keep vertical axis constant; resize horizontal with aspect ratio.
    FixedVertical,
    /// Keep horizontal axis constant; resize vertical with aspect ratio.
    FixedHorizontal,
}

#[derive(Component, Debug, Clone)]
pub struct OrthographicProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub window_origin: WindowOrigin,
    pub scaling_mode: ScalingMode,
    pub scale: f32,
}

impl CameraProjection for OrthographicProjection {
    fn calc_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh_gl(
            self.left * self.scale,
            self.right * self.scale,
            self.bottom * self.scale,
            self.top * self.scale,
            self.near,
            self.far,
        )
    }

    fn resize(&mut self, width: u32, height: u32) {
        let width = width as f32;
        let height = height as f32;
        match (&self.scaling_mode, &self.window_origin) {
            (ScalingMode::WindowSize, WindowOrigin::Center) => {
                let half_width = width / 2.0;
                let half_height = height / 2.0;
                self.left = -half_width;
                self.right = half_width;
                self.top = half_height;
                self.bottom = -half_height;
            }
            (ScalingMode::WindowSize, WindowOrigin::BottomLeft) => {
                self.left = 0.0;
                self.right = width;
                self.top = height;
                self.bottom = 0.0;
            }
            (ScalingMode::FixedVertical, WindowOrigin::Center) => {
                let aspect_ratio = width / height;
                self.left = -aspect_ratio;
                self.right = aspect_ratio;
                self.top = 1.0;
                self.bottom = -1.0;
            }
            (ScalingMode::FixedVertical, WindowOrigin::BottomLeft) => {
                let aspect_ratio = width / height;
                self.left = 0.0;
                self.right = aspect_ratio;
                self.top = 1.0;
                self.bottom = 0.0;
            }
            (ScalingMode::FixedHorizontal, WindowOrigin::Center) => {
                let aspect_ratio = height / width;
                self.left = -1.0;
                self.right = 1.0;
                self.top = aspect_ratio;
                self.bottom = -aspect_ratio;
            }
            (ScalingMode::FixedHorizontal, WindowOrigin::BottomLeft) => {
                let aspect_ratio = height / width;
                self.left = 0.0;
                self.right = 1.0;
                self.top = aspect_ratio;
                self.bottom = 0.0;
            }
            (ScalingMode::None, _) => {}
        }
    }
}

impl Default for OrthographicProjection {
    fn default() -> Self {
        OrthographicProjection {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.01,
            far: 1024.0,
            window_origin: WindowOrigin::BottomLeft,
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.0,
        }
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
