use super::super::misc::{config::THUMBNAIL_SIZE, Result};
use super::{Media, Thumbnail, MediaType};
use image::{DynamicImage, ImageBuffer, ImageFormat};

impl Media {
    pub fn thumbnail(&self) -> Thumbnail {
        match self.kind {
            MediaType::Audio =>
        }
    }
}

impl Thumbnail {
    pub fn from_image(image: DynamicImage) -> Self {
        Self {
            image: image.thumbnail(THUMBNAIL_SIZE.0, THUMBNAIL_SIZE.1)
        }
    }

    pub fn from_text(text: &str) -> Self {
        Self {
        }
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        self.image
            .write_to(writer, ImageFormat::Png)
            .expect("Cannot write.");
        Ok(())
    }
}
