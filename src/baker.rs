// TODO: Run rules

use std::{io::{self, Read}, fs::{self, OpenOptions}, path::Path};
use colored::Colorize;

use crate::{logger::Logger, config::Config};

#[derive(Clone, Copy)]
pub struct BakFile<'a> {
    filename: &'a str,
}

impl<'a> BakFile<'a> {
    pub fn new(filename: &'a str) -> io::Result<Self> {
        Logger::info(&format!("Init {} file", filename.green()));

        if !Path::new(&filename).exists() {
            Logger::warn(&format!("File {} not found", filename.green()));
            if !Config::get_config().gen_files {
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, "gen_files is disabled"));
            }

            fs::write(&filename, "$define hello *\n\techo 'Hello, world!'\n$run")?;
        }

        Logger::warn(&format!("File {} found", filename.green()));
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
