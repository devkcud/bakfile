use std::{io::{self, Read}, fs::{self, OpenOptions}, path::Path};
use colored::Colorize;

use crate::{logger::Logger, config::Config};

#[derive(Clone, Copy)]
pub struct BakFile<'a> {
    filename: &'a str,
}

impl<'a> BakFile<'a> {
    pub fn new(filename: &'a str) -> io::Result<Self> {
        if !Path::new(&filename).exists() {
            Logger::error(&format!("File {} not found", filename.green()));
            if !Config::get_config().gen_files {
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, "gen_files is disabled"));
            }

            fs::write(&filename, "$define hello *\n\techo 'Hello, world!'\n$run")?;
            Logger::log(&format!("File {} created", filename.green()));
        }

        Logger::log(&format!("File {} found", filename.green()));
        return Ok(Self { filename });
    }

    pub fn read(&self) -> io::Result<String> {
        let mut buffer: String = String::new();

        Logger::log(&format!("Reading file {} (readonly)", self.filename.green()));
        OpenOptions::new().read(true).write(false).open(self.filename)?.read_to_string(&mut buffer)?;

        return Ok(buffer);
    }
}
