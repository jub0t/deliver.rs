use std::sync::{Arc, Mutex};

use crate::cache::Cache;
static MAX_CACHE_TIME: u64 = 60;

pub async fn start(watch_cache: Arc<Mutex<Cache>>) {
    tokio::task::spawn(async move {
        loop {
            let files;
            {
                let cache = watch_cache.lock().unwrap();
                files = cache.as_vec().clone(); // Clone the vector to release the lock earlier
            } // Lock is released here

            let now = std::time::SystemTime::now();

            for file in files {
                let expire = file.created + std::time::Duration::from_secs(MAX_CACHE_TIME);
                if now >= expire {
                    let mut cache = watch_cache.lock().unwrap(); // Re-acquire the lock to delete the expired entry
                    println!("Deleted Expired File: {:?}", file.name);
                    cache.delete(file.hash);
                }
            }

            println!("Watchdog Cache Cycle Complete");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await; // Use tokio::time for async sleep
        }
    })
    .await
    .unwrap(); // Wait for the task to complete
}
