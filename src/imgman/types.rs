use crate::cache::types::FileFormat;
use crate::cache::types::ImageFormat as CustomFormat;

pub fn get_imgman_format(format: FileFormat) -> image::ImageFormat {
    match format {
        FileFormat::IMAGE(i) => match i {
            CustomFormat::PNG => image::ImageFormat::Png,
            CustomFormat::JPEG => image::ImageFormat::Jpeg,
            CustomFormat::JPG => image::ImageFormat::Jpeg,
        },

        _ => {
            image::ImageFormat::WebP
        }
    }
}
