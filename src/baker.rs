// TODO: Check for file: .baker
// TODO: Read file
// TODO: Run rules

use std::{io::{self, Read}, fs::{OpenOptions, self}, path::Path};

pub struct BakFile;

// NOTE: Everything is unsafe for now (full unwrap mode lol)

impl BakFile {
    pub fn new() -> () {
        if Path::new(".baker").exists() { return (); }
        fs::write(".baker", "hello, world!").unwrap();
    }

    pub fn content() -> io::Result<String> {
        let oo = OpenOptions::new().read(true).write(false).open(".baker");

        let mut buffer: String = Default::default();
        oo.unwrap().read_to_string(&mut buffer).unwrap();

        return Ok(buffer);
    }
}
