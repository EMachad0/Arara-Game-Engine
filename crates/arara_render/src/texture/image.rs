use super::converters::dynamic_image_to_image;
use bevy_reflect::TypeUuid;
use thiserror::Error;

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "6ea26da6-6cf8-4ea2-9986-1d7bf6c17d6f"]
pub struct Image {
    pub data: Vec<u8>,
    pub dimensions: (u32, u32),
    pub translucent: bool,
}

impl Default for Image {
    fn default() -> Self {
        let data = vec![255; 4];
        Self::new(data, (1, 1), false)
    }
}

impl Image {
    /// Creates a new image from raw binary data and the corresponding metadata.
    ///
    /// # Panics
    /// Panics if the length of the `data`, volume of the `size` and the size of the `format`
    /// do not match.
    pub fn new(data: Vec<u8>, dimensions: (u32, u32), translucent: bool) -> Self {
        Self {
            data,
            dimensions,
            translucent,
        }
    }

    /// Load a bytes buffer in a [`Texture`], according to type `image_type`, using the `image`
    /// crate`
    pub fn from_buffer(buffer: &[u8], image_type: ImageType) -> Result<Image, TextureError> {
        let format = match image_type {
            ImageType::MimeType(mime_type) => match mime_type {
                "image/png" => Ok(image::ImageFormat::Png),
                "image/jpeg" => Ok(image::ImageFormat::Jpeg),
                // "image/bmp" => Ok(image::ImageFormat::Bmp),
                // "image/vnd-ms.dds" => Ok(image::ImageFormat::Dds),
                // "image/x-targa" => Ok(image::ImageFormat::Tga),
                // "image/x-tga" => Ok(image::ImageFormat::Tga),
                // "image/x-bmp" => Ok(image::ImageFormat::Bmp),
                _ => Err(TextureError::InvalidImageMimeType(mime_type.to_string())),
            },
            ImageType::Extension(extension) => image::ImageFormat::from_extension(extension)
                .ok_or_else(|| TextureError::InvalidImageExtension(extension.to_string())),
        }?;

        // Load the image in the expected format.
        // Some formats like PNG allow for R or RG textures too, so the texture
        // format needs to be determined. For RGB textures an alpha channel
        // needs to be added, so the image data needs to be converted in those
        // cases.

        let dyn_img = image::load_from_memory_with_format(buffer, format)?;
        Ok(dynamic_image_to_image(dyn_img))
    }
}

/// An error that occurs when loading a texture
#[derive(Error, Debug)]
pub enum TextureError {
    #[error("invalid image mime type")]
    InvalidImageMimeType(String),
    #[error("invalid image extension")]
    InvalidImageExtension(String),
    #[error("failed to load an image: {0}")]
    ImageError(#[from] image::ImageError),
}

/// The type of a raw image buffer.
pub enum ImageType<'a> {
    /// The mime type of an image, for example `"image/png"`.
    #[allow(dead_code)]
    MimeType(&'a str),
    /// The extension of an image file, for example `"png"`.
    Extension(&'a str),
}
