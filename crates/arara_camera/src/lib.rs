mod bundle;
mod camera;
mod fly_camera;
mod projection;

pub use bundle::*;
pub use camera::*;
pub use fly_camera::*;
pub use projection::*;

pub mod prelude {
    pub use crate::{
        bundle::{FlyCameraBundle, PerspectiveCameraBundle, OrthographicCameraBundle},
        camera::Camera,
        fly_camera::FlyCamera,
        projection::{PerspectiveProjection, OrthographicProjection},
    };
}

use arara_app::{App, CoreStage, Plugin};

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(process_resize::<PerspectiveProjection>);
        app.add_system(process_resize::<OrthographicProjection>);
    }
}

#[derive(Default)]
pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, fly_camera_creation)
            .add_system(camera_movement_system)
            .add_system(mouse_motion_system);
        // .add_system(camera_2d_movement_system)
    }
}
