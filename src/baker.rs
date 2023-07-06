// TODO: Check for file: .baker
// TODO: Read file
// TODO: Run rules

use colored::Colorize;

use super::logger::Logger;
use std::{io::{self, Read}, fs::{OpenOptions, self}, path::Path};

pub struct BakFile;

impl BakFile {
    pub fn new() -> io::Result<()> {
        Logger::info(&format!("Init {} file", ".baker".green()));

        if Path::new(".baker").exists() {
            Logger::log(&format!("File {} found", ".baker".green()));
            return Ok(());
        }

        fs::write(".baker", "$set hello *\n\techo 'Hello, world!'\n$run")?;

        Logger::log(&format!("File {} created", ".baker".green()));
        return Ok(());
    }

    pub fn content() -> io::Result<String> {
        Logger::info(&format!("Reading {} contents", ".baker".green()));

        let mut buffer: String = String::new();

        Logger::log(&format!("Reading file {}", ".baker".green()));
        Logger::log(&format!("Options: read ({}); write ({})", "true".green(), "false".red()));
        OpenOptions::new().read(true).write(false).open(".baker")?.read_to_string(&mut buffer)?;

        Logger::log(&format!("File {} read", ".baker".green()));
        return Ok(buffer);
    }
}
