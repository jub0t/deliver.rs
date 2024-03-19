use std::sync::atomic::AtomicUsize;

use chrono::{Duration, TimeDelta};

pub static MAX_CACHE_TIME: u64 = 60 * 60 * 2; // Cache each item for 2 hours
pub static COUNTER: AtomicUsize = AtomicUsize::new(1);
pub static TOKEN_EXPIRE_TIME: Option<TimeDelta> = Duration::try_hours(24 * 30 * 12);
pub static STORE: &str = "./test/";

pub struct Config {}

impl Config {
    pub fn new() -> Self {
        return Self {};
    }
}
