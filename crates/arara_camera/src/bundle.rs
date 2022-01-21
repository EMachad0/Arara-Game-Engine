use crate::{Camera, FlyCamera, Perspective};
use arara_ecs::bundle::Bundle;
use arara_transform::{GlobalTransform, Transform};

/// Component bundle for camera entities with perspective projection
///
/// Use this for 3D rendering.
#[derive(Bundle, Default)]
pub struct PerspectiveCameraBundle {
    pub camera: Camera,
    pub projection: Perspective,
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
    pub projection: Perspective,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
