use std::{sync::RwLock, path::PathBuf, fs, io};

use dirs::config_dir;

use crate::logger::LogLevel;

lazy_static::lazy_static! {
    static ref CONFIG:      RwLock<Config> = RwLock::new(Default::default());
}

fn parse_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().trim() {
        "true"  => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_logl(s: &str) -> Option<LogLevel> {
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
    pub log:       LogLevel,
    pub colors:    bool,
}

impl Default for Config {
    fn default() -> Self {
        return Self {
            gen_files: false,
            log:       LogLevel::Fault,
            colors:    true,
        };
    }
}

impl Config {
    pub fn setup(using_local: bool) -> io::Result<()> {
        if let None = dirs::config_dir() {
            return Ok(*CONFIG.write().unwrap() = Default::default());
        }

        let config_file: PathBuf = PathBuf::from(if using_local {
            format!("{}/bakfile/config", config_dir().unwrap().to_str().unwrap())
        } else {
            String::from("./.baker.config")
        });

        if !config_file.exists() || !config_file.is_file() {
            return Ok(*CONFIG.write().unwrap() = Default::default());
        }

        for line in fs::read_to_string(config_file)?.lines().filter(|x| !x.is_empty() && !x.starts_with(';')) {
            let options: Vec<&str> = line.split_whitespace().collect();
            if options.len() != 2 { continue; }

            let key:   &str = options.get(0).unwrap();
            let value: &str = options.get(1).unwrap();

            match key {
                "gen_files" => CONFIG.write().unwrap().gen_files = parse_bool(value).unwrap_or(true),
                "log"       => CONFIG.write().unwrap().log       = parse_logl(value).unwrap_or(Default::default()),
                "colors"    => CONFIG.write().unwrap().colors    = parse_bool(value).unwrap_or(true),
                _ => (),
            }
        }

        return Ok(());
    }

    pub fn get_config() -> Config {
        return *CONFIG.read().unwrap();
    }
}
