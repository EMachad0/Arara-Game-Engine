#![allow(unused_variables)]

pub mod runnable;
pub mod window_props;
pub mod window;
pub mod event_loop;

pub use runnable::*;
pub use window_props::*;
pub use window::*;
pub use event_loop::*;

pub mod prelude {
    pub use crate::{
        window_props::WindowProps,
        window::Window,
    };
}

use arara_app::{AppBuilder, Plugin};

#[derive(Default)]
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .init_non_send_resource::<EventLoop>()
            .init_resource::<WindowProps>()
            .init_non_send_resource::<Window>()
            .set_runnable(run);
    }
}
