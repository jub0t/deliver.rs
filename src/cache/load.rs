use std::fs::{self};

use crate::{cache::CacheOptions, STORE};

use super::Cache;

pub fn load_into(cache: &mut Cache) {
    match fs::read_dir(STORE) {
        Err(error) => {
            println!("{:#?}", error)
        }
        Ok(dirs) => {
            for i in dirs.into_iter() {
                match i {
                    Err(error) => {
                        println!("{:#?}", error)
                    }
                    Ok(document) => {
                        let name = document.file_name().to_str().unwrap().to_string();
                        let path = format!("{}{}", STORE, name);
                        println!("- Document: {:#?}", path);

                        // let new_path = format!("{}{}", STORE, nanoid!(10));
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