pub struct WindowProps {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub vsync: bool,
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            title: "window title".into(),
            vsync: true,
        }
    }
}
