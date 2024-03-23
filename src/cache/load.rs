use std::fs::{self};
use uuid::Uuid;

use super::Cache;
use crate::{cache::CacheOptions, config::STORE};
use colored::*;

pub fn load_into(cache: &mut Cache) {
    match fs::read_dir(STORE) {
        Err(_) => {
            match fs::create_dir(STORE) {
                Ok(_) => {
                    println!(
                        "{} Initialized New Directory For Cache Storage at {:#?}, Re-run.",
                        "[CACHE]:".purple(),
                        STORE,
                    )
                }
                Err(_) => {
                    println!(
                        "{} Attempted To Create Cache Directory",
                        "[CACHE]:".purple()
                    )
                }
            };
        }
        Ok(dirs) => {
            for i in dirs.into_iter() {
                match i {
                    Err(error) => {
                        println!("{:#?}", error)
                    }
                    Ok(document) => {
                        let name = document.file_name().to_str().unwrap().to_string();
                        println!("{} {:#}", "[DOCUMENT]:".blue(), name);

                        // let path = format!("{}{}", STORE, name);
                        // let new_path = format!("{}{}", STORE, Uuid::new_v4());
                        // match fs::rename(path.clone(), new_path) {
                        //     Err(error) => {
                        //         println!("{:#?}", error);
                        //     }
                        //     Ok(_) => {}
                        // };

                        read_files(cache, name);
                    }
                }
            }

            println!("{} Assets Loaded Into Cache", "[CACHE]:".purple());
        }
    };
}

pub fn read_files(cache: &mut Cache, path: String) {
    let fullpath = format!("{}{}", STORE, path);

    match fs::read_dir(fullpath) {
        Err(error) => {
            println!("{:#?}", error)
        }
        Ok(dirs) => {
            for i in dirs.into_iter() {
                match i {
                    Err(error) => {
                        println!("{:#?}", error)
                    }
                    Ok(file) => {
                        let name = file.file_name().to_str().unwrap().to_string();
                        cache.cache(path.clone(), name, CacheOptions { minify: true });
                    }
                }
            }
        }
    };
}
