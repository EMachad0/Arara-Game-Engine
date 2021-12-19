mod camera;
mod fly_camera;
mod perspective;

pub use camera::*;
pub use fly_camera::*;
pub use perspective::*;

pub mod prelude {
    pub use crate::{camera::Camera, fly_camera::FlyCamera, perspective::Perspective};
}

use arara_app::prelude::*;
use bevy_ecs::prelude::IntoSystem;

#[derive(Default)]
pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .init_resource::<FlyCamera>()
            .add_system(process_mouse_motion.system())
            .add_system(process_resize.system())
            .add_system(process_scroll.system())
            .add_system(process_keyboard.system())
            .add_system_to_stage(CoreStage::PostUpdate, update_camera.system());
    }
}
