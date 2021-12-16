mod color;
pub mod gradient;

pub use color::*;

pub enum ColorOrGradient {
    Color(Color),
    Gradient(gradient::Gradient),
}

impl ColorOrGradient {
    pub fn get_color(&self) -> Color {
        match self {
            ColorOrGradient::Color(color) => *color,
            ColorOrGradient::Gradient(gradient) => gradient.at(0.),
        }
    }
}