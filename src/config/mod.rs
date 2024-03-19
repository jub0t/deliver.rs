use std::sync::atomic::AtomicUsize;

pub static MAX_CACHE_TIME: u64 = 60 * 60 * 2; // Cache each item for 2 hours
pub static COUNTER: AtomicUsize = AtomicUsize::new(1);
