// Wrapper for Image File

use std::io::Cursor;

use image::io::Reader;

use crate::cache::File;

use super::types::get_imgman_format;

pub struct Universal {
    pub image_format: image::ImageFormat,
    pub core: File,
    pub img: Reader<Cursor<Vec<u8>>>,
}

impl Universal {
    pub fn new(image: File) -> Self {
        let image_format = get_imgman_format(image.format);
        let mut img = image::io::Reader::new(Cursor::new(image.contents.clone()));
        img.set_format(image_format);

        return Self {
            core: image,
            img,
            image_format,
        };
    }
}
