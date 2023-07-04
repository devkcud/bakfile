#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

use logger::Logger;

fn main() {
    baker::BakFile::new();
    let bak = baker::BakFile::content().unwrap();

    Logger::unformatted(&bak);
    Logger::log("Hello, world! This is a log");
    Logger::error("Hello, world! This is a log", 1);
}
