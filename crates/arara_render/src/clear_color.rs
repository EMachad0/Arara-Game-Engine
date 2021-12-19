use crate::Color;

pub struct ClearColor(pub Color);

impl Default for ClearColor {
    fn default() -> Self {
        Self(Color::rgb(0.2, 0.2, 0.2))
    }
}
