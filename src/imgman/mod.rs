// Image Manipulation Module

use colored::*;
use image::imageops::FilterType;

use self::universal::Universal;

pub mod types;
pub mod universal;

pub struct ImgMan {}

impl Default for ImgMan {
    fn default() -> Self {
        Self::new()
    }
}

impl ImgMan {
    pub fn new() -> Self {
        Self {}
    }

    pub fn resize(image: Universal, width: u32, height: u32) {
        let img = image.img;

        match img.decode() {
            Err(err) => {
                println!("{} {:#?}", "[IMGMAN]: ".magenta(), err);
            }
            Ok(dimg) => {
                let results = image::imageops::resize(&dimg, width, height, FilterType::Gaussian);
                println!("{} {:#?} {:#?}", "[IMGMAN]: ".magenta(), dimg, results);
            }
        }
    }
}
