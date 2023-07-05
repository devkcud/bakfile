use chrono::prelude::*;
use colored::Colorize;

fn get_current_time() -> String {
    return format!("{}", Local::now().format("%H:%M:%S").to_string().dimmed());
}

pub struct Logger;

impl Logger {
    pub fn info(what: &str) {
        println!("{}  {} {}", get_current_time(), "INFO".bright_green().bold(), what);
    }

    pub fn log(what: &str) {
        println!("{}   {} {}", get_current_time(), "LOG".bright_blue().bold(), what);
    }

    pub fn warn(what: &str) {
        println!("{}  {} {}", get_current_time(), "WARN".bright_yellow().bold(), what);
    }

    pub fn error(what: &str, exit: i32) {
        println!("{} {} {}", get_current_time(), "ERROR".bright_red().bold(), what);

        if exit != 0 {
            Logger::info("Exiting...");
            std::process::exit(exit);
        }
    }

    pub fn print(what: &str) {
        println!("{what}");
    }
}
