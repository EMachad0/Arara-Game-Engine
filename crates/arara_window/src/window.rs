use glium::{self, Display, glutin::{self, dpi, event_loop::EventLoop}};
use crate::window_props::WindowProps;

#[derive(Default)]
pub struct Window {
    display: Option<Display>,
}

impl Window {
    pub fn build_display(&mut self, window_props: &WindowProps, event_loop: &EventLoop<()>) {
        let size = dpi::LogicalSize::new(window_props.width, window_props.height);
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title(window_props.title.clone());
        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(window_props.vsync);
        self.display = Some(Display::new(wb, cb, event_loop).unwrap())
    }

    pub fn display(&self) -> &Display {
        self.display.as_ref().unwrap()
    }
}
