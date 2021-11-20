use std::path::PathBuf;

/// A window event that is sent whenever a window has been resized.
#[derive(Debug, Clone)]
pub struct WindowResized {
    pub width: u32,
    pub height: u32,
}

/// An event that indicates a window should be closed.
#[derive(Debug, Clone)]
pub struct CloseWindow;

/// An event that is sent whenever a new window is created.
#[derive(Debug, Clone)]
pub struct WindowCreated;

/// An event that is sent whenever a close was requested for a window. For example: when the "close"
/// button is pressed on a window.
#[derive(Debug, Clone)]
pub struct WindowCloseRequested;

#[derive(Debug, Clone)]
pub struct CursorMoved {
    pub position: (f32, f32),
}

#[derive(Debug, Clone)]
pub struct CursorEntered;

#[derive(Debug, Clone)]
pub struct CursorLeft;

/// An event that is sent whenever a window receives a character from the OS or underlying system.
#[derive(Debug, Clone)]
pub struct ReceivedCharacter {
    pub char: char,
}

/// An event that indicates a window has received or lost focus.
#[derive(Debug, Clone)]
pub struct WindowFocused {
    pub focused: bool,
}

/// An event that indicates a window's scale factor has changed.
#[derive(Debug, Clone)]
pub struct WindowScaleFactorChanged {
    pub scale_factor: f64,
}
/// An event that indicates a window's OS-reported scale factor has changed.
#[derive(Debug, Clone)]
pub struct WindowBackendScaleFactorChanged {
    pub scale_factor: f64,
}

/// Events related to files being dragged and dropped on a window.
#[derive(Debug, Clone)]
pub enum FileDragAndDrop {
    DroppedFile { path_buf: PathBuf },
    HoveredFile { path_buf: PathBuf },
    HoveredFileCancelled,
}

/// An event that is sent when a window is repositioned in physical pixels.
#[derive(Debug, Clone)]
pub struct WindowMoved {
    pub position: (i32, i32),
}
