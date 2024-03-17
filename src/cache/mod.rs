pub mod format;
pub mod load;

use std::{collections::HashMap, fs, sync::atomic::AtomicUsize, time::SystemTime};

use serde::{Deserialize, Serialize};

use crate::{
    cache::format::string_to_format,
    hasher::{HashSize, Hasher},
    minify::Minifier,
    STORE,
};

#[derive(Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FileFormat {
    CSS,
    JS,
    HTML,
    IMAGE(ImageFormat),
    None,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct File {
    pub hash: HashSize,
    pub document: String,
    pub name: String,
    pub format: FileFormat,
    pub created: SystemTime,
    pub contents: Vec<u8>,
    pub key: FileKey,
}

pub struct CacheOptions {
    pub minify: bool,
}

pub static COUNTER: AtomicUsize = AtomicUsize::new(1);

pub struct Cache {
    hasher: Hasher,
    files: FileMap,
    minifer: Minifier,
}

pub type FileKey = HashSize;
pub type FileMap = HashMap<FileKey, File>;

impl Cache {
    pub fn new() -> Self {
        let hasher = Hasher::new();
        return Self {
            hasher,
            minifer: Minifier::new(),
            files: HashMap::new(),
        };
    }

    pub fn get(&self, document: String, filename: String) -> Option<&File> {
        let key = format!("{}:{}", document, filename);
        let hash = &self.hasher.hash(key.as_bytes().to_vec()).unwrap();

        let file = self.files.get(hash);
        return file;
    }

    pub fn delete(&mut self, hash: FileKey) -> Option<File> {
        return self.files.remove(&hash);
    }

    pub fn all(&self) -> &FileMap {
        return &self.files;
    }

    pub fn as_vec(&self) -> Vec<File> {
        self.files.values().cloned().collect()
    }

    pub fn cache(&mut self, document: String, filename: String, options: CacheOptions) -> bool {
        let full_path = format!("{}{}/{}", STORE, document, filename);
        let mut extension: FileFormat = FileFormat::None;

        match filename.split(".").last() {
            None => {
                println!("File Ignore, No Extension Found: [{}]", filename);
            }
            Some(ext) => {
                extension = string_to_format(&ext.to_string()).unwrap();
            }
        }

        match fs::read(&full_path) {
            Err(_) => {
                return false;
            }
            Ok(data) => {
                let mut contents = data.clone();
                let hash = match self.hasher.hash(contents.clone()) {
                    Ok(hash) => hash,
                    Err(error) => {
                        println!("ERROR Hashing {}: {:#?}", filename, error);
                        0
                    }
                };

                if options.minify {
                    match extension {
                        FileFormat::JS => {
                            // match file_type {}
                            // but for now
                            let minified = self.minifer.javascript(&data);
                            contents = minified;
                        }

                        FileFormat::CSS => {
                            let minified = self.minifer.css(&data);
                            contents = minified;
                        }

                        FileFormat::HTML => {
                            let minified = self.minifer.html(&data);
                            contents = minified;
                        }

                        _ => {}
                    }
                }

                let key = self
                    .hasher
                    .hash(
                        (format!("{}:{}", document, filename))
                            .as_bytes()
                            .to_vec()
                            .clone(),
                    )
                    .unwrap();

                let file = File {
                    hash: hash.clone(),
                    document: document.clone(),
                    format: extension, // Just for now,
                    name: filename.clone(),
                    contents: contents.clone(),
                    created: SystemTime::now(),
                    key,
                };

                self.files.insert(key, file.clone());

                println!(
                    "   - File {:#?} Size:{} Hash:{}",
                    filename,
                    file.contents.len(),
                    hash.clone(),
                );

                return true;
            }
        }
    }

    pub fn size(&self) -> usize {
        let mut size: usize = 0;

        let files = self.as_vec();
        for file in files {
            size += file.contents.len()
        }

        return size;
    }
}
