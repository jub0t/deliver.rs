use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    cache::{Cache, FileKey},
    config::MAX_CACHE_TIME,
};

pub fn start(cache: Arc<Mutex<Cache>>) {
    loop {
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
            println!("Expired file removed: {:?}", key);
        }

        drop(cache_lock);

        thread::sleep(Duration::from_secs(2 * 60)); // Sleep for 60 seconds (adjust as needed)
    }
}
