mod camera;
mod perspective;
mod fly_camera;

pub use camera::*;
pub use perspective::*;
pub use fly_camera::*;

pub mod prelude {
    pub use crate::{
        perspective::Perspective,
        camera::Camera,
        fly_camera::FlyCamera,
    };
}

use bevy_ecs::prelude::IntoSystem;
use arara_app::prelude::*;

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
