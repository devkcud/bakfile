use std::sync::{Mutex, MutexGuard};

use crate::logger::LogLevel;

lazy_static::lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
}

#[derive(Debug)]
pub struct Config {
    pub gen_files: bool,
    pub log_level: LogLevel,
    pub colors: bool,
}

impl Config {
    pub fn setup(config: Self) {
        *CONFIG.lock().unwrap() = config;
    }

    pub fn default() -> Self {
        return Self {
            gen_files: true,
            log_level: LogLevel::Fault,
            colors:    true,
        };
    }

    pub fn get_config<'a>() -> MutexGuard<'a, Config> {
        return CONFIG.lock().unwrap();
    }
}
