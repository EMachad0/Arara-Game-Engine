use glium::{
    self,
    Display,
    glutin::{
        self,
        dpi
    }
};
use bevy_ecs::world::{World, FromWorld};

use crate::{
    event_loop::EventLoop, 
    window_props::WindowProps
};

pub struct Window {
    display: Display,
}

impl FromWorld for Window {
    fn from_world(world: &mut World) -> Self {
        let event_loop = world.get_non_send_resource::<EventLoop>().unwrap();
        let window_props = world.get_resource::<WindowProps>().unwrap();

        let size = dpi::LogicalSize::new(window_props.width, window_props.height);
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title(window_props.title.clone());
        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(window_props.vsync);

        let display = Display::new(wb, cb, event_loop.borrow()).unwrap();
        Self {
            display,
        }
    }
}

impl Window {
    pub fn display(&self) -> &Display {
        &self.display
    }
}
