use chrono::Duration;
use colored::*;
use std::{fs, sync::atomic::AtomicUsize};
use toml::Value;

pub static MAX_CACHE_TIME: u64 = 60 * 60 * 24;
pub static WATCHDOG_INTERVAL: u64 = 30;
pub static COUNTER: AtomicUsize = AtomicUsize::new(1);
pub static TOKEN_EXPIRE_TIME: Option<Duration> = Some(Duration::hours(24 * 30 * 12));
pub static STORE: &str = "./test/";

pub struct Config {
    pub verbose: bool,
    pub store: String,
}

impl Default for Config {
    fn default() -> Self {
        let config_data = fs::read("./config.toml").unwrap_or_else(|e| {
            panic!(
                "{} Could not read or load './config.toml': {}",
                "[CONFIG]:".red(),
                e
            );
        });

        let config_str = String::from_utf8_lossy(&config_data);
        let doc = config_str.parse::<Value>().unwrap_or_else(|e| {
            panic!("{} Error parsing configuration: {}", "[CONFIG]:".red(), e);
        });

        let verbose = doc["verbose"].as_bool().unwrap_or(false);
        let store = doc["cache_store"].as_str().unwrap_or_default().to_string();

        Self { verbose, store }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
