// Image Manipulation Module

use colored::*;
use image::imageops::FilterType;

use self::universal::Universal;

pub mod types;
pub mod universal;

pub struct ImgMan {}

impl ImgMan {
    pub fn new() -> Self {
        return Self {};
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
