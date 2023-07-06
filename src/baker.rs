// TODO: Run rules

use std::{io::{self, Read}, fs::{self, OpenOptions}, path::Path};
use colored::Colorize;

use crate::logger::Logger;

#[derive(Clone, Copy)]
pub struct BakFile<'a> {
    filename: &'a str,
}

impl<'a> BakFile<'a> {
    pub fn new(filename: &'a str) -> io::Result<Self> {
        Logger::info(&format!("Init {} file", filename.green()));

        if !Path::new(&filename).exists() {
            Logger::log(&format!("File {} not found; creating one", filename.green()));
            fs::write(&filename, "$set hello *\n\techo 'Hello, world!'\n$run")?;
        }

        Logger::log(&format!("File {} found", filename.green()));
        return Ok(Self { filename });
    }

    pub fn read(&self) -> io::Result<String> {
        Logger::info(&format!("Reading {} content", self.filename.green()));

        let mut buffer: String = String::new();

        Logger::log(&format!("Reading file {} (readonly)", self.filename.green()));
        OpenOptions::new().read(true).write(false).open(self.filename)?.read_to_string(&mut buffer)?;

        Logger::log(&format!("File {} read", self.filename.green()));
        return Ok(buffer);
    }
}
