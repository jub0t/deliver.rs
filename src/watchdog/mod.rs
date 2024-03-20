use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use colored::Colorize;

use crate::{
    cache::{Cache, FileKey},
    config::{MAX_CACHE_TIME, WATCHDOG_INTERVAL},
};

pub fn start(cache: Arc<Mutex<Cache>>) {
    loop {
        thread::sleep(Duration::from_secs(WATCHDOG_INTERVAL)); // Sleep for 60 seconds (adjust as needed)
        let mut cache_lock = cache.lock().unwrap();

        let expired_files: Vec<FileKey> = cache_lock
            .all()
            .iter()
            .filter_map(|(key, file)| {
                if let Ok(elapsed) = file.created.elapsed() {
                    if elapsed.as_secs() > MAX_CACHE_TIME {
                        return Some(*key);
                    }
                }
                None
            })
            .collect();

        for key in expired_files {
            cache_lock.delete(key);
            println!(
                "{} File Expired, Removed: [Hash: {:#?}]",
                "[CACHE]:".yellow(),
                key.to_string().bright_black()
            );
        }

        drop(cache_lock);
    }
}
