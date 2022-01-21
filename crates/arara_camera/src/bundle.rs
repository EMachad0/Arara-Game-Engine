use crate::{Camera, FlyCamera, PerspectiveProjection, OrthographicProjection};
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
#[derive(Bundle, Default)]
pub struct OrthographicCameraBundle {
    pub camera: Camera,
    pub projection: OrthographicProjection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl OrthographicCameraBundle {
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
    /// The camera is placed at `Z=+1000-0.1`, looking toward the world origin `(0,0,0)`.
    /// Its orthographic projection extends from `1000.0` to `-1000.0` in camera view space,
    /// corresponding to `Z=+999.9` (closest to camera) to `Z=-999.9` (furthest away from
    /// camera) in world space.
    pub fn new_2d() -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let projection = OrthographicProjection::default();
        let transform = Transform::from_xyz(0.0, 0.0, 512.01); //.looking_at_xyz(0.0, 0.0, 0.0);
        OrthographicCameraBundle {
            camera: Camera::default(),
            transform,
            global_transform: Default::default(),
            projection,
        }
    }
}

