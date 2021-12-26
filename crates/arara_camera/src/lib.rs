mod camera;
mod fly_camera;
mod perspective;

pub use camera::*;
pub use fly_camera::*;
pub use perspective::*;

pub mod prelude {
    pub use crate::{camera::Camera, fly_camera::FlyCamera, perspective::Perspective};
}

use arara_app::{App, CoreStage, Plugin};

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Camera>()
            .init_resource::<Perspective>()
            .add_system(process_resize);
    }
}

#[derive(Default)]
pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlyCamera>()
            .add_system(process_mouse_motion)
            .add_system(process_scroll)
            .add_system(process_keyboard)
            .add_system_to_stage(CoreStage::PostUpdate, update_camera);
    }
}
