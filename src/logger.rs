use chrono::prelude::*;
use colored::Colorize;

fn get_current_time() -> String {
    return format!("{}{}{}", "[".black(), Local::now().format("%H:%M:%S").to_string(), "]".black());
}

pub struct Logger;

impl Logger {
    pub fn info(what: &str) {
        println!("{}  {} {} {}", get_current_time(), "INFO".bright_green().bold(), ":".black(), what);
    }

    pub fn log(what: &str) {
        println!("{}   {} {} {}", get_current_time(), "LOG".bright_blue().bold(), ":".black(), what);
    }

    pub fn warn(what: &str) {
        println!("{}  {} {} {}", get_current_time(), "WARN".bright_yellow().bold(), ":".black(), what);
    }

    pub fn error(what: &str, exit: i32) {
        println!("{} {} {} {}", get_current_time(), "ERROR".bright_red().bold(), ":".black(), what);

        if exit != 0 {
            Logger::error("Exiting...", 0);
            std::process::exit(exit);
        }
    }

    pub fn print(what: &str) {
        println!("{what}");
    }
}
