use std::{sync::RwLock, path::PathBuf, fs, io};

use crate::logger::LogLevel;

lazy_static::lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

fn parse_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().trim() {
        "true"  => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_log_level(s: &str) -> Option<LogLevel> {
    match s.to_lowercase().trim() {
        "none"  => Some(LogLevel::None),
        "info"  => Some(LogLevel::Info),
        "fault" => Some(LogLevel::Fault),
        "full"  => Some(LogLevel::Full),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub gen_files: bool,
    pub log_level: LogLevel,
    pub colors:    bool,
}

impl Config {
    pub fn setup() -> io::Result<()> {
        let config_dir = match dirs::config_dir() {
            Some(o) => o,
            None => {
                return Ok(*CONFIG.write().unwrap() = Self::default());
            },
        };

        let config_file: PathBuf = PathBuf::from(format!("{}/bakfile/config", config_dir.to_str().unwrap()));

        if !config_file.exists() || !config_file.is_file() {
            return Ok(*CONFIG.write().unwrap() = Self::default());
        }

        let mut config = *CONFIG.write().unwrap();

        for line in fs::read_to_string(config_file)?.lines().filter(|x| !x.is_empty() && !x.starts_with('$')) {
            let line = line.split_whitespace().collect::<Vec<&str>>();
            if line.len() != 2 { continue; }

            let key:   &str = line[0];
            let value: &str = line[1];

            match key {
                "gen_files" => config.gen_files = parse_bool(value).unwrap_or(true),
                "log_level" => config.log_level = parse_log_level(value).unwrap_or(LogLevel::Fault),
                "colors"    => config.colors    = parse_bool(value).unwrap_or(true),
                _ => (),
            }
        }

        return Ok(());
    }

    pub fn default() -> Self {
        return Self {
            gen_files: false,
            log_level: LogLevel::Fault,
            colors:    true,
        };
    }

    pub fn get_config() -> Config {
        let config = *CONFIG.read().unwrap();
        return config;
    }
}
