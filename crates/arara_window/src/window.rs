use arara_ecs::world::{FromWorld, World};
use glium::{
    self,
    glutin::{self, dpi},
    Display,
};

use crate::{event_loop::EventLoop, window_props::WindowProps};

/// An operating system window that can present content and receive user input.
#[derive(Debug)]
pub struct Window {
    display: Display,
    width: u32,
    height: u32,
    title: String,
    vsync: bool,
    resizable: bool,
    decorations: bool,
    cursor_visible: bool,
    cursor_locked: bool,
    cursor_position: Option<(f32, f32)>,
    focused: bool,
    mode: WindowMode,
}

impl FromWorld for Window {
    fn from_world(world: &mut World) -> Self {
        let window_props = world
            .get_resource::<WindowProps>()
            .and_then(|w| Some(w.to_owned()))
            .unwrap_or_default();
        let event_loop = world.get_non_send_resource::<EventLoop>().unwrap();

        let size = dpi::LogicalSize::new(window_props.width, window_props.height);
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title(window_props.title.clone());
        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(window_props.vsync);

        let display = Display::new(wb, cb, event_loop.borrow().as_ref().unwrap()).unwrap();
        Self::new(display, &window_props)
    }
}

impl Window {
    pub fn new(display: Display, window_props: &WindowProps) -> Self {
        Window {
            display,
            width: window_props.width,
            height: window_props.height,
            title: window_props.title.clone(),
            vsync: window_props.vsync,
            resizable: window_props.resizable,
            decorations: window_props.decorations,
            cursor_visible: window_props.cursor_visible,
            cursor_locked: window_props.cursor_locked,
            cursor_position: None,
            focused: true,
            mode: window_props.mode,
        }
    }

    #[inline]
    pub fn display(&self) -> &Display {
        &self.display
    }

    /// The current logical width of the window's client area.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The current logical height of the window's client area.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn update_actual_size_from_backend(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    #[inline]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[inline]
    pub fn vsync(&self) -> bool {
        self.vsync
    }

    #[inline]
    pub fn resizable(&self) -> bool {
        self.resizable
    }

    #[inline]
    pub fn decorations(&self) -> bool {
        self.decorations
    }

    #[inline]
    pub fn cursor_locked(&self) -> bool {
        self.cursor_locked
    }

    #[inline]
    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    #[inline]
    pub fn cursor_position(&self) -> Option<(f32, f32)> {
        self.cursor_position
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn update_focused_status_from_backend(&mut self, focused: bool) {
        self.focused = focused;
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn update_cursor_position_from_backend(&mut self, cursor_position: Option<(f32, f32)>) {
        self.cursor_position = cursor_position;
    }

    #[inline]
    pub fn mode(&self) -> WindowMode {
        self.mode
    }

    #[inline]
    pub fn is_focused(&self) -> bool {
        self.focused
    }
}

/// Defines the way a window is displayed
/// The use_size option that is used in the Fullscreen variant
/// defines whether a videomode is chosen that best fits the width and height
/// in the Window structure, or if these are ignored.
/// E.g. when use_size is set to false the best video mode possible is chosen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen { use_size: bool },
}
