use crate::texture::Image;

/// Converts a [`DynamicImage`] to an [`Image`].
pub(crate) fn dynamic_image_to_image(dyn_img: image::DynamicImage) -> Image {
    // Remove this once a way to add diferent size textures is figured out
    let dyn_img = dyn_img.resize_exact(64, 64, image::imageops::FilterType::Gaussian);

    let translucent = match dyn_img {
        image::DynamicImage::ImageRgba8(_)
        | image::DynamicImage::ImageBgra8(_)
        | image::DynamicImage::ImageRgba16(_) => true,
        _ => false,
    };

    let dyn_img = dyn_img.into_rgba8();
    let dimensions = dyn_img.dimensions();
    let data: Vec<u8> = dyn_img.into_raw();
    Image::new(data, dimensions, translucent)
}
