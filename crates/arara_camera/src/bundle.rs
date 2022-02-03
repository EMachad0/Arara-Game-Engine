use crate::{
    Camera, FlyCamera, FlyCamera2d, OrthographicProjection, PerspectiveProjection, WorldMouse2d,
};
use arara_ecs::bundle::Bundle;
use arara_transform::{GlobalTransform, Transform};

/// Component bundle for camera entities with perspective projection
///
/// Use this for 3D rendering.
#[derive(Bundle, Default)]
pub struct PerspectiveCameraBundle {
    pub camera: Camera,
    pub projection: PerspectiveProjection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// Component bundle for camera entities with perspective projection and keyboard + mouse control
///
/// Use this for 3D rendering and debuging.
#[derive(Bundle, Default)]
pub struct FlyCameraBundle {
    pub camera: Camera,
    pub fly_camera: FlyCamera,
    pub projection: PerspectiveProjection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// Component bundle for camera entities with orthographic projection
///
/// Use this for 2D games, isometric games, CAD-like 3D views.
#[derive(Bundle)]
pub struct OrthographicCameraBundle {
    pub camera: Camera,
    pub projection: OrthographicProjection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub mouse: WorldMouse2d,
}

impl Default for OrthographicCameraBundle {
    /// Create an orthographic projection camera to render 2D content.
    ///
    /// The projection creates a camera space where X points to the right of the screen,
    /// Y points to the top of the screen, and Z points out of the screen (backward),
    /// forming a right-handed coordinate system. The center of the screen is at `X=0` and
    /// `Y=0`.
    ///
    /// The default scaling mode is [`ScalingMode::WindowSize`], resulting in a resolution
    /// where 1 unit in X and Y in camera space corresponds to 1 logical pixel on the screen.
    /// That is, for a screen of 1920 pixels in width, the X coordinates visible on screen go
    /// from `X=-960` to `X=+960` in world space, left to right. This can be changed by changing
    /// the [`OrthographicProjection::scaling_mode`] field.
    ///
    /// The camera is placed at `Z=+512+0.01`, looking toward the world origin `(0,0,0)`.
    /// Its orthographic projection extends from `512.0` to `-512.0` in camera view space,
    /// corresponding to `Z=+512.0` (closest to camera) to `Z=-512.0` (furthest away from
    /// camera) in world space.
    fn default() -> Self {
        Self {
            transform: Transform::from_xyz(0.0, 0.0, 512.01),
            camera: Default::default(),
            projection: Default::default(),
            global_transform: Default::default(),
            mouse: Default::default(),
        }
    }
}

/// Component bundle for camera entities with perspective projection and keyboard + mouse control
///
/// Use this for 2D games, isometric games, CAD-like 3D views.
#[derive(Bundle)]
pub struct FlyCamera2dBundle {
    pub camera: Camera,
    pub fly_camera: FlyCamera2d,
    pub projection: OrthographicProjection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub mouse: WorldMouse2d,
}

impl Default for FlyCamera2dBundle {
    fn default() -> Self {
        Self {
            transform: Transform::from_xyz(0.0, 0.0, 512.01),
            fly_camera: FlyCamera2d::default(),
            camera: Default::default(),
            projection: Default::default(),
            global_transform: Default::default(),
            mouse: Default::default(),
        }
    }
}
