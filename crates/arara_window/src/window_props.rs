use crate::WindowMode;

#[derive(Debug, Clone)]
pub struct WindowProps {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub vsync: bool,
    pub resizable: bool,
    pub decorations: bool,
    pub cursor_visible: bool,
    pub cursor_locked: bool,
    pub mode: WindowMode,
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: "Arara title".into(),
            width: 1024,
            height: 768,
            vsync: true,
            resizable: true,
            decorations: true,
            cursor_locked: false,
            cursor_visible: true,
            mode: WindowMode::Windowed,
        }
    }
}
