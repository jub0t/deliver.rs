pub mod format;
pub mod load;
pub mod types;

use std::{collections::HashMap, fs, time::SystemTime};

use crate::{
    cache::format::string_to_format,
    config::STORE,
    hasher::{HashSize, Hasher},
    minify::Minifier,
};
use colored::*;
use serde::{Deserialize, Serialize};

use self::{
    format::{format_to_mime, format_to_string},
    types::FileFormat,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct File {
    pub hash: HashSize,
    pub key: FileKey,
    pub document: String,
    pub name: String,
    pub format: FileFormat,
    pub created: SystemTime,
    pub contents: Vec<u8>,
}

impl File {
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_contents(&mut self, contents: Vec<u8>) {
        self.contents = contents
    }

    pub fn get_extension(&self) -> String {
        format_to_string(self.format)
    }

    pub fn get_mime(&self) -> String {
        format_to_mime(self.format)
    }
}

pub struct CacheOptions {
    pub minify: bool,
}

pub struct Cache {
    hasher: Hasher,
    files: FileMap,
    minifer: Minifier,
}

pub type FileKey = String;
pub type FileMap = HashMap<FileKey, File>;

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Self {
        let hasher = Hasher::new();
        Self {
            hasher,
            minifer: Minifier::new(),
            files: HashMap::new(),
        }
    }

    pub fn get(&self, document: String, filename: String) -> Option<&File> {
        let key = format!("{}:{}", document, filename);
        // let hash = &self.hasher.hash(key.as_bytes().to_vec()).unwrap();

        let file = self.files.get(&key);
        file
    }

    pub fn delete(&mut self, hash: FileKey) -> Option<File> {
        self.files.remove(&hash)
    }

    pub fn all(&self) -> &FileMap {
        &self.files
    }

    pub fn as_vec(&self) -> Vec<File> {
        self.files.values().cloned().collect()
    }

    pub fn as_contentless_vec(&self) -> Vec<File> {
        let clean = self
            .files
            .values()
            .cloned()
            .map(|f| {
                let mut n = f;
                n.contents = Vec::new();
                n
            })
            .collect();

        clean
    }

    pub fn cache(&mut self, document: String, filename: String, options: CacheOptions) -> bool {
        let full_path = format!("{}{}/{}", STORE, document, filename);
        let mut extension: FileFormat = FileFormat::None;

        match filename.split('.').last() {
            None => {
                println!("File Ignore, No Extension Found: [{}]", filename);
            }
            Some(ext) => {
                extension = string_to_format(ext).unwrap();
            }
        }

        match fs::read(full_path) {
            Err(_) => false,
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

                // let key = self
                //     .hasher
                //     .hash(
                //         (format!("{}:{}", document, filename))
                //             .as_bytes()
                //             .to_vec()
                //             .clone(),
                //     )
                //     .unwrap();
                let key = format!("{}:{}", document, filename);
                let file = File {
                    hash,
                    document: document.clone(),
                    format: extension, // Just for now,
                    name: filename.clone(),
                    contents: contents.clone(),
                    created: SystemTime::now(),
                    key: key.clone(),
                };

                self.files.insert(key, file.clone());

                println!(
                    "   {} [Name: {}] [Size: {}] [Hash: {}]",
                    "[FILE]:".green(),
                    filename.bright_black(),
                    file.contents.len().to_string().bright_black(),
                    hash.clone().to_string().bright_black(),
                );

                true
            }
        }
    }

    pub fn size(&self) -> usize {
        let mut size: usize = 0;

        let files = self.as_vec();
        for file in files {
            size += file.contents.len()
        }

        size
    }

    pub fn item_count(&self) -> usize {
        self.files.len()
    }
}
