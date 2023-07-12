use std::{io::Write, sync::Mutex};

use chrono::prelude::*;
use colored::Colorize;

fn get_current_time() -> String {
    return format!("{}", Local::now().format("%H:%M:%S").to_string().dimmed());
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogLevel {
    /// Disable logs
    None,
    /// Log `info` and `logs`
    Info,
    /// Log `warns` and `errors`
    Fault,
    /// Log `everything`
    Full,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::None  => "None".to_string(),
            LogLevel::Info  => "Info".to_string(),
            LogLevel::Fault => "Fault".to_string(),
            LogLevel::Full  => "Full".to_string(),
        }
    }
}

lazy_static::lazy_static! {
    static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Full);
}

pub struct Logger;

impl Logger {
    pub fn set_level(level: LogLevel) {
        let mut log_level = LOG_LEVEL.lock().unwrap();
        *log_level = level;
    }

    pub fn info(what: &str) {
        let log_level = LOG_LEVEL.lock().unwrap();
        if *log_level != LogLevel::Full && *log_level != LogLevel::Info { return; }

        println!("{}  {} {}", get_current_time(), "INFO".bright_green().bold(), what);
    }

    pub fn log(what: &str) {
        let log_level = LOG_LEVEL.lock().unwrap();
        if *log_level != LogLevel::Full && *log_level != LogLevel::Info { return; }

        println!("{}   {} {}", get_current_time(), "LOG".bright_blue().bold(), what);
    }

    pub fn warn(what: &str) {
        let log_level = LOG_LEVEL.lock().unwrap();
        if *log_level != LogLevel::Full && *log_level != LogLevel::Fault { return; }

        println!("{}  {} {}", get_current_time(), "WARN".bright_yellow().bold(), what);
    }

    pub fn error(what: &str) {
        let log_level = LOG_LEVEL.lock().unwrap();
        if *log_level != LogLevel::Full && *log_level != LogLevel::Fault { return; }

        println!("{} {} {}", get_current_time(), "ERROR".bright_red().bold(), what);
    }

    pub fn exit(what: &str) -> ! {
        let log_level = LOG_LEVEL.lock().unwrap();
        if *log_level != LogLevel::Full && *log_level != LogLevel::Fault { std::process::exit(1); }

        println!("{}  {} {}", get_current_time(), "EXIT".red().bold(), what);
        std::process::exit(1);
    }

    pub fn print(what: &str) {
        print!("{what}");
        match std::io::stdout().flush() { _ => () };
    }
}
