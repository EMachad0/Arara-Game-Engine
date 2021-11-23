use std::io::Cursor;

use image::*;

pub fn load_texture() -> DynamicImage {
    let image = image::load(Cursor::new(&include_bytes!("../../../assets/textures/joaozinho.png")),
        image::ImageFormat::Png).unwrap();
    image.resize(128, 128, imageops::FilterType::Gaussian)
}