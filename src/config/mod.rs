use std::sync::atomic::AtomicUsize;

use chrono::{Duration, TimeDelta};

pub static MAX_CACHE_TIME: u64 = 60 * 60;
pub static WATCHDOG_INTERVAL: u64 = 10;
pub static COUNTER: AtomicUsize = AtomicUsize::new(1);
pub static TOKEN_EXPIRE_TIME: Option<TimeDelta> = Duration::try_hours(24 * 30 * 12);
pub static STORE: &str = "./test/";

pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
}
